use aeonetica_client::{renderer::{pipeline::Pipeline, Renderer, layer::LayerUpdater, buffer::framebuffer::*, texture::*, util::*, shader::{self, UniformStr}}, uniform_str, data_store::DataStore};
use aeonetica_engine::{time::Time, math::{camera::Camera, vector::Vector2}, error::ErrorResult};

use super::{light::LightStore, materials::terrain_shader};

pub(super) struct WorldRenderPipeline {
    intermediate_fb: FrameBuffer,
    shader: shader::Program
}

impl WorldRenderPipeline {
    const FB_SIZE: Vector2<u32> = Vector2::new(1920, 1080);
    const FRAME_CCOL: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    const FRAME_USTR: UniformStr = uniform_str!("u_Frame");
    const LIGHTMAP_USTR: UniformStr = uniform_str!("u_LightMap");

    pub fn new(store: &mut DataStore) -> ErrorResult<Self> {
        LightStore::init(store);
        
        scissor(Vector2::new(0, 0), Vector2::new(1920, 1080));

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

        enable_scissor_test();

        let shader = terrain_shader(updater.store());
        let lights = updater.store().mut_store::<LightStore>();
        lights.upload_uniforms(&shader);

        updater.update(renderer, time);
        renderer.draw_vertices(target);
        renderer.end_scene();
        
        disable_scissor_test();
        
        self.intermediate_fb.render([
                (0, &Self::FRAME_USTR),
                (1, &Self::LIGHTMAP_USTR),
            ],
            target, &self.shader
        );

    }
}
