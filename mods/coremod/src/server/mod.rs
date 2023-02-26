mod world;
mod player;

use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ServerMod;
use crate::server::player::PlayerHandler;
use crate::server::world::WorldGen;

pub struct CoreModServer {

}

impl ServerMod for CoreModServer {
    fn start(&mut self, engine: &mut Engine) {
        let world = engine.new_entity();
        engine.tag_entity(world, "WORLD".to_string());
        engine.mut_entity(&world).unwrap().add_module(WorldGen::new());

        let player_handler = engine.new_entity();
        engine.tag_entity(world, "PLAYER_HANDLER".to_string());
        if let Some(player_handler) = engine.mut_entity(&player_handler) {
            player_handler.add_module(PlayerHandler {
                players: Default::default(),
            });
            player_handler.add_module(ConnectionListener::new(|id, world, player| {

            }, |id, world, player| {

            }));
        }
    }
}