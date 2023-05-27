use aeonetica_engine::nanoserde::{self, DeBin, SerBin};

#[repr(u16)]
#[derive(Debug, Copy, Clone, SerBin, DeBin)]
pub enum Tile {
    Wall,
    FloorStoneBrick,
    FloorStone,
    FloorHardStone
}

impl Tile {
    pub fn sprite_sheet_index(&self) -> u16 {
        *self as u16
    }

    pub fn is_solid(&self) -> bool {
        (*self as u16) == 0
    }
}