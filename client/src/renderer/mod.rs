pub mod buffer;
pub mod builtin;
pub mod context;
pub mod glerror;
pub mod layer;
pub mod material;
pub mod pipeline;
pub mod shader;
pub mod texture;
pub mod util;
pub mod window;

mod batch;

pub use batch::VertexLocation;

use std::rc::Rc;

use buffer::*;
use shader::*;
use texture::*;
use batch::*;

use aeonetica_engine::{math::{vector::Vector2, matrix::Matrix4}, collections::OrderedMap, error::{ErrorResult, ErrorValue, IntoError, Fatality, Error}, log};
pub(self) use aeonetica_engine::math::camera::Camera;

use self::{sprite_sheet::Sprite, font::BitmapFont, layer::LayerUpdater, pipeline::{Pipeline, DefaultPipeline}, util::Target};

pub type RenderID = gl::types::GLuint;

#[derive(Debug)]
pub struct RenderError(String);

impl ErrorValue for RenderError {}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "renderer: {}", self.0)
    }
}

impl IntoError for RenderError {
    fn into_error(self) -> Box<aeonetica_engine::error::Error> {
        Error::new(self, Fatality::WARN, false)
    }
}

pub trait Renderable {
    fn vertex_data(&mut self) -> VertexData<'_>;
    fn texture_id(&self) -> Option<RenderID>;

    fn location(&self) -> &Option<VertexLocation>;
    fn set_location(&mut self, location: Option<VertexLocation>);
    fn has_location(&self) -> bool;
    fn is_dirty(&self) -> bool;
}

pub struct Renderer {
    shader: Option<Rc<Program>>,
    view_projection: Option<Matrix4<f32>>,
    batches: OrderedMap<BatchID, Batch, u8>,
    pipeline: Box<dyn Pipeline>,

    batch_counter: BatchID
}

impl Renderer {
    const VIEW_PROJECTION_UNIFORM: UniformStr = uniform_str!("u_ViewProjection");

    pub fn new() -> Self {
        Self {
            shader: None,
            view_projection: None,
            pipeline: Box::new(DefaultPipeline::new()),
            batches: OrderedMap::new(),
            batch_counter: 0,
        }
    }

    pub fn set_pipeline<P: Pipeline + 'static>(&mut self, pipeline: P) {
        self.pipeline = Box::new(pipeline);
    }

    pub fn begin_scene(&mut self, camera: &Camera) {
        if let Some(shader) = &self.shader {
            shader.upload_uniform(&Self::VIEW_PROJECTION_UNIFORM, camera.view_projection_matrix());
        }
        self.view_projection = Some(camera.view_projection_matrix().clone());
    }

    pub fn end_scene(&mut self) {
        self.view_projection = None;
    }

    pub(crate) fn load_shader(&mut self, shader: Rc<Program>) {
        if self.shader.as_ref() == Some(&shader) {
            return;
        }

        shader.bind();
        if let Some(view_projection) = &self.view_projection {
            shader.upload_uniform(&Self::VIEW_PROJECTION_UNIFORM, view_projection);
        }
        self.shader = Some(shader);
    }

    pub(crate) fn unload_shader(&mut self) {
        if let Some(shader) = &self.shader {
            shader.unbind();
        }
        self.shader = None;
    }

    pub fn draw_vertices(&mut self, _target: &Target) {
        let mut_ref_ptr = self as *mut _;
        self.batches.iter().rev().for_each(|(_, batch)|
            batch.draw_vertices(unsafe { &mut *mut_ref_ptr })
        );

        self.unload_shader();
    }

    fn next_id(&mut self) -> BatchID {
        self.batch_counter += 1;
        self.batch_counter
    }

    pub(self) fn delete_batch(&mut self, id: &BatchID) {
        let num_batches = self.batches.len() - 1;
        self.batches.remove(id).map(|batch| {
            log!("delete batch {} ({} exist)", id, num_batches);
            batch.delete()
        });
    }

    pub fn add_vertices(&mut self, data: &mut VertexData) -> VertexLocation {
        if let Some(idx) = self.batches.iter().position(|(_, batch)| batch.has_space_for(data)) {
            self.batches.nth_mut(idx, |batch| batch.add_vertices(data)).unwrap()
        }
        else {
            let mut batch = Batch::new(self.next_id(), data).expect("Error creating new render batch");
            log!("create batch {} ({} exist)", batch.id(), self.batches.len() + 1);
            let location = batch.add_vertices(data);
            self.batches.insert(*batch.id(), batch);

            location
        }
    }

    pub fn modify_vertices(&mut self, location: &VertexLocation, data: &mut [u8], texture: Option<RenderID>) -> ErrorResult<()> {
        self.batches.get_mut(
            location.batch(), 
            |batch| batch.modify_vertices(location, data, texture)
        ).unwrap_or_else(|| Err(RenderError(format!("invalid batch id {}", location.batch())).into_error()))
    }

    pub fn remove_vertices(&mut self, location: &VertexLocation) {
        let remove = self.batches.get_mut(
            location.batch(),
            |batch| {
                batch.remove_vertices(location);
                batch.is_deletable().then(|| *batch.id())
            }
        );

        if let Some(Some(id)) = remove {
            self.delete_batch(&id);
        }
    }

    pub fn add(&mut self, item: &mut impl Renderable) {
        let location = self.add_vertices(&mut item.vertex_data());
        item.set_location(Some(location));
    }

    pub fn modify(&mut self, item: &mut impl Renderable) -> ErrorResult<()> {
        let texture = item.texture_id();
        self.modify_vertices(&item.location().as_ref().unwrap().clone(), item.vertex_data().vertices(), texture)
    }

    // add or modify a given item, if needed
    pub fn draw(&mut self, item: &mut impl Renderable) -> ErrorResult<()> {
        if !item.has_location() {
            let location = self.add_vertices(&mut item.vertex_data());
            item.set_location(Some(location));
        }
        else if item.is_dirty() {
            let texture = item.texture_id();
            self.modify_vertices(&item.location().clone().unwrap(), item.vertex_data().vertices(), texture)?
        }

        Ok(())
    }

    pub fn remove(&mut self, item: &mut impl Renderable) {
        if let Some(location) = item.location() {
            self.remove_vertices(location);
            item.set_location(None);
        } 
    }

    pub(super) fn on_layer_update(&mut self, camera: &Camera, target: &Target, updater: LayerUpdater, delta_time: f64) {
        let ref_mut_ptr = self as *mut _;
        self.pipeline.pipeline(unsafe { &mut *ref_mut_ptr }, camera, target, updater, delta_time);
    }

    pub fn static_string(&mut self, string: &str, position: &Vector2<f32>, size: f32, spacing: f32, font: &BitmapFont, shader: &Rc<Program>, z_index: u8) {
        type Vertices = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;
        let layout = Rc::new(Vertices::build());

        let size = size / font.char_size().y;

        let half_size = font.char_size() * size / 2.0;

        let texture_id = font.sprite_sheet().texture().id();

        const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

        let mut x_offset = 0.0;

        for c in string.chars() {
            let position = Vector2::new(x_offset, position.y());

            let char_idx = font.char_index(c);
            if char_idx.is_none() {
                continue;
            }
            let char_idx = *char_idx.unwrap();

            let width = font.index_width(char_idx) as f32;
            x_offset += width * size + spacing;

            let char_sprite = font.sprite_sheet().get(char_idx);
            if char_sprite.is_none() {
                continue;
            }
            let char_sprite = char_sprite.unwrap();

            let vertices = Vertices::array([
                vertex!([position.x() - half_size.x(), position.y() - half_size.y()], [char_sprite.left(), char_sprite.top()], Sampler2D(0)),
                vertex!([position.x() + half_size.x(), position.y() - half_size.y()], [char_sprite.right(), char_sprite.top()], Sampler2D(0)),
                vertex!([position.x() + half_size.x(), position.y() + half_size.y()], [char_sprite.right(), char_sprite.bottom()], Sampler2D(0)),
                vertex!([position.x() - half_size.x(), position.y() + half_size.y()], [char_sprite.left(), char_sprite.bottom()], Sampler2D(0))
            ]);

            self.add_vertices(
                &mut VertexData::new_textured(
                    util::to_raw_byte_slice!(&vertices),
                    INDICES.as_slice(),
                    &layout,
                    shader,
                    z_index,
                    texture_id
                )
            );
        }
    }
} 
