use std::{rc::Rc, cell::Cell};

use crate::{uniform_str, renderer::{shader::UniformStr, RenderError}};

use super::{buffer::{Buffer, BufferLayout, BufferType, BufferUsage, vertex_array::VertexArray}, RenderID, shader::{self, ShaderDataType}, Renderer};
use aeonetica_engine::{collections::ordered_map::ExtractComparable, error::{ErrorResult, IntoError}, log_warn, log};

pub type BatchID = u32;

#[derive(Debug)]
struct Offset {
    vertices: usize,
    indices: usize,
}

impl Offset {
    fn new(vertices: usize, indices: usize) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    fn reduce(&mut self, dv: usize, di: usize) {
        self.vertices -= dv;
        self.indices -= di;
    }
}

pub(super) struct Batch {
    id: BatchID,

    layout: Rc<BufferLayout>,
    vertex_array: VertexArray,

    vertices: Vec<u8>,
    vertices_dirty: Cell<bool>,
    indices: Vec<u32>,
    indices_dirty: Cell<bool>,
    offsets: Vec<Offset>,

    shader: Rc<shader::Program>,
    textures: Vec<RenderID>,
    z_index: u8
}

impl Batch {
    const MAX_BATCH_VERTEX_COUNT: u32 = 1024;
    const MAX_BATCH_INDEX_COUNT: u32 = 1024;

    const TEXTURE_SLOTS: [i32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // 16 is the minimum amount per stage required by OpenGL
    const NUM_TEXTURE_SLOTS: usize = Self::TEXTURE_SLOTS.len();

    pub fn new(id: BatchID, data: &VertexData) -> ErrorResult<Batch> {
        let mut vertex_array = VertexArray::new()?;

        let vertex_buffer = Buffer::new_sized(
            BufferType::Array, 
            (Self::MAX_BATCH_VERTEX_COUNT * data.layout().stride()) as isize,
            Some(data.layout().clone()), 
            BufferUsage::DYNAMIC
        )?;
        vertex_array.set_vertex_buffer(vertex_buffer);

        let index_buffer = Buffer::new_sized(
            BufferType::ElementArray,
            Self::MAX_BATCH_INDEX_COUNT as isize * std::mem::size_of::<u32>() as isize,
            None,
            BufferUsage::DYNAMIC
        )?;
        vertex_array.set_index_buffer(index_buffer);

        let vertices = Vec::with_capacity((Self::MAX_BATCH_VERTEX_COUNT * data.layout().stride()) as usize);
        let indices = Vec::with_capacity(Self::MAX_BATCH_INDEX_COUNT as usize);
        let offsets = Vec::with_capacity(Self::MAX_BATCH_VERTEX_COUNT as usize);

        log!("creating batch {}", id);

        Ok(Self {
            id,

            layout: data.layout().clone(),
            vertex_array,
            
            vertices,
            vertices_dirty: Cell::new(false),
            indices,
            indices_dirty: Cell::new(false),
            offsets,

            shader: data.shader().clone(),
            textures: vec![],
            z_index: data.z_index
        })
    }

    pub fn delete(self) {
        log!("deleting batch {}", self.id);
        self.vertex_array.delete();
    }

    pub fn has_space_for(&self, data: &VertexData) -> bool {
        if self.z_index != data.z_index { return false }
        self.vertex_array.vertex_buffer().as_ref().unwrap().count() < Self::MAX_BATCH_VERTEX_COUNT &&
        self.vertex_array.index_buffer().as_ref().unwrap().count() + data.num_indices() <= Self::MAX_BATCH_INDEX_COUNT &&
        &self.shader == data.shader() &&
        self.layout.eq(data.layout()) &&
        if let Some(t) = data.texture { self.textures.contains(&t) || self.textures.len() < Self::NUM_TEXTURE_SLOTS } else { true } 
    }

    pub fn add_vertices(&mut self, data: &mut VertexData) -> VertexLocation {
        if let Some(tex_id) = data.texture {
            let index = self.textures.iter().position(|id| *id == tex_id)
                .unwrap_or_else(|| {
                    self.textures.push(tex_id);
                    self.textures.len() - 1
                });

            data.patch_texture_id(index as u32);
        }

        let num_vert_bytes = self.vertices.len();
        let num_vertices = num_vert_bytes as u32 / self.layout.stride();
        self.vertices.extend_from_slice(data.vertices);
        self.vertices_dirty.set(true);
        
        let indices = data.indices().iter().map(|i| i + num_vertices);
        let num_indices = self.indices.len();
        self.indices.extend(indices);
        self.indices_dirty.set(true);

        self.offsets.push(Offset::new(num_vert_bytes, num_indices));

        VertexLocation {
            batch: self.id, 
            offset_index: self.offsets.len() - 1,
            num_vertices: data.num_vertices(),
            num_indices: data.num_indices()
        }
    }

    pub fn modify_vertices(&mut self, location: &VertexLocation, data: &mut [u8], texture: Option<RenderID>) -> ErrorResult<()> {
        let num_bytes: usize = (location.num_vertices() * self.layout.stride()) as usize;
        if num_bytes < data.len() {
            return Err(RenderError(format!("unexpected vertices length; got {}, expected {}", data.len(), num_bytes)).into_error());
        }

        if let Some(texture) = texture {
            let slot = self.textures.iter()
                .position(|t| *t == texture)
                .ok_or_else(|| RenderError(format!("could not find slot for texture {} (id)", texture)).into_error())?;            
            patch_texture_id(data, &self.layout, slot as u32);            
        }

        let offset = &self.offsets[location.offset_index()];

        self.vertices[offset.vertices..offset.vertices + num_bytes].copy_from_slice(data);
        self.vertices_dirty.set(true);

        Ok(())
    }

    pub fn remove_vertices(&mut self, location: &VertexLocation) {
        let index = location.offset_index();
        let offset = &self.offsets[index];
        let num_bytes: usize = (location.num_vertices() * self.layout.stride()) as usize;
        self.vertices.drain(offset.vertices..offset.vertices + num_bytes);
        self.indices.drain(offset.indices..offset.indices + location.num_indices() as usize);

        if self.offsets.len() - index == 1 {
            self.offsets.pop();
        }
        else {
            self.offsets[index + 1..].iter_mut().for_each(|offset| offset.reduce(num_bytes, location.num_indices() as usize));
        }
    }

    pub fn draw_vertices(&self, renderer: &mut Renderer) {
        if self.indices_dirty.get() {
            self.update_indices();
        }

        if self.vertices_dirty.get() {
            self.update_vertices();
        }

        renderer.load_shader(self.shader.clone());

        for (slot, texture) in self.textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + slot as u32);
                gl::BindTexture(gl::TEXTURE_2D, *texture);
            }
        }
        if !self.textures.is_empty() {
            const TEXTURES_UNIFORM: UniformStr = uniform_str!("u_Textures");
            self.shader.upload_uniform(&TEXTURES_UNIFORM, &Self::TEXTURE_SLOTS.as_slice())
        }

        self.vertex_array.bind();
        let num_indices = self.vertex_array.index_buffer().as_ref().unwrap().count() as i32;
        unsafe {
            gl::DrawElements(gl::TRIANGLES, num_indices, gl::UNSIGNED_INT, std::ptr::null());
        }

        self.vertex_array.unbind();
        for slot in 0..self.textures.len() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + slot as u32);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }

    pub fn id(&self) -> &BatchID {
        &self.id
    }

    pub fn is_deletable(&self) -> bool {
        self.indices.len() == 0 && self.vertices.len() == 0
    }

    pub fn update_indices(&self) {
        let num_indices = self.indices.len();

        let index_buffer = self.vertex_array.index_buffer().as_ref().unwrap();
        index_buffer.bind();

        unsafe {
            gl::BufferData(
                index_buffer.gl_typ(),
                (num_indices * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            )
        }
        index_buffer.set_count(num_indices as u32);

        self.indices_dirty.set(false);
    }

    pub fn update_vertices(&self) {
        let num_bytes = self.vertices.len();

        let vertex_buffer = self.vertex_array.vertex_buffer().as_ref().unwrap();
        vertex_buffer.bind();

        unsafe {
            gl::BufferData(
                vertex_buffer.gl_typ(),
                num_bytes as isize,
                self.vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            );
        }
        vertex_buffer.set_count(num_bytes as u32 / self.layout.stride());

        self.vertices_dirty.set(false);
    }
}

impl ExtractComparable<u8> for Batch {
    fn extract_comparable(&self) -> u8 {
        self.z_index
    }
}

pub struct VertexData<'a> {
    vertices: &'a mut [u8],
    indices: &'a[u32],
    layout: &'a Rc<BufferLayout>,
    shader: &'a Rc<shader::Program>,
    z_index: u8,
    texture: Option<RenderID>,
}

impl<'a> VertexData<'a> {
    pub fn new(vertices: &'a mut [u8], indices: &'a[u32], layout: &'a Rc<BufferLayout>, shader: &'a Rc<shader::Program>, z_index: u8) -> Self {
        Self {
            vertices,
            indices,
            layout,
            shader,
            z_index,
            texture: None,
        }
    }

    pub fn new_textured(vertices: &'a mut [u8], indices: &'a[u32], layout: &'a Rc<BufferLayout>, shader: &'a Rc<shader::Program>, z_index: u8, texture: RenderID) -> Self {
        Self {
            vertices,
            indices,
            layout,
            shader,
            z_index,
            texture: Some(texture),
        }
    }

    pub fn indices(&self) -> &'a[u32] {
        self.indices
    }

    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }

    pub fn layout(&self) -> &Rc<BufferLayout> {
        self.layout
    }

    pub fn vertices(&mut self) -> &mut [u8] {
        self.vertices
    }

    pub fn num_vertices(&self) -> u32 {
        self.vertices.len() as u32 / self.layout.stride()
    }

    pub fn texture(&self) -> Option<RenderID> {
        self.texture
    }

    pub fn shader(&self) -> &Rc<shader::Program> {
        &self.shader
    }

    fn patch_texture_id(&mut self, slot: u32) {
        patch_texture_id(self.vertices, &self.layout, slot)
    }

    pub fn z_index(&self) -> u8 {
        self.z_index
    }
}

fn patch_texture_id(vertices: &mut [u8], layout: &BufferLayout, slot: u32) {
    let slot_bytes = slot.to_le_bytes();
    for element in layout.elements().iter().filter(|e| e.typ() == ShaderDataType::Sampler2D) {
        for i in 0..(vertices.len() as u32 / layout.stride()) {
            let pos = (layout.stride() * i + element.offset()) as usize;
            (0..slot_bytes.len()).for_each(|i| vertices[i + pos] = slot_bytes[i]);
        }
    }
}

#[derive(Clone)]
pub struct VertexLocation {
    batch: BatchID,
    offset_index: usize,
    num_vertices: u32,
    num_indices: u32
}

impl VertexLocation {
    pub(super) fn batch(&self) -> &BatchID {
        &self.batch
    }

    pub fn offset_index(&self) -> usize {
        self.offset_index
    }

    pub(super) fn num_vertices(&self) -> u32 {
        self.num_vertices
    }

    pub(super) fn num_indices(&self) -> u32 {
        self.num_indices
    }
}
