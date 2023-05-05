use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::nanoserde;
use aeonetica_engine::util::vector::Vector2;

pub type Tile = u16;

pub const CHUNK_SIZE: usize = 16;

#[derive(SerBin, DeBin, Debug, Clone)]
pub struct Chunk {
    pub chunk_pos: Vector2<i32>,
    pub tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE]
}

impl Chunk {
    pub(crate) fn new(chunk_pos: Vector2<i32>) -> Self {
        Self {
            chunk_pos,
            tiles: [0; CHUNK_SIZE*CHUNK_SIZE]
        }
    }

    pub(crate) fn tiles(&self) -> &[Tile; CHUNK_SIZE * CHUNK_SIZE] {
        &self.tiles
    }

    pub fn get_tile(&self, pos: Vector2<i32>) -> Tile {
        self.tiles[pos.y as usize * CHUNK_SIZE + pos.x as usize]
    }

    pub fn mut_tile(&mut self, pos: Vector2<i32>) -> &mut Tile {
        &mut self.tiles[pos.y as usize * CHUNK_SIZE + pos.x as usize]
    }

    pub fn set_tile(&mut self, pos: Vector2<i32>, tile: Tile) {
        self.tiles[pos.y as usize * CHUNK_SIZE + pos.x as usize] = tile
    }
}
