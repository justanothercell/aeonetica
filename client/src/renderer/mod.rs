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
        
        let vertex_buffer = Buffer::new(BufferType::Array, util::to_raw_byte_slice!(VERTICES), Some(layout))
            .expect("Error creating Vertex Buffer");
        vao.add_vertex_buffer(vertex_buffer);

        const INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];
        let index_buffer = Buffer::new(BufferType::ElementArray, util::to_raw_byte_slice!(INDICES), None)
            .expect("Error creating Index Buffer");
        vao.set_index_buffer(index_buffer);

        let shader_src = include_str!("../../assets/test_shader.glsl");
        let shader_program = Program::from_source(shader_src)
            .unwrap_or_else(|err| panic!("Error loading shader: {err}"));
        shader_program.use_program();

        Self {
            vao
        }
    }

    pub fn render(&self) { 
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.vao.index_buffer().as_ref().unwrap().count() as i32, gl::UNSIGNED_INT, std::ptr::null())
        }
    }
}