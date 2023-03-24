use std::time::{Instant};
use aeonetica_engine::{Id, log, log_err};
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::SendMode;
use crate::client_runtime::ClientRuntime;
use crate::renderer::context::Context;
use crate::renderer::window::Window;

pub fn run(ip: &str, server_ip: &str) {
    let client_id = Id::new();

    let mut client = ClientRuntime::create(client_id, ip, server_ip).map_err(|e| {
        e.log_exit();
    }).unwrap();

    let _ = client.nc.borrow().send(&ClientPacket {
        client_id,
        conv_id: Id::new(),
        message: ClientMessage::Login,
    }, SendMode::Safe);

    log!("sent login");

    let mut window = Window::new(false, Context::new());
    let mut time = 0;
    let mut frames = 0;
    let mut last_time = 0;

    while !window.should_close() {
        let t = Instant::now();
        window.render(time);

        let _ = client.handle_queued().map_err(|e| {
            log_err!("{e}")
        });

        window.poll_events();
        
        frames += 1;
        time += t.elapsed().as_nanos() as usize;

        if time - last_time >= 1_000_000_000 {
            log!("fps: {}", frames);
            last_time = time;
            frames = 0;
        }
    }

    log!("shutting down client after {time} ns");
}
