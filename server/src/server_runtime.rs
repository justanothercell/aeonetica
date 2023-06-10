use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use aeonetica_engine::error::builtin::ModError;
use aeonetica_engine::libloading::{Library, Symbol};
use aeonetica_engine::{log, nanoserde};
use aeonetica_engine::error::*;
use aeonetica_engine::nanoserde::{DeBin, DeRon, SerBin, SerRon};
use aeonetica_engine::util::unzip_archive;
use crate::{ServerMod, ServerModBox};
use crate::networking::NetworkServer;


mod paths_util {
    #[cfg(target_os = "windows")]
    pub(crate) const MOD_FILE_EXTENSION: &str = "dll";
    #[cfg(target_os = "linux")]
    pub(crate) const MOD_FILE_EXTENSION: &str = "so";

    pub(crate) fn server_lib(path: &str, name: &str) -> String {
        format!("runtime/{path}/server/{name}_server.{MOD_FILE_EXTENSION}")
    }
    pub(crate) fn mod_zip(path: &str) -> String {
        format!("mods/{path}.zip")
    }
    pub(crate) fn mod_server_zip(path: &str, name: &str) -> String {
        format!("runtime/{path}/{name}_server.zip")
    }
    pub(crate) fn mod_client_zip(path: &str, name: &str, target: &str) -> String {
        format!("runtime/{path}/{name}_client-{target}.zip")
    }
}

pub(crate) use paths_util::*;

pub struct ServerRuntime {
    pub(crate) mod_profile: ModProfile,
    pub(crate) supported_mod_targets: HashSet<String>,
    pub(crate) loaded_mods: Vec<ServerModBox>,
    pub(crate) ns: Rc<RefCell<NetworkServer>>
}

#[derive(SerRon, DeRon, SerBin, DeBin)]
pub struct ModProfile {
    pub profile: String,
    pub version: String,
    pub mod_targets: Option<Vec<String>>,
    pub modstack: HashMap<String, Vec<String>>
}

impl ServerRuntime {
    pub(crate) fn create(addr: &str) -> ErrorResult<ServerRuntime> {
        let mut data = String::new();
        File::open("mods/mods.ron")?.read_to_string(&mut data)?;
        let profile: ModProfile = DeRon::deserialize_ron(&data)?;
        let mod_targets = HashSet::from_iter(profile.mod_targets.clone().unwrap_or(vec![aeonetica_engine::MOD_TARGET.to_string()]).iter().cloned());
        let mut mods = vec![];
        log!("loading mods for targets {mod_targets:?}");
        for item in &profile.modstack {
            log!("loading mod {} ...", item.0);
            let mut m = load_mod(item.0, &mod_targets)
                .map_err(|mut e| {
                    e.add_info(format!("could not load mod {}", item.0));
                    e
                })?;
            m.init(item.1);
            mods.push(m);
            log!("loaded mod {}", item.0)
        }
        log!("successfully loaded {} mods from profile {} v{}", mods.len(), profile.profile, profile.version);
        Ok(ServerRuntime {
            supported_mod_targets: mod_targets,
            mod_profile: profile,
            loaded_mods: mods,
            ns: Rc::new(RefCell::new(NetworkServer::start(addr)?))
        })
    }
}

pub(crate) fn load_mod(name_path: &str, supported_mod_targets: &HashSet<String>) -> ErrorResult<ServerModBox> {
    let (path, name) = name_path.split_once(':').unwrap();

    unzip_archive(File::open(mod_zip(path))?, format!("runtime/{path}"))?;
    unzip_archive(File::open(mod_server_zip(path, name))?, format!("runtime/{path}/server"))?;

    for target in supported_mod_targets {
        if !Path::new(&mod_client_zip(path, name, target)).exists() {
            Err(Error::new(ModError(format!("Mod {name_path} does not support target advertised in mods.ron: {target}\n(of {supported_mod_targets:?})")), Fatality::FATAL, false))?;
        }
    }

    let server_lib_file = server_lib(path, name);
    log!(DEBUG, "loading lib: {}", server_lib_file);
    let server_lib = unsafe { Library::new(server_lib_file)
        .map_err(|e| Error::new(ModError(format!("could not load mod: {e}")), Fatality::FATAL, false))? };
    let _create_mod_server: Symbol<fn() -> Box<dyn ServerMod>> = unsafe { server_lib.get("_create_mod_server".as_ref())
        .map_err(|e| Error::new(ModError(format!("could not load mod: {e}")), Fatality::FATAL, false))? };
    let mod_server = _create_mod_server();
    Ok(ServerModBox::new(mod_server, server_lib))
}
