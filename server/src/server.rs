use std::net::UdpSocket;
use aeonetica_engine::error::{AET};
use aeonetica_engine::{Id, log, log_err, MAX_CLIENT_TIMEOUT};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use crate::ecs::Engine;
use crate::server_runtime::ServerRuntime;

pub const TPS: usize = 20;
pub const SNG_EVERY_N_TICKS: usize = 2;

pub fn run(ip: &str) {
    let runtime = ServerRuntime::create(&ip).map_err(|e| {
        e.log_exit();
    }).unwrap();

    log!("running start for all mods");
    let mut engine = Engine::new(runtime);
    let mut_engine_ref = unsafe { &mut *(&mut engine as *mut Engine) };
    engine.runtime.loaded_mods.iter_mut().fold((), |_, m| {
        m.start(mut_engine_ref);
    });



    loop {
        let _ = engine.handle_queued().map_err(|e| {
            log_err!("{e}")
        })
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