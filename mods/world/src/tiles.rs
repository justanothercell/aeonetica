use aeonetica_engine::nanoserde::{self, DeBin, SerBin};

#[repr(u16)]
#[derive(Debug, Copy, Clone, SerBin, DeBin, PartialEq)]
pub enum Tile {
    Wall,
    StoneBrick,
    MossyStoneBrick,
    Stone,
    HardStone,
    Lamp,
    QuarteredLamp,
    LabWall,
    LabBrickWall
}

impl Tile {
    pub fn sprite_sheet_index(&self) -> u16 {
        *self as u16
    }

    pub fn is_solid(&self) -> bool {
        (*self as u16) == 0
    }

    pub fn is_natural(&self) -> bool {
        matches!(self, Tile::Wall | Tile::StoneBrick | Tile::Stone | Tile::HardStone)
    }
}