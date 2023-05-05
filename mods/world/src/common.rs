use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::nanoserde;

pub const CHUNK_SIZE: usize = 16;

#[derive(SerBin, DeBin)]
pub struct Chunk {
    pub tiles: [u16; CHUNK_SIZE*CHUNK_SIZE]
}