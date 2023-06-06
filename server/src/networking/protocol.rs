use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::net::SocketAddr;

use aeonetica_engine::error::ErrorResult;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerInfo, ServerMessage, ServerPacket};
use aeonetica_engine::{ENGINE_VERSION, MAX_CLIENT_TIMEOUT};
use aeonetica_engine::{log, Id};
use aeonetica_engine::networking::{MOD_DOWNLOAD_CHUNK_SIZE, NetResult, SendMode};
use aeonetica_engine::sha2;
use aeonetica_engine::sha2::Digest;
use crate::ecs::Engine;
use crate::ecs::events::ConnectionListener;
use crate::ecs::messaging::Messenger;
use crate::networking::ClientHandle;

impl Engine {
    pub(crate) fn handle_queued(&mut self) -> ErrorResult<()> {
        let packets = self.runtime.ns.borrow_mut().queued_packets();
        let mut i = 0;
        let r = packets.into_iter().map(|(addr, packet)| self.handle_packet(&addr, &packet))
        .reduce(|acc, r| {
            acc?;
            r?;
            i += 1;
            Ok(())
        }).unwrap_or(Ok(()));
        r
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
                    }, SendMode::Safe);
                    log!("timed out client ip {}", client.client_addr);
                    true
                }
            }).unwrap_or(true) {
                mut_self_ref.runtime.ns.borrow_mut().clients.remove(&id);
            }
        }
    }

    pub(crate) fn handle_packet(&mut self, addr: &SocketAddr, packet: &ClientPacket) -> ErrorResult<()> {
        if let Some(client) = self.runtime.ns.borrow_mut().clients.get_mut(&packet.client_id) {
            client.last_seen = std::time::Instant::now();
        }
        match &packet.message {
            ClientMessage::Register(client_info) => {
                if client_info.client_version == ENGINE_VERSION {
                    self.runtime.ns.borrow_mut().clients.insert(packet.client_id, ClientHandle {
                        last_seen: std::time::Instant::now(),
                        client_addr: *addr
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
                    }, SendMode::Safe)?;
                    log!("registered client ip {addr} with id {}", packet.client_id);
                } else {
                    self.runtime.ns.borrow().send_raw(*addr, &ServerPacket{
                        conv_id: packet.conv_id,
                        message: ServerMessage::RegisterResponse(NetResult::Err(
                            format!("client and server engine versions do not match: {} != {}", client_info.client_version, ENGINE_VERSION)
                        ))
                    }, SendMode::Safe)?;
                }
            }
            ClientMessage::Ping(msg) => self.runtime.ns.borrow().send(&packet.client_id, &ServerPacket{
                conv_id: packet.conv_id,
                message: ServerMessage::Pong(msg.clone()),
            }, SendMode::Safe)?,
            ClientMessage::DownloadMod(name_path, offset) => {
                let (name, path) = name_path.split_once(':').unwrap();
                let client_path = format!("runtime/{path}/{name}_client.zip");
                let mut file = File::open(client_path)?;
                let mut buffer = [0;MOD_DOWNLOAD_CHUNK_SIZE];
                file.seek(SeekFrom::Start(*offset))?;
                let len = file.read(&mut buffer[..])?;
                self.runtime.ns.borrow().send(&packet.client_id, &ServerPacket{
                    conv_id: packet.conv_id,
                    message: ServerMessage::RawData(buffer[..len].to_vec())
                }, SendMode::Safe)?;
            },
            ClientMessage::Login => {
                if !self.clients.contains(&packet.client_id) {
                    log!("client logged in: {}", packet.client_id);
                    self.clients.insert(packet.client_id);
                    self.for_each_module_of_type::<ConnectionListener, _>(|engine, id, m| (m.on_join)(id, engine, &packet.client_id))
                }
            }
            ClientMessage::Logout => {
                if self.clients.contains(&packet.client_id) {
                    log!("client logged out: {}", packet.client_id);
                    self.for_each_module_of_type::<ConnectionListener, _>(|engine, id, m| (m.on_leave)(id, engine, &packet.client_id));
                    self.clients.remove(&packet.client_id);
                    let mut ns = self.runtime.ns.borrow_mut();
                    ns.clients.remove(&packet.client_id);
                    ns.tcp.lock().unwrap().remove(addr);
                }
            }
            ClientMessage::ModMessage(eid, rid, data) => {
                let mut_engine_ref = unsafe { &mut *(self as *mut Self) };
                if let Some(m) = self.get_module_of::<Messenger>(eid).ref_option() {
                    if let Some(f) = m.receiver_functions.get(rid) {
                        f(eid, mut_engine_ref, &packet.client_id, data)
                    }
                }
            }
            _ => ()
        }
        Ok(())
    }
}