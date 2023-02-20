use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use aeonetica_engine::libloading::{Library, Symbol};
use aeonetica_engine::{log, nanoserde};
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::nanoserde::{DeBin, DeRon, SerBin, SerRon};
use aeonetica_engine::util::unzip_archive;
use server::{ServerMod, ServerModBox};
use crate::networking::NetworkServer;

pub struct ServerRuntime {
    pub(crate) mod_profile: ModProfile,
    pub(crate) loaded_mods: Vec<ServerModBox>,
    pub(crate) ns: NetworkServer
}

#[derive(SerRon, DeRon, SerBin, DeBin)]
pub struct ModProfile {
    pub profile: String,
    pub version: String,
    pub modstack: HashMap<String, Vec<String>>
}

impl ServerRuntime {
    pub(crate) fn create(mod_profile: &str, addr: &str) -> Result<ServerRuntime, AError> {
        let mut data = String::new();
        File::open(&format!("mods/{mod_profile}.ron"))?.read_to_string(&mut data)?;
        let profile: ModProfile = DeRon::deserialize_ron(&data)?;
        let mut mods = vec![];
        for item in &profile.modstack {
            log!("loading mod {} ...", item.0);
            let mut m = load_mod(item.0)
                .map_err(|mut e| {
                    e.additional_info.push(format!("could not load mod {}", item.0));
                    e
                })?;
            m.init(item.1);
            mods.push(m);
            log!("loaded mod {}", item.0)
        }
        log!("successfully loaded {} mods from profile {} v{}", mods.len(), profile.profile, profile.version);
        Ok(ServerRuntime {
            mod_profile: profile,
            loaded_mods: mods,
            ns: {
                let ns = NetworkServer::start(addr)?;
                log!("started server with ip {}", ns.socket.local_addr()?);
                ns
            }
        })
    }
}

pub(crate) fn load_mod(name_path: &str) -> Result<ServerModBox, AError> {
    let (name, path) = name_path.split_once(":").unwrap();

    unzip_archive(File::open(format!("mods/{path}.zip"))?, format!("runtime/{path}"))?;
    unzip_archive(File::open(format!("runtime/{path}/{name}_server.zip"))?, format!("runtime/{path}/server"))?;

    let server_lib = unsafe { Library::new(&format!("runtime/{path}/server/{name}_server.dll"))
        .map_err(|e| AError::new(AET::ModError(format!("could not load mod: {e}"))))? };
    let _create_mod_server: Symbol<fn() -> Box<dyn ServerMod>> = unsafe { server_lib.get("_create_mod_server".as_ref())
        .map_err(|e| AError::new(AET::ModError(format!("could not load mod: {e}"))))? };
    let mod_server = _create_mod_server();
    Ok(ServerModBox::new(mod_server, server_lib))
}