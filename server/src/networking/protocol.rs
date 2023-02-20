use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::net::SocketAddr;
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerInfo, ServerMessage, ServerPacket};
use aeonetica_engine::{ENGINE_VERSION};
use aeonetica_engine::{log, Id};
use aeonetica_engine::networking::{MAX_RAW_DATA_SIZE, NetResult};
use aeonetica_engine::sha2;
use aeonetica_engine::sha2::Digest;
use crate::networking::ClientHandle;
use crate::server_runtime::ServerRuntime;

impl ServerRuntime {
    pub(crate) fn handle_packet(&mut self, addr: &SocketAddr, packet: &ClientPacket) -> Result<(), AError>{
        if let Some(client) = self.ns.clients.get_mut(&packet.client_id) {
            client.last_seen = std::time::Instant::now();
            if let Some(handler) = client.awaiting_replies.remove(&packet.conv_id) {
                handler(self, packet);
                return Ok(())
            }
        }
        match &packet.message {
            ClientMessage::Register(client_info) => {
                if client_info.client_version == ENGINE_VERSION {
                    self.ns.clients.insert(packet.client_id.clone(), ClientHandle {
                        last_seen: std::time::Instant::now(),
                        client_addr: addr.clone(),
                        socket: {
                            let sock = self.ns.socket.try_clone()?;
                            sock.connect(addr)?;
                            sock
                        },
                        awaiting_replies: Default::default(),
                    });
                    self.ns.send(&packet.client_id, &ServerPacket{
                        conv_id: packet.conv_id.clone(),
                        message: ServerMessage::RegisterResponse(NetResult::Ok(ServerInfo {
                            server_version: ENGINE_VERSION.to_string(),
                            mod_profile: self.mod_profile.profile.clone(),
                            mod_version: self.mod_profile.version.clone(),
                            mods: self.mod_profile.modstack.iter().map(|(name_path, flags)| {
                                let (name, path) = name_path.split_once(":").unwrap();
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
                    log!("registered client ip {}", addr);
                } else {
                    self.ns.send_raw(&addr.to_string(), &ServerPacket{
                        conv_id: packet.conv_id.clone(),
                        message: ServerMessage::RegisterResponse(NetResult::Err(
                            format!("client and server engine versions do not match: {} != {}", client_info.client_version, ENGINE_VERSION)
                        ))
                    })?;
                }
            }
            ClientMessage::Unregister => {
                log!("unregistered client ip {}", addr);
                self.ns.clients.remove(&packet.client_id);
            }
            ClientMessage::Ping(msg) => self.ns.send(&packet.client_id, &ServerPacket{
                conv_id: packet.conv_id.clone(),
                message: ServerMessage::Pong(msg.clone()),
            })?,
            ClientMessage::DownloadMod(name_path, offset) => {
                let (name, path) = name_path.split_once(":").unwrap();
                let client_path = format!("runtime/{path}/{name}_client.zip");
                let mut file = File::open(client_path)?;
                let mut buffer = [0;MAX_RAW_DATA_SIZE];
                file.seek(SeekFrom::Start(*offset))?;
                let len = file.read(&mut buffer[..])?;
                self.ns.send(&packet.client_id, &ServerPacket{
                    conv_id: packet.conv_id.clone(),
                    message: ServerMessage::RawData(buffer[..len].to_vec())
                })?;
            }
            _ => ()
        }
        Ok(())
    }

    pub(crate) fn request_response<F: Fn(&mut ServerRuntime, &ClientPacket) + 'static>(&mut self, client_id: &Id, packet: &ServerPacket, handler: F) -> Result<(), AError> {
        match self.ns.clients.get_mut(client_id) {
            Some(client) => {
                client.awaiting_replies.insert(packet.conv_id, Box::new(handler));
                self.ns.send(client_id, packet)?;
                Ok(())
            }
            None => {
                Err(AError::new(AET::NetworkError("invalid client".to_string())))
            }
        }
    }
}