use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::net::SocketAddr;
use std::process::exit;
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerInfo, ServerMessage, ServerPacket};
use aeonetica_engine::{ENGINE_VERSION, MAX_CLIENT_TIMEOUT};
use aeonetica_engine::{log, Id};
use aeonetica_engine::networking::{MAX_RAW_DATA_SIZE, NetResult};
use aeonetica_engine::sha2;
use aeonetica_engine::sha2::Digest;
use crate::ecs::Engine;
use crate::ecs::events::ConnectionListener;
use crate::networking::ClientHandle;
use crate::server_runtime::ServerRuntime;

impl Engine {
    pub(crate) fn handle_queued(&mut self) -> Result<(), AError> {
        let packets = self.runtime.ns.borrow_mut().queued_packets();
        packets.into_iter().map(|(addr, packet)| self.handle_packet(&addr, &packet))
        .reduce(|acc, r| {
            acc?;
            r?;
            Ok(())
        }).unwrap_or(Ok(()))
    }

    pub(crate) fn timeout_inactive(&mut self) {
        let mut_self_ref_ptr = self as *mut Self;
        let clients = self.runtime.ns.borrow().clients.keys().cloned().collect::<Vec<_>>();
        for id in clients {
            let mut_self_ref = unsafe { &mut *mut_self_ref_ptr };
            if self.runtime.ns.borrow().clients.get(&id).map(|client| {
                if client.last_seen.elapsed().as_millis() < MAX_CLIENT_TIMEOUT {
                    false
                } else {
                    mut_self_ref.kick_client(&id, "TIMEOUT");
                    let _ = mut_self_ref.runtime.ns.borrow().send(&id, &ServerPacket {
                        conv_id: Id::new(),
                        message: ServerMessage::Unregister("TIMEOUT".to_string()),
                    });
                    log!("timed out client ip {}", client.client_addr);
                    true
                }
            }).unwrap_or(true) {
                mut_self_ref.runtime.ns.borrow_mut().clients.remove(&id);
            }
        }
    }

    pub(crate) fn handle_packet(&mut self, addr: &SocketAddr, packet: &ClientPacket) -> Result<(), AError>{
        let mut_ref_ptr = &mut self.runtime as *mut ServerRuntime;
        if let Some(client) = self.runtime.ns.borrow_mut().clients.get_mut(&packet.client_id) {
            client.last_seen = std::time::Instant::now();
            if let Some(handler) = client.awaiting_replies.remove(&packet.conv_id) {
                let mut_ref = unsafe { &mut *mut_ref_ptr };
                handler(mut_ref, packet);
                return Ok(())
            }
        }
        match &packet.message {
            ClientMessage::Register(client_info) => {
                if client_info.client_version == ENGINE_VERSION {
                    self.runtime.ns.borrow_mut().clients.insert(packet.client_id, ClientHandle {
                        last_seen: std::time::Instant::now(),
                        client_addr: *addr,
                        awaiting_replies: Default::default(),
                    });
                    self.runtime.ns.borrow().send(&packet.client_id, &ServerPacket{
                        conv_id: packet.conv_id,
                        message: ServerMessage::RegisterResponse(NetResult::Ok(ServerInfo {
                            server_version: ENGINE_VERSION.to_string(),
                            mod_profile: self.runtime.mod_profile.profile.clone(),
                            mod_version: self.runtime.mod_profile.version.clone(),
                            mods: self.runtime.mod_profile.modstack.iter().map(|(name_path, flags)| {
                                let (name, path) = name_path.split_once(':').unwrap();
                                let client_path = format!("runtime/{path}/{name}_client.zip");
                                let size = std::fs::metadata(&client_path).unwrap().len();
                                let mut file = File::open(&client_path).unwrap();
                                let mut hasher = sha2::Sha256::default();
                                std::io::copy(&mut file, &mut hasher).unwrap();
                                let digest = hasher.finalize();
                                (name_path.clone(), flags.clone(), format!("{digest:X}"), size)
                            }).collect(),
                        }))
                    })?;
                    log!("registered client ip {addr} with id {}", packet.client_id);
                } else {
                    self.runtime.ns.borrow().send_raw(*addr, &ServerPacket{
                        conv_id: packet.conv_id,
                        message: ServerMessage::RegisterResponse(NetResult::Err(
                            format!("client and server engine versions do not match: {} != {}", client_info.client_version, ENGINE_VERSION)
                        ))
                    })?;
                }
            }
            ClientMessage::Ping(msg) => self.runtime.ns.borrow().send(&packet.client_id, &ServerPacket{
                conv_id: packet.conv_id,
                message: ServerMessage::Pong(msg.clone()),
            })?,
            ClientMessage::DownloadMod(name_path, offset) => {
                let (name, path) = name_path.split_once(':').unwrap();
                let client_path = format!("runtime/{path}/{name}_client.zip");
                let mut file = File::open(client_path)?;
                let mut buffer = [0;MAX_RAW_DATA_SIZE];
                file.seek(SeekFrom::Start(*offset))?;
                let len = file.read(&mut buffer[..])?;
                self.runtime.ns.borrow().send(&packet.client_id, &ServerPacket{
                    conv_id: packet.conv_id,
                    message: ServerMessage::RawData(buffer[..len].to_vec())
                })?;
            },
            ClientMessage::Login => {
                self.clients.insert(packet.client_id);
                self.for_each_module_of_type::<ConnectionListener, _>(|engine, id,  m| (m.on_join)(id, &packet.client_id, engine))
            }
            ClientMessage::Logout => {
                log!("client logged out: {}", packet.client_id);
                self.clients.remove(&packet.client_id);
                self.for_each_module_of_type::<ConnectionListener, _>(|engine, id,  m| (m.on_leave)(id, &packet.client_id, engine))
            }
            _ => ()
        }
        Ok(())
    }

    pub(crate) fn request_response<F: Fn(&mut ServerRuntime, &ClientPacket) + 'static>(&mut self, client_id: &Id, packet: &ServerPacket, handler: F) -> Result<(), AError> {
        match self.runtime.ns.borrow_mut().clients.get_mut(client_id) {
            Some(client) => {
                client.awaiting_replies.insert(packet.conv_id, Box::new(handler));
                self.runtime.ns.borrow().send(client_id, packet)?;
                Ok(())
            }
            None => {
                Err(AError::new(AET::NetworkError("invalid client".to_string())))
            }
        }
    }
}