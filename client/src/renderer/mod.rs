pub mod window;
pub mod layer;
pub mod context;
pub mod util;

mod vertex_array;
use std::rc::Rc;

use vertex_array::*;
mod buffer;
use buffer::*;
mod shader;
use shader::*;
mod texture;
use texture::*;

pub(self) use aeonetica_engine::util::camera::Camera as Camera;

pub(self) type RenderID = gl::types::GLuint;

pub struct Renderer {
    shader: Option<Rc<Program>>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            shader: None
        }
    }

    pub fn begin_scene(&self, camera: &Camera) {
        if let Some(shader) = &self.shader {
            shader.bind();
            shader.upload_uniform("u_ViewProjection", camera.view_projection_matrix());
        }
        else {
            panic!("no shader was loaded")
        }
    }

    pub fn load_shader(&mut self, shader: Rc<Program>) {
        shader.bind();
        self.shader = Some(shader);
    }

    pub fn unload_shader(&mut self) {
        if let Some(shader) = &self.shader {
            shader.unbind();
        }
        self.shader = None;
    }

    pub fn shader(&self) -> &Option<Rc<Program>> {
        &self.shader
    }

    pub fn end_scene(&self) {
    }

    pub fn draw_vertex_array(&self, vao: &VertexArray) { 
        unsafe {
            gl::DrawElements(gl::TRIANGLES, vao.index_buffer().as_ref().unwrap().count() as i32, gl::UNSIGNED_INT, std::ptr::null())
        }
    }
}