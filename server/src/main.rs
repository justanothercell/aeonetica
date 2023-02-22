use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{Id, log, log_err, MAX_CLIENT_TIMEOUT};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use server::ecs::World;
use crate::server_runtime::ServerRuntime;

mod server_runtime;
mod networking;


fn main() {
    // cargo run -- 0.0.0.0:6090
    let args: Vec<_> = std::env::args().skip(1).collect();
    log!("started server with args {args:?}");
    if args.is_empty() {
        let e = AError::new(AET::ValueError(format!("expected command line arg ip:port>, got {}", args.len())));
        e.log_exit();
    }
    let mut runtime = ServerRuntime::create(&args[0]).map_err(|e| {
        e.log_exit();
    }).unwrap();

    let mut world = World::new();
    runtime.loaded_mods.iter_mut().fold((), |_, m| {
        m.start(&mut world);
    });

    loop {
        for (addr, packet) in runtime.ns.queued_packets() {
            let _ = runtime.handle_packet(&addr, &packet).map_err(|e| {
                log_err!("{e}")
            });
        }
        let mut removed = vec![];
        runtime.ns.clients.retain(|id, client| {
            if client.last_seen.elapsed().as_millis() < MAX_CLIENT_TIMEOUT {
                true
            } else {
                removed.push(*id);
                log!("timed out client ip {}", client.client_addr);
                false
            }
        });
        for r in removed {
            let _ = runtime.ns.send(&r, &ServerPacket {
                conv_id: Id::new(),
                message: ServerMessage::Unregister("TIMEOUT".to_string()),
            });
        }
    }
}