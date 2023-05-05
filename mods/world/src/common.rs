use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::nanoserde;

pub type Tile = u16;

pub const CHUNK_SIZE: usize = 16;

#[derive(SerBin, DeBin)]
pub struct Chunk {
    pub tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE]
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Self {
            tiles: [0; CHUNK_SIZE*CHUNK_SIZE]
        }
    }
}