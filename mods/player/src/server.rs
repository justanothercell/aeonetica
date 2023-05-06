use aeonetica_engine::EntityId;
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::vector::Vector2;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ServerMod;

pub struct PlayerModServer {

}

impl ServerMod for PlayerModServer {
    fn start(&mut self, engine: &mut Engine) {
        let eid = engine.new_entity();
        let handler = engine.mut_entity(&eid).unwrap();
        handler.add_module(Messenger::new::<WorldHandle>());

        handler.add_module(ConnectionListener::new(
            |id, engine, client| {
                let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
                messenger.add_client(*client);
                messenger.call_client_fn_for(WorldHandle::receive_chunk_data, &client, Chunk::new((0, 0).into()), SendMode::Quick);
            },
            |_id, _engine, client| {

            }));

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
    fn start(id: &EntityId, engine: &mut Engine) where Self: Sized {
        todo!()
    }
}