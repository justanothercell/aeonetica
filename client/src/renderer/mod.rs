pub mod window;
pub mod layer;
pub mod context;
pub mod util;

mod vertex_array;
use std::rc::Rc;

use aeonetica_engine::util::{vector::Vector2, matrix::Matrix4};
mod buffer;
use buffer::*;
pub mod shader;
use shader::*;
mod texture;
use texture::*;
mod batch;
use batch::*;

pub(self) use aeonetica_engine::util::camera::Camera;

pub(self) type RenderID = gl::types::GLuint;

pub struct Renderer {
    shader: Option<Program>,
    view_projection: Option<Matrix4<f32>>,
    batches: Vec<Batch>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            shader: None,
            view_projection: None,
            batches: vec![],
        }
    }

    pub fn begin_scene(&mut self, camera: &Camera) {
        if let Some(shader) = &self.shader {
            shader.upload_uniform("u_ViewProjection", camera.view_projection_matrix());
        }
        self.view_projection = Some(camera.view_projection_matrix().clone());
    }

    pub fn end_scene(&mut self) {
        self.view_projection = None;
    }

    pub fn load_shader(&mut self, shader: Program) {
        shader.bind();
        if let Some(view_projection) = &self.view_projection {
            shader.upload_uniform("u_ViewProjection", view_projection);
        }
        self.shader = Some(shader);
    }

    pub fn unload_shader(&mut self) {
        if let Some(shader) = &self.shader {
            shader.unbind();
        }
        self.shader = None;
    }

    pub fn shader(&self) -> &Option<Program> {
        &self.shader
    }

    pub fn draw_vertices(&mut self) {
        let mut_ref_ptr = self as *mut _;
        self.batches.iter().for_each(|batch|
                batch.draw_vertices(unsafe { &mut *mut_ref_ptr })
        );
    }

    pub fn add_vertices<'a>(&mut self, data: &VertexData<'a>) {
        let mut batch = self.batches
            .iter_mut()
            .filter(|buffer| buffer.has_space_for(data))
            .nth(0);

        if batch.is_none() {
            self.batches.push(Batch::new(data).expect("Error creating new render batch"));
            batch = self.batches.last_mut();
        }

        batch.unwrap().add_vertices(data);
    }

    pub fn static_quad(&mut self, position: &Vector2<f32>, size: Vector2<f32>, color: [f32; 4], shader: Program) {
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
        self.add_vertices(&VertexData::new(util::to_raw_byte_slice!(vertices), indices.as_slice(), Rc::new(layout), shader));
    }
} 
