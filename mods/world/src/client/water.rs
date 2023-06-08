use aeonetica_client::{data_store::DataStore, renderer::shader::{self, UniformStr}, uniform_str};
use aeonetica_engine::{math::vector::Vector2, time::Time};

use super::light;

pub struct WaterStore {
    water_blocks: Vec<Vector2<f32>>
}

const TIME_USTR: UniformStr = uniform_str!("u_Time");

impl WaterStore {
    pub fn init(store: &mut DataStore) {
        store.add_store(WaterStore {
            water_blocks: vec![]
        });
    }

    pub fn add(&mut self, water_block: Vector2<f32>) {
        if !self.water_blocks.contains(&water_block) {
            self.water_blocks.push(water_block)
        }   
    }

    pub fn remove(&mut self, water_block: Vector2<f32>) {
        if let Some(index) = self.water_blocks.iter().position(|b| b == &water_block) {
            self.water_blocks.remove(index);
        }
    }

    pub fn upload_uniforms(&self, shader: &shader::Program, ambient_light: f32, time: &Time) {
        if self.water_blocks.len() == 0 {
            return;
        }

        shader.bind();

        shader.upload_uniform(&light::AMBIENT_LIGHT_STRENGTH_USTR, &ambient_light);
        shader.upload_uniform(&TIME_USTR, &time.time);       
   }
}