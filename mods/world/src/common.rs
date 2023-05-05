use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::nanoserde;
use aeonetica_engine::util::vector::Vector2;

pub type Tile = u16;

pub const CHUNK_SIZE: usize = 16;

#[derive(SerBin, DeBin, Debug, Clone)]
pub struct Chunk {
    pub chunk_pos: Vector2<u32>,
    pub tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE]
}

impl Chunk {
    pub(crate) fn new(chunk_pos: Vector2<u32>) -> Self {
        Self {
            chunk_pos,
            tiles: [0; CHUNK_SIZE*CHUNK_SIZE]
        }
    }

    pub(crate) fn tiles(&self) -> &[Tile; CHUNK_SIZE * CHUNK_SIZE] {
        &self.tiles
    }
}
