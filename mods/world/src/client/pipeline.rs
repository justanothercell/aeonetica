use aeonetica_client::{renderer::{pipeline::Pipeline, Renderer, layer::LayerUpdater, buffer::{framebuffer::*, renderbuffer::RenderBuffer}, texture::Texture, util::Target, shader::{self, UniformStr}}, uniform_str};
use aeonetica_engine::{math::{camera::Camera, vector::Vector2}, error::ErrorResult};

pub(super) struct WorldRenderPipeline {
    intermediate_fb: FrameBuffer,
    shader: shader::Program
}

impl WorldRenderPipeline {
    const FB_SIZE: Vector2<u32> = Vector2::new(1920, 1080);
    const FRAME_USTR: UniformStr = uniform_str!("u_Frame");
    const FRAME_CCOL: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    pub fn new() -> ErrorResult<Self> {
        Ok(Self {
            intermediate_fb: FrameBuffer::new([
                    Attachment::Color(Texture::create(Self::FB_SIZE)),
                    Attachment::DepthStencil(RenderBuffer::new(Self::FB_SIZE)?)
                ], true)?,
            shader: shader::Program::from_source(include_str!("../../assets/world_shader.glsl"))?
        })
    }
}

impl Pipeline for WorldRenderPipeline {
    fn pipeline(&mut self, renderer: &mut Renderer, camera: &Camera, target: &Target, updater: LayerUpdater, delta_time: f64) {
        self.intermediate_fb.bind();
        self.intermediate_fb.clear(Self::FRAME_CCOL);
        renderer.begin_scene(camera);
        updater.update(renderer, delta_time);
        renderer.draw_vertices(target);
        renderer.end_scene();

        self.intermediate_fb.render(0, target, &self.shader, &Self::FRAME_USTR);
    }
}
