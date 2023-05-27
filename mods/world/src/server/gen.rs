use noise::{Fbm, NoiseFn, OpenSimplex, RidgedMulti, Terrace, Worley};
use aeonetica_engine::log;
use aeonetica_engine::math::vector::Vector2;
use crate::common::{CHUNK_SIZE, Population};
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
    pub(crate) fn populate(&mut self, chunk_pos: Vector2<i32>) {
        log!(WARN, "populated {chunk_pos}");
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
                        Tile::FloorHardStone
                    } else {
                        Tile::FloorStoneBrick
                    })
                }
                else if (current && around > 1) || around > 2 {
                    chunk.set_tile((x, y).into(), if accent > 1 {
                        Tile::FloorStone
                    } else {
                        Tile::FloorStoneBrick
                    })
                }
            }
        }

        // TEMPORARY:
        chunk.set_tile(Vector2::default(), Tile::Lamp);
        
        chunk.population = Population::Finished;
    }
}