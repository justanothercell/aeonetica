use std::io::empty;
use std::process::id;
use aeonetica_engine::{EntityId, log};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::vector::Vector2;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ecs::module::Module;
use crate::client::WorldHandle;
use crate::common::{Chunk, Tile};

pub const WORLD: &str = "WORLD";

struct ChunkHolder {
    chunk: Chunk
}

impl ChunkHolder {
    pub(crate) fn new(chunk_pos: Vector2<u32>) -> ChunkHolder {
        ChunkHolder {
            chunk: Chunk::new(chunk_pos),
        }
    }
}

pub struct World {
    origin: ChunkHolder
}

impl World {
    pub(crate) fn new_wold_entity(engine: &mut Engine) -> EntityId {
        let eid = engine.new_entity();
        engine.tag_entity(eid, WORLD);
        let entity = engine.mut_entity(&eid).unwrap();
        entity.add_module(Messenger::new::<WorldHandle>());

        entity.add_module(ConnectionListener::new(
            |id, engine, client| {
                log!("sent chunk whether they wanted or not: {client}");
                let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
                messenger.add_client(*client);
                messenger.call_client_fn_for(WorldHandle::receive_chunk_data, &client, Chunk::new((0, 0).into()), SendMode::Safe);
            },
            |_id, _engine, client| {
                log!("user said bye bye to world: {client}");

            }));
        
        entity.add_module(World {
            origin: ChunkHolder::new((0, 0).into())
        });
        eid
    }

    pub fn get_tile_at(x: i32, y: i32) -> Tile {
        0
    }

    pub fn set_tile_at(x: i32, y: i32, t: Tile) {

    }
}

impl Module for World {
    fn start(id: &EntityId, engine: &mut Engine) where Self: Sized {

    }
}