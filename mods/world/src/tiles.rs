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

    pub fn glow_color(&self) -> Option<[f32; 4]> {
        match self {
            Self::Lamp => Some([0.9, 0.9, 0.7, 1.0]),
            Self::QuarteredLamp => Some([1.0, 0.5, 0.5, 1.0]),
            _ => None
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, SerBin, DeBin, PartialEq)]
pub enum FgTile {
    Empty,
    PipeEndL,
    PipeLR,
    PipeLRU,
    PipeLRD,
    PipeEndR,
    PipeEndD,
    PipeUD,
    PipeEndU,
    PipeRUD,
    PipeLUD,
    PipeLD,
    PipeRD,
    PipeLU,
    PipeRU,
    PipeLRUD,
    ChainV,
    ChainH,
    FluorecentLampL,
    FluorecentLampM,
    FluorecentLampR,
    MetalFrameBlock,
    MetalFrameFloorL,
    MetalFrameFloorM,
    MetalFrameFloorR,
    MetalFrameFloorMSupport,
    MetalFrameFloorMItemSupport,
    FramedPipeLR,
    FramedPipeUD,
    FramedPipeJunction,
    Water
}

impl FgTile {
    pub fn sprite_sheet_index(&self) -> u16 {
        *self as u16
    }

    pub fn glow_color(&self) -> Option<[f32; 4]> {
        match self {
            Self::FluorecentLampL | Self::FluorecentLampM | Self::FluorecentLampR => Some([0.8, 0.8, 1.0, 1.0]),
            _ => None
        }   
    }
}
