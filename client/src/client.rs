use std::time::Instant;
use aeonetica_engine::*;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::SendMode;
use crate::client_runtime::ClientRuntime;
use crate::data_store::DataStore;
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

    let mut window = Window::new(false);
    let mut time = 0;
    let mut frames = 0;
    let mut last_full_sec = 0;
    let mut delta_time = 0;

    let mut store = DataStore::new();
    let mut context = Context::new();

    client.loaded_mods.iter()
        .for_each(|loaded_mod| loaded_mod.client_mod.start(&mut context, &mut store, window.context_provider()));

    while !window.should_close() {
        let t = Instant::now();

        window.poll_events(&mut client, &mut context);
        
        let _ = client.handle_queued(&mut store).map_err(|e| {
            log_err!("{e}")
        });
        
        window.render(&mut context, &mut client, &mut store, delta_time as f64 / 1_000_000_000.0);
        
        delta_time = t.elapsed().as_nanos() as usize;
        time += delta_time;
        
        frames += 1;

        if time - last_full_sec >= 1_000_000_000 {
            log!("fps: {}", frames);
            last_full_sec = time;
            frames = 0;
        }
    }

    log!("shutting down client after {time} ns");
    context.finish();
    window.finish();
    client.nc.borrow().send(&ClientPacket {
        client_id: client.client_id,
        conv_id: Id::new(),
        message: ClientMessage::Logout,
    }, SendMode::Safe).expect("couldnt exit client");
}
