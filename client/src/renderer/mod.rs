pub mod window;
pub mod layer;
pub mod context;
pub mod util;
pub mod postprocessing;
pub mod framebuffer;
pub mod sprite_sheet;
pub mod font;

mod vertex_array;
use std::rc::Rc;

use aeonetica_engine::{util::{vector::Vector2, matrix::Matrix4}, collections::OrderedMap};
mod buffer;
use buffer::*;
pub mod shader;
use shader::*;
pub mod texture;
use texture::*;
mod batch;
use batch::*;

pub(self) use aeonetica_engine::util::camera::Camera;

use self::{sprite_sheet::Sprite, font::BitmapFont};

pub(self) type RenderID = gl::types::GLuint;

pub struct Renderer {
    shader: Option<Program>,
    view_projection: Option<Matrix4<f32>>,
    batches: OrderedMap<BatchID, Batch, u8>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            shader: None,
            view_projection: None,
            batches: OrderedMap::new(),
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
        self.batches.iter().for_each(|(_, batch)|
            batch.draw_vertices(unsafe { &mut *mut_ref_ptr })
        );
    }

    pub fn add_vertices(&mut self, data: &mut VertexData) {
        if let Some(idx) = self.batches.iter().position(|(_, batch)| batch.has_space_for(data)) {
            self.batches.nth_mut(idx, |batch| batch.add_vertices(data));
        }
        else {
            let mut batch = Batch::new(data).expect("Error creating new render batch");
            batch.add_vertices(data);
            self.batches.insert(*batch.id(), batch);
        }
    }

    pub fn static_quad(&mut self, position: &Vector2<f32>, size: Vector2<f32>, color: [f32; 4], shader: Program, z_index: u8) {
        let half_size = size / Vector2::new(2.0, 2.0);

        let layout = Vertices::build();
        type Vertices = BufferLayoutBuilder<(Vertex, Color)>;
        let vertices = Vertices::array([
            vertex!([position.x() - half_size.x(), position.y() - half_size.y(), 0.0], color),
            vertex!([position.x() + half_size.x(), position.y() - half_size.y(), 0.0], color),
            vertex!([position.x() + half_size.x(), position.y() + half_size.y(), 0.0], color),
            vertex!([position.x() - half_size.x(), position.y() + half_size.y(), 0.0], color)
        ]);

        const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];
        self.add_vertices(&mut VertexData::new(util::to_raw_byte_slice!(vertices), INDICES.as_slice(), Rc::new(layout), shader, z_index));
    }

    pub fn textured_quad(&mut self, position: &Vector2<f32>, size: Vector2<f32>, texture: RenderID, shader: Program, z_index: u8) {
        let half_size = size / Vector2::new(2.0, 2.0);

        type Vertices = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;
        let layout = Vertices::build();
        let vertices = Vertices::array([
            vertex!([position.x() - half_size.x(), position.y() - half_size.y(), 0.0], [0.0, 0.0], Sampler2D(0)),
            vertex!([position.x() + half_size.x(), position.y() - half_size.y(), 0.0], [1.0, 0.0], Sampler2D(0)),
            vertex!([position.x() + half_size.x(), position.y() + half_size.y(), 0.0], [1.0, 1.0], Sampler2D(0)),
            vertex!([position.x() - half_size.x(), position.y() + half_size.y(), 0.0], [0.0, 1.0], Sampler2D(0))
        ]);

        const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];
        self.add_vertices(&mut VertexData::new_textured(
            &mut util::to_raw_byte_slice!(vertices),
            INDICES.as_slice(),
            Rc::new(layout), shader, z_index, texture)
        );
    }

    pub fn sprite_quad(&mut self, position: &Vector2<f32>, size: Vector2<f32>, sprite: Sprite, shader: Program, z_index: u8) {
        let half_size = size / Vector2::new(2.0, 2.0);

        type Vertices = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;
        let layout = Vertices::build();
        let vertices = Vertices::array([
            vertex!([position.x() - half_size.x(), position.y() - half_size.y(), 0.0], [sprite.left(), sprite.top()], Sampler2D(0)),
            vertex!([position.x() + half_size.x(), position.y() - half_size.y(), 0.0], [sprite.right(), sprite.top()], Sampler2D(0)),
            vertex!([position.x() + half_size.x(), position.y() + half_size.y(), 0.0], [sprite.right(), sprite.bottom()], Sampler2D(0)),
            vertex!([position.x() - half_size.x(), position.y() + half_size.y(), 0.0], [sprite.left(), sprite.bottom()], Sampler2D(0))
        ]);

        const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];
        self.add_vertices(&mut VertexData::new_textured(
            &mut util::to_raw_byte_slice!(vertices),
            INDICES.as_slice(),
            Rc::new(layout), shader, z_index, sprite.texture())
        );
    }

    pub fn static_string(&mut self, string: &str, position: &Vector2<f32>, size: f32, spacing: f32, font: &BitmapFont, shader: Program, z_index: u8) {
        type Vertices = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;
        let layout = Rc::new(Vertices::build());

        let size = font.char_size(size);
        let half_size = size / Vector2::new(2.0, 2.0);

        let texture_id = font.sprite_sheet().texture().id();

        const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

        for (i, c) in string.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            let position = Vector2::new(position.x() + i as f32 * (size.x() + spacing), position.y());

            let char_idx = font.char_idx(c);
            if char_idx.is_none() {
                continue;
            }

            let char_sprite = font.sprite_sheet().get(*char_idx.unwrap());
            if char_sprite.is_none() {
                continue;
            }
            let char_sprite = char_sprite.unwrap();

            let vertices = Vertices::array([
                vertex!([position.x() - half_size.x(), position.y() - half_size.y(), 0.0], [char_sprite.left(), char_sprite.top()], Sampler2D(0)),
                vertex!([position.x() + half_size.x(), position.y() - half_size.y(), 0.0], [char_sprite.right(), char_sprite.top()], Sampler2D(0)),
                vertex!([position.x() + half_size.x(), position.y() + half_size.y(), 0.0], [char_sprite.right(), char_sprite.bottom()], Sampler2D(0)),
                vertex!([position.x() - half_size.x(), position.y() + half_size.y(), 0.0], [char_sprite.left(), char_sprite.bottom()], Sampler2D(0))
            ]);

            self.add_vertices(
                &mut VertexData::new_textured(
                    &mut util::to_raw_byte_slice!(vertices),
                    INDICES.as_slice(),
                    layout.clone(),
                    shader.clone(),
                    z_index,
                    texture_id
                )
            );
        }
    }
} 
