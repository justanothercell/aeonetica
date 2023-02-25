use std::time::Instant;
use aeonetica_engine::{log, log_err};
use crate::ecs::Engine;
use crate::server_runtime::ServerRuntime;

pub const TPS: usize = 20;
pub const MSG_EVERY_N_TICKS: usize = 2;

pub fn run(ip: &str) {
    let runtime = ServerRuntime::create(ip).map_err(|e| {
        e.log_exit();
    }).unwrap();

    log!("running start for all mods");
    let mut engine = Engine::new(runtime);

    let mut_engine_ref = unsafe { &mut *(&mut engine as *mut Engine) };
    engine.runtime.loaded_mods.iter_mut().for_each(|m| {
        m.start(mut_engine_ref);
    });

    let mut tick_count = 0usize;

    let mut time = 0;

    loop {
        let t = Instant::now();

        let _ = engine.handle_queued().map_err(|e| {
            log_err!("{e}")
        });

        engine.timeout_inactive();

        if time > 10000000 / TPS {
            time -= 10000000 / TPS;
            engine.for_each_module(|engine, id, m| m.tick_dyn(id, engine));
            if tick_count % MSG_EVERY_N_TICKS == 0 {
                engine.send_messages();
            }
            tick_count = tick_count.wrapping_add(1);
        }

        time += t.elapsed().as_nanos() as usize;
    }
}