pub mod window;
pub mod layer;
pub mod context;
pub mod util;

mod vertex_array;
use vertex_array::*;
mod buffer;
use buffer::*;
mod shader;
use shader::*;

pub(self) use aeonetica_engine::util::camera::Camera as Camera;

pub(self) type RenderID = gl::types::GLuint;

pub struct Renderer {
    shader: Program
}

impl Renderer {
    pub fn new() -> Self {
        let shader_src = include_str!("../../assets/test_shader.glsl");
        let shader_program = Program::from_source(shader_src)
            .unwrap_or_else(|err| panic!("Error loading shader: {err}"));

        Self {
            shader: shader_program
        }
    }

    pub fn begin_scene(&self, camera: &Camera) {
        self.shader.bind();
        self.shader.upload_uniform("u_ViewProjection", camera.view_projection_matrix());
    }

    pub fn end_scene(&self) {
    }

    pub fn draw_vertex_array(&self, vao: &VertexArray) { 
        unsafe {
            gl::DrawElements(gl::TRIANGLES, vao.index_buffer().as_ref().unwrap().count() as i32, gl::UNSIGNED_INT, std::ptr::null())
        }
    }
}