use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use aeonetica_engine::uuid::Uuid;
use aeonetica_engine::error::AError;
use aeonetica_engine::{ENGINE_VERSION, Id, log, log_err};
use aeonetica_engine::nanoserde::DeRonTok::Str;
use aeonetica_engine::networking::client_packets::{ClientInfo, ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerInfo, ServerMessage, ServerPacket};
use aeonetica_engine::networking::NetResult;
use client::ClientModBox;
use crate::networking::NetworkClient;

#[derive(Debug, PartialEq)]
pub(crate) enum ClientState {
    Start,
    Registered,
    AquiredModList,
    DownloadingMods(String, usize),
    DownloadedMods,
}

pub(crate) struct ClientRuntime {
    pub(crate) client_id: Id,
    pub(crate) mod_profile: String,
    pub(crate) mod_profile_version: String,
    pub(crate) nc: NetworkClient,
    pub(crate) awaiting_replies: HashMap<Id, fn(&mut ClientRuntime, &ServerPacket)>,
    pub(crate) loaded_mods: Vec<ClientModBox>,
    pub(crate) state: ClientState,
    modloading: Modloading
}

pub(crate) struct Modloading{
    mod_list: HashMap<String, (String, String, Vec<String>, String, u64, u64, Vec<u8>, bool)>
}

impl ClientRuntime {
    pub(crate) fn create(client_id: Id, addr: &str, server_addr: &str) -> Result<Self, AError>{
        let nc = NetworkClient::start(addr, server_addr).map_err(|e| {
            e.log_exit();
        }).unwrap();
        log!("started client {addr} and initiating handshake to {server_addr}");
        let mut client = Self {
            client_id,
            nc,
            mod_profile: String::new(),
            mod_profile_version: String::new(),
            awaiting_replies: Default::default(),
            loaded_mods: vec![],
            state: ClientState::Start,
            modloading: Modloading {
                mod_list: vec![],
            }
        };
        client.register();
        Ok(client)
    }

    pub(crate) fn request_response(&mut self, packet: &ClientPacket, handler: fn(&mut ClientRuntime, &ServerPacket)) -> Result<(), AError> {
        self.awaiting_replies.insert(packet.conv_id, handler);
        self.nc.send(packet)?;
        Ok(())
    }

    fn register(&mut self) {
        self.request_response(&ClientPacket {
            client_id: self.client_id.clone(),
            conv_id: Uuid::new_v4().into_bytes(),
            message: ClientMessage::Register(ClientInfo {
                client_id: self.client_id,
                client_version: ENGINE_VERSION.to_string(),
            }),
        }, |client, resp| {
            match &resp.message {
                ServerMessage::RegisterResponse(res) => {
                    match res {
                        NetResult::Ok(info) => {
                            log!("successfully connected to server");
                            log!("registered client");
                            client.state = ClientState::Registered;
                            client.mod_profile = info.mod_profile.clone();
                            client.mod_profile_version = info.mod_version.clone();
                            log!("server has mod profile {} v{} with {} mod(s):", client.mod_profile, client.mod_profile_version, info.mods.len());
                            client.modloading.mod_list = info.mods.clone().into_iter()
                                .map(|(name_path, flags, hash, size)| {
                                    let (name, path) = name_path.split_once(":").unwrap();
                                    let mut local_hash = String::new();
                                    let _ = File::open(&format!("runtime/{path}.hash")).map(|mut f| f.read_to_string(&mut local_hash));
                                    let available = local_hash.trim() == &hash;
                                    log!("  - {name_path}");
                                    if !available {
                                        let _ = std::fs::remove_dir_all(&format!("runtime/{path}"));
                                    }
                                    (name.to_string(), path.to_string(), flags, hash, size, 0, vec![], available)
                                }).collect();
                            log!("downloading {} mod(s)", client.modloading.mod_list.iter().filter(|m| !m.7).count());
                        }
                        NetResult::Err(msg) => {
                            log_err!("server did not accept connection: {msg}");
                            exit(1);
                        }
                    }
                },
                e => {
                    log_err!("invalid response: {e:?}");
                    exit(1);
                }
            }
        }).map_err(|e| {
            e.log_exit();
        }).unwrap();
        while self.state != ClientState::Registered {
            for packet in self.nc.queued_packets() {
                self.handle_packet(&packet).map_err(|e| {
                    e.log_exit();
                }).unwrap();
            }
        }
    }

    fn load_mods(&mut self) {

    }

    fn gracefully_abort(&self){
        let _ = self.nc.send(&ClientPacket {
            client_id: self.client_id,
            conv_id: Uuid::new_v4().into_bytes(),
            message: ClientMessage::Unregister,
        });
        log_err!("gracefully aborted client");
        exit(1);
    }
}