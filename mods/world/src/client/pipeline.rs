use aeonetica_client::{renderer::{pipeline::Pipeline, Renderer, layer::LayerUpdater, buffer::framebuffer::*, texture::*, util::Target, shader::{self, UniformStr}}, uniform_str, data_store::DataStore};
use aeonetica_engine::{time::Time, math::{camera::Camera, vector::Vector2}, error::ErrorResult};

use super::materials;

#[derive(Debug)]
pub(super) struct LightPositions(Vec<Vector2<f32>>);

impl LightPositions {
    pub(super) fn add(&mut self, position: Vector2<f32>) {
        if !self.0.contains(&position) {
            self.0.push(position);
        }
    }

    pub(super) fn remove(&mut self, position: &Vector2<f32>) {
        if let Some(i) = self.0.iter().position(|p| p == position) {
            self.0.remove(i);
        }
    }
}

const MAX_LIGHT_SOURCE_COUNT: usize = 20;

pub(super) struct WorldRenderPipeline {
    intermediate_fb: FrameBuffer,
    shader: shader::Program
}

impl WorldRenderPipeline {
    const FB_SIZE: Vector2<u32> = Vector2::new(1920, 1080);
    const FRAME_CCOL: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    const FRAME_USTR: UniformStr = uniform_str!("u_Frame");
    const LIGHTMAP_USTR: UniformStr = uniform_str!("u_LightMap");

    const LIGHT_POSITIONS_USTR: UniformStr = uniform_str!("u_LightPosition");
    const AMBIENT_LIGHT_STRENGHT_USTR: UniformStr = uniform_str!("u_AmbientLightStrength");
    const NUM_LIGHTS_USTR: UniformStr = uniform_str!("u_NumLights");

    pub fn new(store: &mut DataStore) -> ErrorResult<Self> {
        store.add_store(LightPositions(vec![]));
        Ok(Self {
            intermediate_fb: FrameBuffer::new([
                    Attachment::Color(Texture::create(Self::FB_SIZE, Format::RgbaF16)), // main scene colors
                    Attachment::Color(Texture::create(Self::FB_SIZE, Format::RgbaF16)), // light map
                ], true)?,
            shader: shader::Program::from_source(include_str!("../../assets/world-shader.glsl"))?
        })
    }
}

impl Pipeline for WorldRenderPipeline {
    fn pipeline(&mut self, renderer: &mut Renderer, camera: &Camera, target: &Target, mut updater: LayerUpdater, time: Time) {
        self.intermediate_fb.bind();
        self.intermediate_fb.clear(Self::FRAME_CCOL);
        renderer.begin_scene(camera);

        let light_positions = updater.store().mut_store::<LightPositions>();

        let terrain_shader = materials::terrain_shader();
        

        let slice = if light_positions.0.len() > MAX_LIGHT_SOURCE_COUNT { &light_positions.0[..MAX_LIGHT_SOURCE_COUNT] } else { light_positions.0.as_slice() };
        terrain_shader.bind();
        terrain_shader.upload_uniform(&Self::NUM_LIGHTS_USTR, &(slice.len() as u32));
        terrain_shader.upload_uniform(&Self::LIGHT_POSITIONS_USTR, slice);
        terrain_shader.upload_uniform(&Self::AMBIENT_LIGHT_STRENGHT_USTR, &0.3f32);

        updater.update(renderer, time);
        renderer.draw_vertices(target);
        renderer.end_scene();

        self.intermediate_fb.render([
                (0, &Self::FRAME_USTR),
                (1, &Self::LIGHTMAP_USTR),
            ],
            target, &self.shader
        );
    }
}
