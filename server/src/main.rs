use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{log, log_err};
use crate::server_runtime::ServerRuntime;

mod server_runtime;
mod networking;


fn main() {
    // cargo run -- vanilla 0.0.0.0:6090
    let args: Vec<_> = std::env::args().skip(1).collect();
    log!("started server with args {args:?}");
    if args.len() < 2 {
        let e = AError::new(AET::ValueError(format!("expected command line arg <profile> <ip:port>, got {}", args.len())));
        e.log_exit();
    }
    let mut runtime = ServerRuntime::create(&args[0], &args[1]).map_err(|e| {
        e.log_exit();
    }).unwrap();


    loop {
        for (addr, packet) in runtime.ns.queued_packets() {
            println!("{packet:#?}");
            let _ = runtime.handle_packet(&addr, &packet).map_err(|e| {
                log_err!("{e}")
            });
        }
    }
}