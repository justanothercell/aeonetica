use std::{rc::Rc, cell::Cell};

use crate::{uniform_str, renderer::{shader::UniformStr, RenderError}};

use super::{buffer::{Buffer, BufferLayout, BufferType, BufferUsage, vertex_array::VertexArray}, RenderID, shader::{self, ShaderDataType}, Renderer, material::Material};
use aeonetica_engine::{collections::ordered_map::ExtractComparable, error::{ErrorResult, IntoError}, Id};

pub type BatchID = Id;

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
    const MAX_BATCH_VERTEX_COUNT: u32 = 6000;
    const MAX_BATCH_INDEX_COUNT: u32 = 6000;

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
        self.vertex_array.delete();
    }

    pub fn has_space_for(&self, data: &VertexData) -> bool {
        // check if the data belongs in the batch
        if self.z_index != data.z_index || self.shader != *data.shader() || self.layout != *data.layout() {
            return false
        }

        // check if the batch has space for the data
        if self.vertices.len() as u32 + data.vertices_num_bytes() >= Self::MAX_BATCH_VERTEX_COUNT {
            return false
        }

        if self.indices.len() as u32 + data.num_indices() >= Self::MAX_BATCH_INDEX_COUNT {
            return false
        }

        // check if the batch contains or has space for the texture
        if let Some(t) = data.texture { 
            return self.textures.contains(&t) || self.textures.len() < Self::NUM_TEXTURE_SLOTS
        }

        return true;
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

        let num_existing_vert_bytes = self.vertices.len();
        let num_existing_vertices = num_existing_vert_bytes as u32 / self.layout.stride();
        self.vertices.extend_from_slice(data.vertices);
        self.vertices_dirty.set(true);
        
        let indices = data.indices().iter().map(|i| i + num_existing_vertices);
        let num_existing_indices = self.indices.len();
        self.indices.extend(indices);
        self.indices_dirty.set(true);

        self.offsets.push(Offset::new(num_existing_vert_bytes, num_existing_indices));

        VertexLocation {
            batch: self.id, 
            offset_index: self.offsets.len() - 1,
            num_vertices: data.num_vertices(),
            num_indices: data.num_indices()
        }
    }

    pub fn modify_vertices(&mut self, location: &VertexLocation, data: &mut [u8], texture: Option<RenderID>) -> ErrorResult<()> {
        let num_vert_bytes: usize = (location.num_vertices() * self.layout.stride()) as usize;
        if num_vert_bytes != data.len() {
            return Err(RenderError(format!("unexpected vertices length; got {}, expected {}", data.len(), num_vert_bytes)).into_error());
        }

        if let Some(texture) = texture {
            let slot = self.textures.iter()
                .position(|t| *t == texture)
                .ok_or_else(|| RenderError(format!("could not find slot for texture {} (id)", texture)).into_error())?;            
            patch_texture_id(data, &self.layout, slot as u32);            
        }

        let offset = &self.offsets[location.offset_index()];

        self.vertices[offset.vertices..offset.vertices + num_vert_bytes].copy_from_slice(data);
        self.vertices_dirty.set(true);

        Ok(())
    }

    pub fn remove_vertices(&mut self, location: &VertexLocation) {
        let offset_index = location.offset_index();
        let offset = &self.offsets[offset_index];
        let num_vert_bytes: usize = (location.num_vertices() * self.layout.stride()) as usize;
        let num_indices = location.num_indices() as usize;
        
        self.vertices.drain(offset.vertices .. offset.vertices + num_vert_bytes);
        self.vertices_dirty.set(true);

        self.indices.drain(offset.indices .. offset.indices + num_indices);
        self.indices_dirty.set(true);

        if self.offsets.len() - 1 == offset_index {
            self.offsets.pop();
        }
        else {
            self.indices[offset.indices..].iter_mut().for_each(|i| *i -= location.num_vertices());

            self.offsets[offset_index + 1..].iter_mut().for_each(|offset| {
                offset.vertices -= num_vert_bytes;
                offset.indices -= num_indices;
            });
        }
    }

    pub fn draw_vertices(&self, renderer: &mut Renderer) {
        if self.indices_dirty.get() {
            self.update_indices();
        }

        if self.vertices_dirty.get() {
            self.update_vertices();
        }

        #[cfg(feature = "gpu_debug")]
        {
            use std::io::Write;
            let renderer = crate::renderer::gpu_debug::RENDERER.with(|f| *f.borrow());
            let batch = self.id;
            let z_index = self.z_index;
            let mut file = std::fs::File::create(format!("gpu_dump/{renderer:08X}_{batch}_{z_index}.batch")).unwrap();
            file.write_all(&self.indices.len().to_le_bytes()).unwrap();
            file.write_all(unsafe { std::slice::from_raw_parts(self.indices.as_ptr() as *const u8, self.indices.len() * 4) }).unwrap();
            file.write_all(&self.vertices.len().to_le_bytes()).unwrap();
            file.write_all(&self.vertices).unwrap();
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
        let num_indices = self.indices.len() as i32;
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
        self.indices.is_empty() && self.vertices.is_empty()
    }

    pub fn update_indices(&self) {
        let num_indices = self.indices.len();

        let index_buffer = self.vertex_array.index_buffer().as_ref().unwrap();
        index_buffer.bind();

        unsafe {
            gl::BufferSubData(
                index_buffer.gl_typ(),
                0isize,
                (num_indices * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _
            )
        }

        self.indices_dirty.set(false);
    }

    pub fn update_vertices(&self) {
        let num_bytes = self.vertices.len();

        let vertex_buffer = self.vertex_array.vertex_buffer().as_ref().unwrap();
        vertex_buffer.bind();

        //eprintln!("Batch {} -> z_index: {} @ 0x{:08X}:\n\tindices ({} u32): {:?}\n\tvertices ({} u8): {:?}", self.id, self.z_index, self as *const _ as usize, self.indices.len(), self.indices, self.vertices.len(), self.vertices);

        unsafe {
            gl::BufferSubData(
                vertex_buffer.gl_typ(),
                0isize,
                num_bytes as isize,
                self.vertices.as_ptr() as *const _
            );
        }

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

    pub fn from_material<M: Material, const N: usize>(vertices: &'a mut [u8], indices: &'a[u32], material: &'a Rc<M>, data: &M::Data<N>, z_index: u8) -> Self {
        Self {
            vertices,
            indices,
            layout: M::layout(),
            shader: material.shader(),
            z_index,
            texture: M::texture_id(data)
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

    pub fn mut_vertices(&mut self) -> &mut [u8] {
        self.vertices
    }

    pub fn vertices_num_bytes(&self) -> u32 {
        self.vertices.len() as u32
    }

    pub fn num_vertices(&self) -> u32 {
        self.vertices.len() as u32 / self.layout.stride()
    }

    pub fn texture(&self) -> Option<RenderID> {
        self.texture
    }

    pub fn shader(&self) -> &Rc<shader::Program> {
        self.shader
    }

    fn patch_texture_id(&mut self, slot: u32) {
        patch_texture_id(self.vertices, self.layout, slot)
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
