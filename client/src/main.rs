#![feature(int_roundings)]

use std::collections::HashMap;
use std::time::Instant;
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{Id, log, log_err};
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use client::MSG_PER_SECOND;
use crate::client_runtime::ClientRuntime;

mod networking;
mod client_runtime;
mod window;
mod layers;
mod events;
mod context;

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

    //let window = window::Window::new(context::Context::new()).expect("error creating main window");
    //window.run();

    let mut client = ClientRuntime::create(client_id, &args[0], &args[1]).map_err(|e| {
        e.log_exit();
    }).unwrap();

    let _ = client.nc.send(&ClientPacket {
        client_id,
        conv_id: Id::new(),
        message: ClientMessage::Login,
    });

    log!("sent login");

    let mut time = 0;

    loop {
        let t = Instant::now();

        let _ = client.handle_queued().map_err(|e| {
            log_err!("{e}")
        });

        if time > 10000000 / MSG_PER_SECOND {
            time -= 10000000 / MSG_PER_SECOND;
            let mut messages = HashMap::new();
            for (id, handle) in &mut client.handles {
                let data = handle.send_data();
                if !data.is_empty() {
                    messages.insert(id, data);
                }
            }
        }

        time += t.elapsed().as_nanos() as usize;
    }
}
