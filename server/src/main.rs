use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{log};
use crate::mods::load_profile;
use crate::networking::{NetworkServer};

mod mods;
mod runtime;
mod networking;


fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    log!("started server with args {args:?}");
    if args.len() < 1 {
        let e = AError::new(AET::ValueError(format!("expected command line arg <profile>, got {}", args.len())));
        e.log_exit();
    }
    let runtime = load_profile(&args[0]).map_err(|e| {
        e.log_exit();
    }).unwrap();
    log!("successfully loaded {} mods from profile {} v{}", runtime.loaded_mods.len(), runtime.mod_profile.profile, runtime.mod_profile.version);

    let mut ns = NetworkServer::start("0.0.0.0:6090").map_err(|e| {
        e.log_exit();
    }).unwrap();
    loop {
        for packet in ns.queued_packets() {
            println!("{packet:#?}")
        }
    }
}