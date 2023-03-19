pub mod window;
pub mod layer;
pub mod context;

use std::mem;

mod vertex_array;
use vertex_array::*;
mod buffer;
use buffer::*;
mod shader;
use shader::*;

macro_rules! to_raw_byte_slice {
    ($value:expr) => {
        std::slice::from_raw_parts(VERTICES.as_ptr().cast(), mem::size_of_val(&VERTICES))
    };
}

extern crate gl;

pub(self) type RenderID = gl::types::GLuint;

pub(self) struct Renderer {

}

impl Renderer {
    pub fn new() -> Self {
        unsafe {
            let mut vao = VertexArray::new().expect("Error creating vertex array");
            vao.bind();

            type Vertex = [f32; 3];
            const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

            let vbo = Buffer::new().expect("Error creating VBO");
            vbo.bind(BufferType::Array);
            Buffer::data(
                BufferType::Array,
                to_raw_byte_slice!(VERTICES),
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                0 as *const _
            );

            gl::EnableVertexAttribArray(0);

            let shader_src = include_str!("../../assets/test_shader.glsl");

            let shader_program = Program::from_source(shader_src)
                .unwrap_or_else(|err| panic!("Error loading shader: {err}"));
            shader_program.use_program();
        }

        Self {}
    }

    pub unsafe fn render(&self) { 
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}