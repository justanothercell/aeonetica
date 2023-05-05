use aeonetica_engine::EntityId;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::module::Module;
use crate::common::{Chunk, Tile};

pub const WORLD: &str = "WORLD";

struct ChunkHolder {
    chunk: Chunk
}

impl ChunkHolder {
    pub(crate) fn new() -> ChunkHolder {
        ChunkHolder {
            chunk: Chunk::new(),
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
        engine.mut_entity(&eid).unwrap().add_module(World {
            origin: ChunkHolder::new()
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