pub mod window;
pub mod layer;
pub mod context;
pub mod util;

mod vertex_array;
use core::num;
use std::rc::Rc;

use aeonetica_engine::util::{vector::Vector2, Either, matrix::Matrix4};
use vertex_array::*;
mod buffer;
use buffer::*;
mod shader;
use shader::*;
mod texture;
use texture::*;
mod batch;
use batch::*;

pub(self) use aeonetica_engine::util::camera::Camera;

pub(self) type RenderID = gl::types::GLuint;

pub struct Renderer {
    shader: Option<Rc<Program>>,
    batches: Vec<Batch>
}

impl Renderer {
    pub fn new() -> Self {
        let shader_src = include_str!("../../assets/test_shader.glsl");
        let default_shader = Rc::new(Program::from_source(shader_src)
            .unwrap_or_else(|err| panic!("Error loading shader: {err}")));
        default_shader.bind();

        Self {
            shader: Some(default_shader.clone()),
            batches: vec![],
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
        self.shader.as_ref().map(|shader| shader.unbind());
    }

    pub fn draw_vertices(&self) {
        self.batches.iter().for_each(|batch| batch.draw_vertices());
    }

    pub fn add_vertices(&mut self, vertices: &[u8], layout: &Rc<BufferLayout>, indices: &[u32]) {
        let num_indices = indices.len() as u32;
        let mut batch = self.batches
            .iter_mut()
            .filter(|buffer| buffer.has_space_for(&layout, num_indices))
            .nth(0);

        if batch.is_none() {
            self.batches.push(Batch::new(layout.clone()).expect("Error creating new render batch"));
            batch = self.batches.last_mut();
        }

        let batch = batch.unwrap();
        batch.add_vertices(vertices, indices);
    }

    pub fn static_quad(&mut self, position: &Vector2<f32>, size: Vector2<f32>, color: [f32; 4]) {
        let half_size = size / Vector2::new(2.0, 2.0);

        let layout = Vertices::build();
        type Vertices = BufferLayoutBuilder<(Vertex, Color)>;
        let vertices = Vertices::array([
            ([position.x() - half_size.x(), position.y() - half_size.y(), 0.0], color),
            ([position.x() + half_size.x(), position.y() - half_size.y(), 0.0], color),
            ([position.x() + half_size.x(), position.y() + half_size.y(), 0.0], color),
            ([position.x() - half_size.x(), position.y() + half_size.y(), 0.0], color)
        ]);

        let indices = [0, 1, 2, 2, 3, 0];
        self.add_vertices(util::to_raw_byte_slice!(vertices), &Rc::new(layout), &indices);
    }
} 