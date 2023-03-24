pub mod window;
pub mod layer;
pub mod context;
pub mod util;

mod vertex_array;
use std::rc::Rc;

use aeonetica_engine::util::{vector::Vector2, Either, matrix::Matrix4};
use vertex_array::*;
mod buffer;
use buffer::*;
mod shader;
use shader::*;
mod texture;
use texture::*;

pub(self) use aeonetica_engine::util::camera::Camera;

pub(self) type RenderID = gl::types::GLuint;

pub struct Renderer {
    shader: Option<Rc<Program>>,
    quad_vertex_array: VertexArray,
    white_texture: Texture,
}

impl Renderer {
    pub fn new() -> Self {
        let white_texture = Texture::create(1, 1)
            .expect("error creating white texture");
        white_texture.set_data(&[0xff, 0xff, 0xff, 0xff]);

        let mut quad_vertex_array = VertexArray::new()
            .expect("error creating quad vertex array");

        let layout = Vertices::build();
        type Vertices = BufferLayoutBuilder<(Vertex, TexCoord)>;
        let vertices = Vertices::array([
            ([-0.5, -0.5, 0.0], [0.0, 0.0]),
            ([ 0.5, -0.5, 0.0], [1.0, 0.0]),
            ([ 0.5,  0.5, 0.0], [1.0, 1.0]),
            ([-0.5,  0.5, 0.0], [0.0, 1.0])
        ]);
        
        let vertex_buffer = Buffer::new(BufferType::Array, util::to_raw_byte_slice!(vertices), Some(layout))
            .expect("Error creating Vertex Buffer");
        quad_vertex_array.add_vertex_buffer(vertex_buffer);

        let indices = [ 0, 1, 2, 2, 3, 0 ];
        let index_buffer = Buffer::new(BufferType::ElementArray, util::to_raw_byte_slice!(indices), None)
            .expect("Error creating Index Buffer");
        quad_vertex_array.set_index_buffer(index_buffer);
        
        let shader_src = include_str!("../../assets/test_shader.glsl");
        let default_shader = Rc::new(Program::from_source(shader_src)
            .unwrap_or_else(|err| panic!("Error loading shader: {err}")));
        default_shader.bind();

        Self {
            shader: Some(default_shader.clone()),
            white_texture,
            quad_vertex_array,
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

    pub fn draw_quad(&self, position: &Vector2<f32>, size: &Vector2<f32>, material: Either<&Texture, &(f32, f32, f32, f32)>) {
        /*let translation = Vector2::new((time as f64 / 200_000.0).sin() as f32 * 100.0, (time as f64 / 200_000.0).cos() as f32 * 100.0);
        let transform = Matrix4::from(1.0).translate(&translation);
        self.renderer.shader()
            .as_ref().unwrap()
            .upload_uniform("u_Transform", &transform);*/
        let shader = self.shader.as_ref().unwrap();
        match material {
            Either::Left(texture) => {
                shader.upload_uniform("u_Color", &(1.0, 1.0, 1.0, 1.0));
                texture.bind(0);
            },
            Either::Right(color) => {
                shader.upload_uniform("u_Color", color);
                self.white_texture.bind(0);
            }
        }

        shader.upload_uniform("u_TilingFactor", &1.0);
        
        let transform = Matrix4::from(1.0).scale(size) * Matrix4::from(1.0).translate(&position);
        shader.upload_uniform("u_Transform", &transform);

        self.quad_vertex_array.bind();
        self.draw_vertex_array(&self.quad_vertex_array);
    }
}