use std::cell::{RefCell};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Cursor, Write};
use std::process::exit;
use std::thread;
use std::rc::Rc;
use std::time::Duration;
use aeonetica_engine::libloading::{Library, Symbol};
use aeonetica_engine::error::{AError,AET};
use aeonetica_engine::nanoserde::SerBin;
use aeonetica_engine::{ENGINE_VERSION, Id, log, log_err, MAX_CLIENT_TIMEOUT};
use aeonetica_engine::networking::client_packets::{ClientInfo, ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use aeonetica_engine::networking::{MAX_RAW_DATA_SIZE, NetResult};
use aeonetica_engine::util::unzip_archive;
use client::{ClientMod, ClientModBox};
use crate::networking::NetworkClient;

#[cfg(target_os = "windows")]
mod paths_util {
    pub(crate) const MOD_FILE_EXTENSION: &str = ".dll";
    pub(crate) fn client_lib(path: &str, name: &str) -> String {
        format!("runtime/{path}/{name}_client{MOD_FILE_EXTENSION}")
    }
}

#[cfg(target_os = "linux")]
mod paths_util {
    pub(crate) const MOD_FILE_EXTENSION: &str = ".so";
    pub(crate) fn client_lib(path: &str, name: &str) -> String {
        format!("runtime/{path}/{name}_client{MOD_FILE_EXTENSION}")
    }
}

mod paths_util_common {
    pub(crate) fn mod_hash(path: &str) -> String {
        format!("runtime/{path}.hash")
    }
}

pub(crate) use paths_util::*;
use crate::client_runtime::paths_util_common::mod_hash;

#[derive(Debug, PartialEq)]
pub(crate) enum ClientState {
    Start,
    Registered,
    DownloadedMods,
}

pub(crate) struct ClientRuntime {
    pub(crate) client_id: Id,
    pub(crate) mod_profile: String,
    pub(crate) mod_profile_version: String,
    pub(crate) nc: NetworkClient,
    pub(crate) awaiting_replies: HashMap<Id, Box<dyn Fn(&mut ClientRuntime, &ServerPacket)>>,
    pub(crate) loaded_mods: Vec<ClientModBox>,
    pub(crate) state: ClientState
}

pub(crate) struct LoadingMod{
    name: String,
    path: String,
    flags: Vec<String>,
    hash: String,
    total_size: u64,
    size: u64,
    data: Vec<u8>,
    available: bool
}

type LoadingModList = Rc<RefCell<HashMap<String, Rc<RefCell<LoadingMod>>>>>;

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
            state: ClientState::Start
        };
        let mut mod_list = client.register()?;
        let timeout_socket = client.nc.socket.try_clone()?;
        thread::spawn(move || {
            loop {
                let data = SerBin::serialize_bin(&ClientPacket {
                    client_id: client_id.clone(),
                    conv_id: Id::new(),
                    message: ClientMessage::KeepAlive,
                });
                let _ = timeout_socket.send(data.as_slice()).map_err(|e|{
                    let e: AError = e.into();
                    log_err!("{e}");
                    exit(1);
                });
                std::thread::sleep(Duration::from_millis((MAX_CLIENT_TIMEOUT - 1000) as u64))
            }
        });
        log!("started timeout preventer");
        client.download_mods(&mut mod_list).map_err(|e| client.gracefully_abort(e));
        client.enable_mods(&mut mod_list).map_err(|e| client.gracefully_abort(e));
        Ok(client)
    }

    pub(crate) fn request_response<F: Fn(&mut ClientRuntime, &ServerPacket) + 'static>(&mut self, packet: &ClientPacket, handler: F) -> Result<(), AError> {
        self.awaiting_replies.insert(packet.conv_id, Box::new(handler));
        self.nc.send(packet)?;
        Ok(())
    }

    fn register(&mut self) -> Result<LoadingModList, AError>{
        let mod_list = Rc::new(RefCell::new(HashMap::new()));
        let mod_list_filler = mod_list.clone();
        self.request_response(&ClientPacket {
            client_id: self.client_id,
            conv_id: Id::new(),
            message: ClientMessage::Register(ClientInfo {
                client_id: self.client_id,
                client_version: ENGINE_VERSION.to_string(),
            }),
        }, move |client, resp| {
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
                            let local_mod_list: HashMap<_, _> = info.mods.clone().into_iter()
                                .map(|(name_path, flags, hash, size)| {
                                    let (name, path) = name_path.split_once(':').unwrap();
                                    let mut local_hash = String::new();
                                    let _ = File::open(mod_hash(path)).map(|mut f| f.read_to_string(&mut local_hash));
                                    let available = local_hash.trim() == hash;
                                    log!("  - {name_path}");
                                    if !available {
                                        let _ = std::fs::remove_dir_all(format!("runtime/{path}"));
                                        (name_path.clone(),  Rc::new(RefCell::new(LoadingMod {
                                            name: name.to_string(),
                                            path: name.to_string(),
                                            flags,
                                            hash,
                                            total_size: size,
                                            size: 0,
                                            data: vec![0;size as usize],
                                            available: false,
                                        })))
                                    } else {
                                        (name_path.clone(), Rc::new(RefCell::new(LoadingMod {
                                            name: name.to_string(),
                                            path: name.to_string(),
                                            flags,
                                            hash,
                                            total_size: size,
                                            size, // already fully available, is redundant
                                            data: vec![],
                                            available: true,
                                        })))
                                    }
                                }).collect();
                            mod_list_filler.replace(local_mod_list);
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
        })?;
        while self.state != ClientState::Registered {
            for packet in self.nc.queued_packets() {
                self.handle_packet(&packet)?;
            }
        }
        Ok(mod_list)
    }

    fn download_mods(&mut self, mod_list: &LoadingModList) -> Result<(), AError>{
        log!("downloading {} mod(s)", mod_list.borrow().values().filter(|m| !m.borrow().available).count());
        let mut total = 0;
        let mut borrowed_ml = mod_list.borrow_mut();
        for (name_path, lm) in borrowed_ml.iter_mut() {
            let lmb = lm.borrow_mut();
            if lmb.available {
                continue
            }
            log!("downloading mod {name_path} across {} packets", lmb.total_size.div_ceil(MAX_RAW_DATA_SIZE as u64));
            total += lmb.total_size;
            for i in (0..lmb.total_size).step_by(MAX_RAW_DATA_SIZE) {
                let lm = lm.clone();
                self.request_response(&ClientPacket {
                    client_id: self.client_id,
                    conv_id: Id::new(),
                    message: ClientMessage::DownloadMod(name_path.clone(), i),
                }, move |_client, resp| {
                    let mut lmb = lm.borrow_mut();
                    match &resp.message {
                        ServerMessage::RawData(data) => {
                            lmb.size += data.len() as u64;
                            lmb.data.splice(i as usize..(i as usize+data.len()), data.to_owned());
                        },
                        e => {
                            log_err!("invalid response: {e:?}");
                            exit(1);
                        }
                    }
                }).map_err(|e| {
                    e.log_exit();
                }).unwrap();
            }
        }

        let mut p = 0.0;
        while self.state != ClientState::DownloadedMods {
            for packet in self.nc.queued_packets() {
                self.handle_packet(&packet)?;
            }

            let mut downloaded = 0;
            for (key, lm) in borrowed_ml.iter_mut(){
                let mut lm = lm.borrow_mut();
                if !lm.available {
                    downloaded += lm.size;
                    if lm.size == lm.total_size {
                        lm.available = true;
                        unzip_archive(Cursor::new(&lm.data), &format!("runtime/{}", lm.path))?;
                        File::create(mod_hash(&lm.path)).unwrap().write_all(lm.data.as_slice())?;
                        log!("finished downloading mod {}", key)
                    }
                }
            }
            if downloaded as f32 / total as f32 - p > 0.2 {
                p = downloaded as f32 / total as f32;
                log!("progress: {p}")
            }
            if downloaded == total {
                self.state = ClientState::DownloadedMods
            }
        }
        log!("downloaded all missing mods");
        Ok(())
    }

    fn enable_mods(&mut self, mod_list: &LoadingModList) -> Result<(), AError>{
        for (name_path, lm) in mod_list.borrow_mut().iter_mut() {
            log!("loading mod {} ...", name_path);
            let mut loaded_mod = load_mod(name_path)?;
            loaded_mod.init(&lm.borrow().flags);
            self.loaded_mods.push(loaded_mod);
            log!("loaded mod {} ...", name_path);
        }
        log!("successfully loaded {} mods from profile {} v{}", self.loaded_mods.len(), self.mod_profile, self.mod_profile_version);
        Ok(())
    }

    fn gracefully_abort<E: Into<AError>>(&self, e: E) -> !{
        let err = e.into();
        err.log();
        let _ = self.nc.send(&ClientPacket {
            client_id: self.client_id,
            conv_id: Id::new(),
            message: ClientMessage::Unregister,
        });
        log_err!("gracefully aborted client");
        exit(1);
    }
}

pub(crate) fn load_mod(name_path: &str) -> Result<ClientModBox, AError> {
    let (name, path) = name_path.split_once(':').unwrap();
    let client_lib = unsafe { Library::new(client_lib(path, name))
        .map_err(|e| AError::new(AET::ModError(format!("could not load mod: {e}"))))? };
    let _create_mod_client: Symbol<fn() -> Box<dyn ClientMod>> = unsafe { client_lib.get("_create_mod_client".as_ref())
        .map_err(|e| AError::new(AET::ModError(format!("could not load mod create function: {e}"))))? };
    let mod_client = _create_mod_client();
    Ok(ClientModBox::new(mod_client, client_lib))
}
