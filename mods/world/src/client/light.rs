use aeonetica_client::{data_store::DataStore, renderer::shader};
use aeonetica_engine::math::vector::*;
use crate::client::light::shader::*;

#[derive(Debug)]
pub(super) struct LightStore {
    positions: Vec<Vector2<f32>>,
    intensities: Vec<f32>,
    light_colors: Vec<Vector3<f32>>,
    ambient_light: f32,
    is_dirty: bool
}

const MAX_LIGHT_SOURCE_COUNT: usize = 20;

const LIGHT_POSITIONS_USTR: UniformStr = uniform_str!("u_LightPositions");
const AMBIENT_LIGHT_STRENGTH_USTR: UniformStr = uniform_str!("u_AmbientLightStrength");
const INTENSITIES_USTR: UniformStr = uniform_str!("u_LightIntensities");
const LIGHT_COLORS_USTR: UniformStr = uniform_str!("u_LightColors");
const NUM_LIGHTS_USTR: UniformStr = uniform_str!("u_NumLights");

impl LightStore {
    pub fn init(store: &mut DataStore) {
        store.add_store(LightStore {
            positions: vec![],
            intensities: vec![],
            light_colors: vec!{},
            ambient_light: 0.3,
            is_dirty: true
        });
    }

    pub fn add(&mut self, light: &Light) {
        if !self.positions.contains(&light.position) {
            self.positions.push(light.position);
            self.intensities.push(light.intensity);
            self.light_colors.push(light.color);
            self.is_dirty = true;
        }
    }

    pub fn remove(&mut self, light: &Light) {
        if let Some(i) = self.positions.iter().position(|p| p == &light.position) {
            self.positions.remove(i);
            self.intensities.remove(i);
            self.light_colors.remove(i);
            self.is_dirty = true;
        }
    }

    pub fn upload_uniforms(&self, shader: &shader::Program) {
        if !self.is_dirty {
            return;
        }
        
        let num_lights = self.positions.len().min(MAX_LIGHT_SOURCE_COUNT);

        shader.bind();
        shader.upload_uniform(&AMBIENT_LIGHT_STRENGTH_USTR, &self.ambient_light);
        shader.upload_uniform(&NUM_LIGHTS_USTR, &(num_lights as u32));
        shader.upload_uniform(&LIGHT_POSITIONS_USTR, &self.positions[..num_lights]);
        shader.upload_uniform(&INTENSITIES_USTR, &self.intensities[..num_lights]);
        shader.upload_uniform(&LIGHT_COLORS_USTR, &self.light_colors[..num_lights]);
    }
}

pub struct Light {
    position: Vector2<f32>,
    intensity: f32,
    color: Vector3<f32>
}

impl Light {
    pub fn new(position: Vector2<f32>, intensity: f32, color: Vector3<f32>) -> Light {
        Self {
            position,
            intensity,
            color
        }
    }
}