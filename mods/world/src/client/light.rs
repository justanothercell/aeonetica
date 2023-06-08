use std::collections::BTreeMap;

use aeonetica_client::{data_store::DataStore, renderer::shader};
use aeonetica_engine::math::vector::*;
use crate::client::light::shader::*;

pub type LightId = u32;

pub struct LightStore {
    lights: BTreeMap<LightId, Light>,
    ambient_light: f32,
    is_dirty: bool,
    light_id: u32
}

const MAX_LIGHT_SOURCE_COUNT: usize = 30;

const LIGHT_POSITIONS_USTR: UniformStr = uniform_str!("u_LightPositions");
pub(super) const AMBIENT_LIGHT_STRENGTH_USTR: UniformStr = uniform_str!("u_AmbientLightStrength");
const INTENSITIES_USTR: UniformStr = uniform_str!("u_LightIntensities");
const LIGHT_COLORS_USTR: UniformStr = uniform_str!("u_LightColors");
const NUM_LIGHTS_USTR: UniformStr = uniform_str!("u_NumLights");

impl LightStore {
    pub fn init(store: &mut DataStore) {
        store.add_store(LightStore {
            lights: BTreeMap::new(),
            ambient_light: 0.1,
            is_dirty: true,
            light_id: 0
        });
    }

    pub fn add(&mut self, light: Light) -> LightId {
        let id = self.light_id;
        self.light_id += 1;

        self.lights.insert(id, light);
        self.is_dirty = true;

        id
    }

    pub fn remove(&mut self, light: &LightId) {
        self.lights.remove(light);
        self.is_dirty = true;
    }

    pub fn update(&mut self, id: &LightId, light: Light) {
        *self.lights.get_mut(id).unwrap() = light;
    }

    pub fn ambient_light(&self) -> f32 {
        self.ambient_light
    }

    pub fn set_ambient_light(&mut self, ambient_light: f32) {
        self.ambient_light = ambient_light
    }

    pub fn upload_uniforms(&self, shader: &shader::Program) {
        if !self.is_dirty {
            return;
        }
        
        shader.bind();

        let light_positions_location = shader.uniform_location(&LIGHT_POSITIONS_USTR);
        let light_intensities_location = shader.uniform_location(&INTENSITIES_USTR);
        let light_colors_location = shader.uniform_location(&LIGHT_COLORS_USTR);
        
        shader.upload_uniform(&NUM_LIGHTS_USTR, &(self.lights.len().min(MAX_LIGHT_SOURCE_COUNT) as u32));
        shader.upload_uniform(&AMBIENT_LIGHT_STRENGTH_USTR, &self.ambient_light);

        for (i, light) in self.lights.values().enumerate() {
            if i >= MAX_LIGHT_SOURCE_COUNT {
                break;
            }

            light.position.upload(light_positions_location + i as i32);
            light.intensity.upload(light_intensities_location + i as i32);
            light.color.upload(light_colors_location + i as i32);
        }
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