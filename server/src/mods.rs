use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use aeonetica_engine::libloading::{Library, Symbol};
use aeonetica_engine::{log, nanoserde};
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::nanoserde::{DeRon, SerRon};
use server::{ServerMod, ServerModBox};
use crate::runtime::ServerRuntime;

#[derive(SerRon, DeRon)]
pub struct ModProfile {
    pub profile: String,
    pub version: String,
    pub modstack: HashMap<String, Vec<String>>
}

pub(crate) fn load_profile(mod_profile: &str) -> Result<ServerRuntime, AError>{
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
    Ok(ServerRuntime {
        mod_profile: profile,
        loaded_mods: mods,
    })
}

pub(crate) fn load_mod(name_path: &str) -> Result<ServerModBox, AError> {
    let name = name_path.rsplit_once("/").unwrap_or(("", name_path)).1;
    let mut archive = zip::read::ZipArchive::new(File::open(&format!("mods/{}.zip", name_path))?)
        .map_err(|e| AError::new(AET::IOError(format!("could not read zip file: {e}"))))?;
    std::fs::create_dir_all(&format!("runtime/{}", name_path)).expect("unable to create directory");
    for i in 0..archive.len() {
        let mut f = archive.by_index(i)
            .map_err(|e| AError::new(AET::IOError(format!("could not read zip file: {e}"))))?;
        let full_path = f.enclosed_name().unwrap().to_str().unwrap();
        if f.is_dir() {
            std::fs::create_dir_all(&format!("runtime/{name_path}/{full_path}"))?;
        } else {
            if let Some(p) = f.enclosed_name().unwrap().parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&format!("runtime/{name_path}/{full_path}"))?;
            std::io::copy(&mut f, &mut outfile)?;
        }
    }

    let server_lib = unsafe { Library::new(&format!("runtime/{name_path}/{name}_server.dll"))
        .map_err(|e| AError::new(AET::ModError(format!("could not load mod: {e}"))))? };
    let _create_mod_server: Symbol<fn() -> Box<dyn ServerMod>> = unsafe { server_lib.get("_create_mod_server".as_ref())
        .map_err(|e| AError::new(AET::ModError(format!("could not load mod: {e}"))))? };
    let mod_server = _create_mod_server();
    Ok(ServerModBox::new(mod_server, server_lib))
}
