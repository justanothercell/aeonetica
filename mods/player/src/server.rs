use aeonetica_engine::util::vector::Vector2;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ServerMod;

pub struct PlayerModServer {

}

impl ServerMod for PlayerModServer {
    fn start(&mut self, engine: &mut Engine) {
        let eid = engine.new_entity();
        let handler = engine.mut_entity(&eid).unwrap();
        handler.add_module(PlayerHandler{});
    }
}

struct PlayerHandler {

}

impl Module for PlayerHandler {

}

struct Player {
    position: Vector2<f32>
}

impl Player {
    pub(crate) fn new(position: Vector2<f32>) -> Self {
        Self {
            position
        }
    }
}

impl Module for Player {

}