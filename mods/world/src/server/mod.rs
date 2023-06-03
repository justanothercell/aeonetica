pub mod world;
pub(crate) mod gen;

use aeonetica_server::ServerMod;

use aeonetica_engine::log;
use aeonetica_server::ecs::Engine;
use crate::server::world::World;

pub struct WorldModServer {
    seed: u64
}

impl WorldModServer {
    pub(crate) fn new() -> Self {
        Self {
            seed: 0
        }
    }
}

impl ServerMod for WorldModServer {
    fn init(&mut self, flags: &Vec<String>) {
        if !flags.is_empty() {
            self.seed = flags[0].parse().unwrap_or_else(|_| panic!("seed '{}' is not a valid integer", flags[0]));
            log!("found seed {}", self.seed);
        } else {
            self.seed = rand::random();
            log!(DEBUG, "No seed found. Generated {}", self.seed);
        }
    }

    fn start(&mut self, engine: &mut Engine) {
        World::new_wold_entity(engine, self.seed);
    }
}