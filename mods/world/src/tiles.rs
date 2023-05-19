use aeonetica_engine::nanoserde::{self, DeBin, SerBin};

#[repr(u16)]
#[derive(Debug, Copy, Clone, SerBin, DeBin)]
pub enum Tile {
    Air,
    StoneBrick,
    Stone,
}

impl Tile {
    pub fn sprite_sheet_index(&self) -> u16 {
        *self as u16
    }
}