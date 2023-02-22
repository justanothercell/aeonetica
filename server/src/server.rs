use std::task::Wake;
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{Id, log, log_err, MAX_CLIENT_TIMEOUT};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use crate::ecs::Engine;
use crate::server_runtime::ServerRuntime;

pub fn run(ip: &str) {
    let runtime = ServerRuntime::create(&ip).map_err(|e| {
        e.log_exit();
    }).unwrap();

    let mut engine = Engine::new(runtime);
    let mut_engine_ref = unsafe { &mut *(&mut engine as *mut Engine) };
    engine.runtime.loaded_mods.iter_mut().fold((), |_, m| {
        m.start(mut_engine_ref);
    });

    loop {
        for (addr, packet) in engine.runtime.ns.queued_packets() {
            let _ = engine.runtime.handle_packet(&addr, &packet).map_err(|e| {
                log_err!("{e}")
            });
        }
        let mut removed = vec![];
        engine.runtime.ns.clients.retain(|id, client| {
            if client.last_seen.elapsed().as_millis() < MAX_CLIENT_TIMEOUT {
                true
            } else {
                removed.push(*id);
                log!("timed out client ip {}", client.client_addr);
                false
            }
        });
        for r in removed {
            let _ = engine.runtime.ns.send(&r, &ServerPacket {
                conv_id: Id::new(),
                message: ServerMessage::Unregister("TIMEOUT".to_string()),
            });
        }
    }
}