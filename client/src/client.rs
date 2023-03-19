use std::time::{Instant};
use aeonetica_engine::{Id, log, log_err};
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
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
    });

    log!("sent login");

    let mut window = Window::new(false, Context::new());
    let mut time = 0;

    while !window.should_close() {
        window.render();
        let t = Instant::now();

        let _ = client.handle_queued().map_err(|e| {
            log_err!("{e}")
        });

        time += t.elapsed().as_nanos() as usize;
        window.poll_events();
    }

    log!("shutting down client after {time} ns");
}
