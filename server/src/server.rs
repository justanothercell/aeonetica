use std::time::{Duration, Instant};
use aeonetica_engine::time::Time;
use aeonetica_engine::{log};
use crate::ecs::Engine;
use crate::server_runtime::ServerRuntime;

const TPS: usize = 20;
const FULL_SEC: usize = 1_000_000_000;

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

    let mut time_nanos = 0;
    let mut time = Time {
        time: 0.0,
        delta: 0.0
    };

    println!("\x1b[38;5;200mServer successfully set up and ready for clients to connect\x1b[0m");

    loop {
        let t = Instant::now();

        let _ = engine.handle_queued().map_err(|e| {
            log!(ERROR, "{e}")
        });

        engine.timeout_inactive();

        engine.for_each_module(|engine, id, m| m.tick_dyn(id, engine, time));
        engine.run_tasks();

        let delta_time_nanos_intermediate = t.elapsed().as_nanos();
        

        engine.tick += 1;

        if (delta_time_nanos_intermediate as usize) < 1_000_000_000 / TPS {
            let to_wait = 1_000_000_000 / TPS - delta_time_nanos_intermediate as usize;
            //println!("to_wait = {}", to_wait as f32 / 1_000_000_000.0);
            std::thread::sleep(Duration::from_nanos(to_wait as u64));
        }

        let delta_time_nanos = t.elapsed().as_nanos();
        time_nanos += delta_time_nanos;
        time.raw_delta = delta_time_nanos as f32 / FULL_SEC as f32;
		time.delta = time.raw_delta.min(0.2);
        time.time = time_nanos as f32 / FULL_SEC as f32;
        //println!("time.delta = {}  percentage = {}", time.delta, time.delta / (1.0 / TPS as f32) * 100.0);
    }
}