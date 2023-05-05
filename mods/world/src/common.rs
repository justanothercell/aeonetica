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
        let mut s = Self {
            chunk_pos,
            tiles: [0; CHUNK_SIZE*CHUNK_SIZE]
        };
        for i in 0..CHUNK_SIZE {
            if i != 7 && i != 8 {
                *s.mut_tile((0, i as i32).into()) = 1;
                *s.mut_tile((i as i32, 0).into()) = 1;
            }
        }
        s
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
