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

#[macro_export]
macro_rules! to_raw_byte_slice {
    ($value:expr) => {
        unsafe { std::slice::from_raw_parts($value.as_ptr().cast(), std::mem::size_of_val(&$value)) }
    };
}

pub(self) type RenderID = gl::types::GLuint;

pub(self) struct Renderer {
    vao: VertexArray
}

impl Renderer {
    pub fn new() -> Self {
        let mut vao = VertexArray::new().expect("Error creating vertex array");
        vao.bind();

        type Vertex = [f32; 3];
        type Color = [f32; 3];

        const VERTICES: [(Vertex, Color); 4] = [
            ([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
            ([0.5,  -0.5, 0.0], [0.0, 1.0, 0.0]),
            ([0.5,  0.5,  0.0], [0.0, 0.0, 1.0]),
            ([-0.5, 0.5,  0.0], [0.0, 0.0, 0.0])
        ];

        let layout = BufferLayout::new(vec![
            BufferElement::new(ShaderDataType::Float3), // position
            BufferElement::new(ShaderDataType::Float3), // color
        ]);
        
        let vbo = Buffer::new(BufferType::Array, to_raw_byte_slice!(VERTICES), Some(layout))
            .expect("Error creating Vertex Buffer");
        vao.add_vertex_buffer(vbo);

        const INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];
        let ibo = Buffer::new(BufferType::ElementArray, to_raw_byte_slice!(INDICES), None)
            .expect("Error creating Index Buffer");
      
        vao.set_index_buffer(ibo);
        let shader_src = include_str!("../../assets/test_shader.glsl");
        let shader_program = Program::from_source(shader_src)
            .unwrap_or_else(|err| panic!("Error loading shader: {err}"));
        shader_program.use_program();

        Self {
            vao
        }
    }

    pub unsafe fn render(&self) { 
        gl::DrawElements(gl::TRIANGLES, self.vao.index_buffer().as_ref().unwrap().count() as i32, gl::UNSIGNED_INT, std::ptr::null())
    }
}