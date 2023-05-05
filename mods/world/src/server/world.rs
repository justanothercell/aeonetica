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
use crate::common::{Chunk, CHUNK_SIZE, Tile};

pub const WORLD: &str = "WORLD";

struct ChunkHolder {
    further_x: Option<Box<ChunkHolder>>,
    further_y: Option<Box<ChunkHolder>>,
    chunk: Chunk
}

impl ChunkHolder {
    pub(crate) fn new(chunk_pos: Vector2<i32>) -> ChunkHolder {
        ChunkHolder {
            further_x: None,
            further_y: None,
            chunk: Chunk::new(chunk_pos),
        }
    }
}

pub struct World {
    origin_ne: ChunkHolder,
    origin_se: ChunkHolder,
    origin_nw: ChunkHolder,
    origin_sw: ChunkHolder,
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
            origin_ne: ChunkHolder::new((0, 0).into()),
            origin_se: ChunkHolder::new((0, -1).into()),
            origin_nw: ChunkHolder::new((-1, 0).into()),
            origin_sw: ChunkHolder::new((-1, -1).into())
        });
        eid
    }

    pub fn get_tile_at(&mut self, pos: Vector2<i32>) -> Tile {
        self.get_chunk_at(pos / 16).get_tile(((pos % CHUNK_SIZE as i32) + (CHUNK_SIZE as i32, CHUNK_SIZE as i32).into()) % CHUNK_SIZE as i32)
    }

    pub fn set_tile_at(&mut self, pos: Vector2<i32>, t: Tile) {
        self.mut_chunk_at(pos / 16).set_tile(((pos % CHUNK_SIZE as i32) + (CHUNK_SIZE as i32, CHUNK_SIZE as i32).into()) % CHUNK_SIZE as i32, t)
    }

    pub fn mut_chunk_at(&mut self, chunk_pos: Vector2<i32>) -> &mut Chunk {
        let mut cp = chunk_pos;
        let mut chunk_ref = match (chunk_pos.x >= 0, chunk_pos.y >= 0) {
            (true, true) => {
                &mut self.origin_ne
            },
            (true, false) => {
                cp.y = -cp.y - 1;
                &mut self.origin_se
            },
            (false, true) => {
                cp.x = -cp.x - 1;
                &mut self.origin_nw
            },
            (false, false) => {
                cp.x = -cp.x - 1;
                cp.y = -cp.y - 1;
                &mut self.origin_sw
            },
        };
        while cp.x > 0 {
            cp.x -= 1;
            if chunk_ref.further_x.is_none() {
                let mut pos = chunk_ref.chunk.chunk_pos;
                if chunk_pos.x < 0 { pos.x = -(pos.x + 1) };
                if chunk_pos.y < 0 { pos.y = -(pos.y + 1) };
                chunk_ref.further_x = Some(Box::new(ChunkHolder::new(pos)))
            }
            chunk_ref = chunk_ref.further_x.as_mut().unwrap();
        }
        while cp.y > 0 {
            cp.y -= 1;
            if chunk_ref.further_y.is_none() {
                let mut pos = chunk_ref.chunk.chunk_pos;
                if chunk_pos.x < 0 { pos.x = -(pos.x + 1) };
                if chunk_pos.y < 0 { pos.y = -(pos.y + 1) };
                chunk_ref.further_y = Some(Box::new(ChunkHolder::new(pos)))
            }
            chunk_ref = chunk_ref.further_y.as_mut().unwrap();
        }
        &mut chunk_ref.chunk
    }

    pub fn get_chunk_at(&mut self, chunk_pos: Vector2<i32>) -> &Chunk {
        self.mut_chunk_at(chunk_pos)
    }
}

impl Module for World {
    fn start(id: &EntityId, engine: &mut Engine) where Self: Sized {

    }
}