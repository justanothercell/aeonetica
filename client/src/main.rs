#![feature(int_roundings)]

use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{Id, log, log_err};
use crate::client_runtime::ClientRuntime;

mod networking;
mod client_runtime;

fn main() {
    // nc -u 127.0.01 6090
    // cargo run -- 127.0.0.1:9000 127.0.0.1:6090
    let args: Vec<_> = std::env::args().skip(1).collect();
    log!("started client with args {args:?}");
    if args.len() < 2 {
        let e = AError::new(AET::ValueError(format!("expected command line arg <local_ip:port> <server_ip:port>, got {}", args.len())));
        e.log_exit();
    }
    let client_id = Id::new();
    let mut client = ClientRuntime::create(client_id, &args[0], &args[1]).map_err(|e| {
        e.log_exit();
    }).unwrap();

    loop {
        for packet in client.nc.queued_packets() {
            let _ = client.handle_packet(&packet).map_err(|e| {
                log_err!("{e}")
            });
        }
    }
}