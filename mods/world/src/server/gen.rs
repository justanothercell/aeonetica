use aeonetica_engine::log;
use aeonetica_engine::math::vector::Vector2;
use crate::common::{CHUNK_SIZE, Population};
use crate::server::world::World;
use crate::tiles::Tile;


impl World {
    pub(crate) fn populate(&mut self, chunk_pos: Vector2<i32>) {
        log!(WARN, "populated {chunk_pos:?}");
        let chunk = self.mut_chunk_at_raw(chunk_pos);
        for i in 0..CHUNK_SIZE as i32 {
            chunk.set_tile((i, 0).into(), Tile::StoneBrick);
            chunk.set_tile((0, i).into(), Tile::StoneBrick);
        }
        chunk.set_tile((chunk.chunk_pos.x.abs(), chunk.chunk_pos.y.abs()).into(), Tile::Stone);
        chunk.population = Population::Finished;
    }
}