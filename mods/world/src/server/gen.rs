use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use noise::{Fbm, NoiseFn, OpenSimplex, RidgedMulti, Terrace, Worley};
use aeonetica_engine::math::vector::Vector2;
use rand::{SeedableRng, Rng};
use crate::common::{CHUNK_SIZE, Population, Chunk, WorldView};
use crate::server::world::World;
use crate::tiles::{Tile, FgTile};

pub(crate) struct GenProvider {
    pub(crate) seed: u64,
    pub(crate) cave_noise: Box<dyn NoiseFn<f64, 2>>,
    pub(crate) space_cave_noise: Box<dyn NoiseFn<f64, 2>>,
}

impl GenProvider {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            seed,
            cave_noise: {
                let fbm = Fbm::<RidgedMulti<OpenSimplex>>::new(seed as u32);
                Box::new(Terrace::new(fbm)
                    .add_control_point(0.0)
                    .add_control_point(0.1))
            },
            space_cave_noise: Box::new(Worley::new(seed as u32 + 1))
        }
    }
}

impl World {
    pub(crate) fn mut_init_chunk_at(&mut self, chunk_pos: Vector2<i32>, stage: Population) -> &mut Chunk{
        let mut p = self.mut_chunk_at_raw(chunk_pos).population;
        while (p as u8) < stage as u8 {
            match p {
                Population::Uninit => self.populate_terrain(chunk_pos),
                Population::TerrainRaw => self.post_process_terrain(chunk_pos),
                Population::TerrainPostProcess => self.structurize_chunk(chunk_pos),
                Population::Structures => self.mut_chunk_at_raw(chunk_pos).population = Population::Finished,
                Population::Finished => unreachable!("finished should always be last population stage"),
            }
            p = self.mut_chunk_at_raw(chunk_pos).population;
        }
        self.mut_chunk_at_raw(chunk_pos)
    }

    pub fn get_init_tile_at(&mut self, pos: Vector2<i32>, stage: Population) -> Tile {
        self.mut_init_chunk_at(World::chunk(pos), stage).get_tile(World::pos_in_chunk(pos))
    }

    pub fn set_init_tile_at(&mut self, pos: Vector2<i32>, stage: Population, t: Tile) {
        self.mut_init_chunk_at(World::chunk(pos), stage).set_tile(World::pos_in_chunk(pos), t)
    }

    pub fn get_init_fg_tile_at(&mut self, pos: Vector2<i32>, stage: Population) -> FgTile {
        self.mut_init_chunk_at(World::chunk(pos), stage).get_fg_tile(World::pos_in_chunk(pos))
    }

    pub fn set_init_fg_tile_at(&mut self, pos: Vector2<i32>, stage: Population, t: FgTile) {
        self.mut_init_chunk_at(World::chunk(pos), stage).set_fg_tile(World::pos_in_chunk(pos), t)
    }

    fn populate_terrain(&mut self, chunk_pos: Vector2<i32>) {
        let gen = self.generator.clone();
        let chunk = self.mut_chunk_at_raw(chunk_pos);
        let scale = 0.75;
        let scale2 = 1.6;
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_SIZE as i32 {
                let p = Vector2::new(x, y).to_f64() / 16.0 * scale + chunk_pos.to_f64() * scale;
                let ps2 = Vector2::new(x, y).to_f64() / 16.0 * scale2 + chunk_pos.to_f64() * scale2;
                let accent_2 = gen.space_cave_noise.get(ps2.into_array()) < -0.865;
                let current = gen.cave_noise.get(p.into_array()) > 0.0 || accent_2;
                let around =
                    (gen.cave_noise.get((p + Vector2::new(1.0/16.0 * scale, 0.0/16.0 * scale)).into_array()) > 0.0) as i32 +
                    (gen.cave_noise.get((p + Vector2::new(-1.0/16.0 * scale, 0.0/16.0 * scale)).into_array()) > 0.0) as i32 +
                    (gen.cave_noise.get((p + Vector2::new(0.0/16.0 * scale, 1.0/16.0 * scale)).into_array()) > 0.0) as i32 +
                    (gen.cave_noise.get((p + Vector2::new(0.0/16.0 * scale, -1.0/16.0 * scale)).into_array()) > 0.0) as i32;
                // a bit of a random approach - found accidentally
                let accent =
                    (gen.cave_noise.get((p + Vector2::new(scale, 0.0)).into_array()) > 0.0) as i32 +
                        (gen.cave_noise.get((p + Vector2::new(-scale, 0.0)).into_array()) > 0.0) as i32 +
                        (gen.cave_noise.get((p + Vector2::new(0.0, scale)).into_array()) > 0.0) as i32 +
                        (gen.cave_noise.get((p + Vector2::new(0.0, -scale)).into_array()) > 0.0) as i32;
                if accent_2 {
                    chunk.set_tile((x, y).into(), if accent > 1 {
                        Tile::HardStone
                    } else {
                        Tile::StoneBrick
                    })
                }
                else if (current && around > 1) || around > 2 {
                    chunk.set_tile((x, y).into(), if accent > 1 {
                        Tile::Stone
                    } else {
                        Tile::StoneBrick
                    })
                }
            }
        }

        // TEMPORARY:
        chunk.set_tile(Vector2::default(), Tile::Lamp);
        
        chunk.population = Population::TerrainRaw;
    }

    fn post_process_terrain(&mut self, chunk_pos: Vector2<i32>) {
        let pos = chunk_pos * 16;
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_SIZE as i32 {
                if self.get_init_tile_at(pos + Vector2::new(x, y), Population::TerrainRaw) != Tile::Wall {
                    let s = (self.get_init_tile_at(pos + Vector2::new(x + 1, y + 0), Population::TerrainRaw) == Tile::Wall) as u8
                              + (self.get_init_tile_at(pos + Vector2::new(x - 1, y + 0), Population::TerrainRaw) == Tile::Wall) as u8
                              + (self.get_init_tile_at(pos + Vector2::new(x + 0, y + 1), Population::TerrainRaw) == Tile::Wall) as u8
                              + (self.get_init_tile_at(pos + Vector2::new(x + 0, y - 1), Population::TerrainRaw) == Tile::Wall) as u8;
                    if s == 4 {
                        self.set_init_tile_at(pos + Vector2::new(x, y), Population::TerrainRaw, Tile::Wall)
                    }
                } else {
                    let s = (self.get_init_tile_at(pos + Vector2::new(x + 1, y + 0), Population::TerrainRaw) != Tile::Wall) as u8
                              + (self.get_init_tile_at(pos + Vector2::new(x - 1, y + 0), Population::TerrainRaw) != Tile::Wall) as u8
                              + (self.get_init_tile_at(pos + Vector2::new(x + 0, y + 1), Population::TerrainRaw) != Tile::Wall) as u8
                              + (self.get_init_tile_at(pos + Vector2::new(x + 0, y - 1), Population::TerrainRaw) != Tile::Wall) as u8;
                    if s == 4 {
                        self.set_init_tile_at(pos + Vector2::new(x, y), Population::TerrainRaw, Tile::StoneBrick)
                    }
                }
            }
        }
        self.mut_chunk_at_raw(chunk_pos).population = Population::TerrainPostProcess;
    }

    fn structurize_chunk(&mut self, chunk_pos: Vector2<i32>) {
        self.mut_chunk_at_raw(chunk_pos).population = Population::Structures;
        if chunk_pos.mag_sq() <= 2 { return }
        let mut pos = chunk_pos * 16;
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.chunk_hash_with_seed_and_salt(pos, 0));
        let both = rng.gen_ratio(1, 12);
        // pipes
        if rng.gen_ratio(1, 12) || both {
            pos += Vector2::new(rng.gen_range(0..CHUNK_SIZE as i32), rng.gen_range(0..CHUNK_SIZE as i32));
            let mut pipes = HashSet::new();
            fn gen_pipe(rng: &mut rand::rngs::StdRng, pipes: &mut HashSet<Vector2<i32>>, world: &mut World, mut pos: Vector2<i32>, dir: Vector2<i32>, len: i32) {
                let len = len - rng.gen_range(1..4);
                if len <= 0 { return }
                if dir.mag_sq() > 0 {
                    for _ in 0..len {
                        pos += dir;
                        if world.get_init_tile_at(pos, Population::TerrainPostProcess) == Tile::Wall || 
                            world.get_init_fg_tile_at(pos, Population::TerrainPostProcess) != FgTile::Empty || 
                            !pipes.insert(pos) { return; }
                    }
                }

                if rng.gen_ratio(4, 5) { gen_pipe(rng, pipes, world, pos, Vector2::new(1, 0), len); }
                if rng.gen_ratio(4, 5) { gen_pipe(rng, pipes, world, pos, Vector2::new(-1, 0), len); }
                if rng.gen_ratio(4, 5) { gen_pipe(rng, pipes, world, pos, Vector2::new(0, 1), len); }
                if rng.gen_ratio(4, 5) { gen_pipe(rng, pipes, world, pos, Vector2::new(0, -1), len); }
            }
            pipes.insert(pos);
            gen_pipe(&mut rng, &mut pipes, self, pos, Vector2::new(0, 0), 16);

            for pipe in &pipes {
                let (wl, wr, wu, wd) = (
                    self.get_init_tile_at(*pipe + Vector2::new(-1, 0), Population::TerrainPostProcess) == Tile::Wall,
                    self.get_init_tile_at(*pipe + Vector2::new(1, 0), Population::TerrainPostProcess) == Tile::Wall,
                    self.get_init_tile_at(*pipe + Vector2::new(0, -1), Population::TerrainPostProcess) == Tile::Wall,
                    self.get_init_tile_at(*pipe + Vector2::new( 0, 1), Population::TerrainPostProcess) == Tile::Wall
                );
                self.set_init_fg_tile_at(*pipe, Population::TerrainPostProcess, 
                match (
                    pipes.contains(&(*pipe + Vector2::new(-1, 0))) || wl, 
                    pipes.contains(&(*pipe + Vector2::new(1, 0))) || wr, 
                    pipes.contains(&(*pipe + Vector2::new(0, -1))) || wu, 
                    pipes.contains(&(*pipe + Vector2::new(0, 1))) || wd
                ) {
                    (_, _, _, _) if wl && wr && wu && wd => FgTile::Empty,
                    (true, true, true, true) => FgTile::PipeLRUD,

                    (true, true, true, false) if !wu => FgTile::PipeLRU,
                    (true, true, true, false) => FgTile::PipeLR,
                    (true, true, false, true) if !wd => FgTile::PipeLRD,
                    (true, true, false, true) => FgTile::PipeLR,
                    (true, false, true, true) if !wl => FgTile::PipeLUD,
                    (true, false, true, true) => FgTile::PipeUD,
                    (false, true, true, true) if !wr => FgTile::PipeRUD,
                    (false, true, true, true) => FgTile::PipeUD,

                    (true, true, false, false) => FgTile::PipeLR,
                    (false, false, true, true) => FgTile::PipeUD,

                    (true, false, true, false) => FgTile::PipeLU,
                    (true, false, false, true) => FgTile::PipeLD,
                    (false, true, true, false) => FgTile::PipeRU,
                    (false, true, false, true) => FgTile::PipeRD,

                    (true, false, false, false) => FgTile::PipeEndR,
                    (false, true, false, false) => FgTile::PipeEndL,
                    (false, false, true, false) => FgTile::PipeEndD,
                    (false, false, false, true) => FgTile::PipeEndU,

                    (false, false, false, false) => FgTile::Empty
                });
            }
        }
        if rng.gen_ratio(1, 12) || both {
            pos += Vector2::new(rng.gen_range(0..CHUNK_SIZE as i32), rng.gen_range(0..CHUNK_SIZE as i32));
            let mut platforms = HashSet::new();
            fn gen_support(rng: &mut rand::rngs::StdRng, platforms: &mut HashSet<Vector2<i32>>, world: &mut World, pos: Vector2<i32>, is_up_chain: bool) {
                let mut next_layer = rng.gen_range(4..12);
                for i in 1.. {
                    if is_up_chain {
                        let mut pos = pos;
                        pos.y -= i;
                        let current = world.get_init_fg_tile_at(pos, Population::TerrainPostProcess);
                        use FgTile::*;
                        if world.get_init_tile_at(pos, Population::TerrainPostProcess) == Tile::Wall || 
                            matches!(current, 
                            FramedPipeJunction | FramedPipeLR | FramedPipeUD | MetalFrameBlock | MetalFrameFloorL | MetalFrameFloorM | MetalFrameFloorR | MetalFrameFloorMSupport | MetalFrameFloorMItemSupport ) { 
                                break; 
                            } else {
                                world.set_init_fg_tile_at(pos, Population::TerrainPostProcess, match current {
                                    PipeEndL | PipeLR | PipeEndR => FgTile::FramedPipeLR,
                                    PipeEndU | PipeUD | PipeEndD => FgTile::FramedPipeUD,
                                    PipeLRU | PipeLRD | PipeRUD | PipeLUD | PipeLD | PipeRD | PipeLU | PipeRU | PipeLRUD => FgTile::FramedPipeJunction,
                                    _ => FgTile::ChainV
                                });
                            }
                    } else {
                        let mut pos = pos;
                        pos.y += i;
                        use FgTile::*;
                        if world.get_init_tile_at(pos, Population::TerrainPostProcess) == Tile::Wall || 
                            matches!(world.get_init_fg_tile_at(pos, Population::TerrainPostProcess), 
                            FramedPipeJunction | FramedPipeLR | FramedPipeUD | MetalFrameBlock | MetalFrameFloorL | MetalFrameFloorM | MetalFrameFloorR | MetalFrameFloorMSupport | MetalFrameFloorMItemSupport ) || 
                            !platforms.insert(pos) { break; }
                        next_layer -= 1;
                        if next_layer <= 0 {
                            next_layer = rng.gen_range(4..12);
                            gen_platform(rng, platforms, world, pos);
                        }
                    }
                }
                return;
            }
            fn gen_platform(rng: &mut rand::rngs::StdRng, platforms: &mut HashSet<Vector2<i32>>, world: &mut World, pos: Vector2<i32>) {
                let mut next_support = rng.gen_range(2..8);
                let mut next_chain = rng.gen_range(2..8);
                for i in 0..rng.gen_range(12..24) {
                    let mut pos = pos;
                    pos.x += i;
                    use FgTile::*;
                    if world.get_init_tile_at(pos, Population::TerrainPostProcess) == Tile::Wall || 
                        matches!(world.get_init_fg_tile_at(pos, Population::TerrainPostProcess), FramedPipeJunction | FramedPipeLR | FramedPipeUD | MetalFrameBlock) || 
                        !platforms.insert(pos) { break; }
                    next_support -= 1;
                    if next_support <= 0 {
                        next_support = rng.gen_range(2..8);
                        gen_support(rng, platforms, world, pos, false);
                    }
                    next_chain -= 1;
                    if next_chain <= 0 {
                        next_chain = rng.gen_range(2..8);
                        gen_support(rng, platforms, world, pos, true);
                    }
                }
                next_support = rng.gen_range(2..8);
                next_chain = rng.gen_range(2..8);
                for i in 1..rng.gen_range(12..24) {
                    let mut pos = pos;
                    pos.x -= i;
                    use FgTile::*;
                    if world.get_init_tile_at(pos, Population::TerrainPostProcess) == Tile::Wall || 
                        matches!(world.get_init_fg_tile_at(pos, Population::TerrainPostProcess), FramedPipeJunction | FramedPipeLR | FramedPipeUD | MetalFrameBlock) || 
                        !platforms.insert(pos) { break; }
                    next_support -= 1;
                    if next_support <= 0 {
                        next_support = rng.gen_range(2..8);
                        gen_support(rng, platforms, world, pos, false);
                    }
                    next_chain -= 1;
                    if next_chain <= 0 {
                        next_chain = rng.gen_range(2..8);
                        gen_support(rng, platforms, world, pos, true);
                    }
                }
            }
            gen_platform(&mut rng, &mut platforms, self, pos);

            for platform in &platforms {
                if matches!(self.get_init_fg_tile_at(*platform + Vector2::new(1, 0), Population::TerrainPostProcess), 
                FgTile::FluorecentLampL | FgTile::FluorecentLampR | FgTile::ChainV) {
                    self.set_init_fg_tile_at(*platform, Population::TerrainPostProcess, FgTile::MetalFrameFloorMItemSupport);
                    continue;
                }
                let (wl, wr, wu, wd) = (
                    self.get_init_tile_at(*platform + Vector2::new(-1, 0), Population::TerrainPostProcess) == Tile::Wall,
                    self.get_init_tile_at(*platform + Vector2::new(1, 0), Population::TerrainPostProcess) == Tile::Wall,
                    self.get_init_tile_at(*platform + Vector2::new(0, -1), Population::TerrainPostProcess) == Tile::Wall,
                    self.get_init_tile_at(*platform + Vector2::new( 0, 1), Population::TerrainPostProcess) == Tile::Wall
                );
                let t = 
                match (
                    platforms.contains(&(*platform + Vector2::new(-1, 0))) || wl, 
                    platforms.contains(&(*platform + Vector2::new(1, 0))) || wr, 
                    platforms.contains(&(*platform + Vector2::new(0, -1))) || wu, 
                    platforms.contains(&(*platform + Vector2::new(0, 1))) || wd
                ) {
                    (true, true, _, false) if !wu => FgTile::MetalFrameFloorM,
                    (true, false, _, false) if !wu => FgTile::MetalFrameFloorR,
                    (false, true, _, false) if !wu => FgTile::MetalFrameFloorL,
                    (true, true, _, true) => FgTile::MetalFrameFloorMSupport,

                    (_, _, _, _) => FgTile::MetalFrameBlock,
                };
                let current = self.get_init_fg_tile_at(*platform, Population::TerrainPostProcess);
                use FgTile::*;
                let t = match current {
                    PipeEndL | PipeLR | PipeEndR => FgTile::FramedPipeLR,
                    PipeEndU | PipeUD | PipeEndD => FgTile::FramedPipeUD,
                    PipeLRU | PipeLRD | PipeRUD | PipeLUD | PipeLD | PipeRD | PipeLU | PipeRU | PipeLRUD => FgTile::FramedPipeJunction,
                    _ => t
                };
                self.set_init_fg_tile_at(*platform, Population::TerrainPostProcess, t);
            }
        }
    }

    fn chunk_hash_with_seed_and_salt(&self, pos: Vector2<i32>, salt: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(self.generator.seed);
        hasher.write_i32(pos.x);
        hasher.write_i32(pos.y);
        hasher.write_u64(salt);
        hasher.finish()
    }
}