use std::collections::hash_map::DefaultHasher;
use std::hash::{SipHasher, Hasher};

use aeonetica_engine::log;
use noise::{Fbm, NoiseFn, OpenSimplex, RidgedMulti, Terrace, Worley};
use aeonetica_engine::math::vector::Vector2;
use rand::{SeedableRng, Rng};
use crate::common::{CHUNK_SIZE, Population, Chunk, WorldView};
use crate::server::world::World;
use crate::tiles::Tile;

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
        let pos = chunk_pos * 16;
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.chunk_hash_with_seed_and_salt(pos, 0));
        if rng.gen_ratio(1, 10) {
            for x in 3..(CHUNK_SIZE-3) as i32 {
                for y in 3..(CHUNK_SIZE-3) as i32 {
                    self.set_init_tile_at(pos + Vector2::new(x, y), Population::TerrainPostProcess, 
                    if (x == 3) as u8 + (y == 3) as u8 + (x == CHUNK_SIZE as i32 - 4) as u8 + (y == CHUNK_SIZE as i32 - 4) as u8 > 1 { Tile::Lamp } else { Tile::LabWall })
                }
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