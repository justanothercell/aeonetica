use std::rc::Rc;

use super::{vertex_array::VertexArray, buffer::{Buffer, BufferLayout, BufferType, BufferUsage}, RenderID, shader::{self, ShaderDataType}, Renderer};

pub(super) struct Batch {
    layout: Rc<BufferLayout>,
    vertex_array: VertexArray,
    shader: shader::Program,
    textures: Vec<RenderID>,
    z_index: u8
}

impl Batch {
    const MAX_BATCH_VERTEX_COUNT: u32 = 1024;
    const MAX_BATCH_INDEX_COUNT: u32 = 1024;

    const TEXTURE_SLOTS: [i32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // 16 is the minimum amount per stage required by OpenGL
    const NUM_TEXTURE_SLOTS: usize = Self::TEXTURE_SLOTS.len();

    pub fn new(data: &VertexData) -> Option<Batch> {
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

        Some(Self {
            layout: data.layout().clone(),
            vertex_array,
            shader: data.shader(),
            textures: vec![],
            z_index: 0
        })
    }

    pub fn has_space_for(&self, data: &VertexData) -> bool {
        self.vertex_array.vertex_buffer().as_ref().unwrap().count() < Self::MAX_BATCH_VERTEX_COUNT &&
        self.vertex_array.index_buffer().as_ref().unwrap().count() + data.num_indices() <= Self::MAX_BATCH_INDEX_COUNT &&
        self.shader == data.shader() &&
        self.z_index == data.z_index &&
        self.layout.eq(data.layout()) &&
        if let Some(t) = data.texture { self.textures.contains(&t) || self.textures.len() < Self::NUM_TEXTURE_SLOTS } else { true } 
    }

    pub fn add_vertices(&mut self, data: &mut VertexData) {
        if let Some(tex_id) = data.texture {
            let index = self.textures.iter().position(|id| *id == tex_id)
                .unwrap_or_else(|| {
                    self.textures.push(tex_id);
                    self.textures.len() - 1
                });

            data.patch_texture_id(index as u32);
        }

        let num_new_vertices = data.num_vertices();

        let vertex_buffer = self.vertex_array.vertex_buffer_mut().as_mut().unwrap();
        let num_vertices = vertex_buffer.count();
        vertex_buffer.bind();
        unsafe {
            gl::BufferSubData(
                BufferType::Array.into(), 
                (self.layout.stride() * num_vertices) as isize, 
                data.vertices().len() as isize, 
                data.vertices().as_ptr() as *const _
            );
        }     
        vertex_buffer.set_count(num_vertices + num_new_vertices);

        let indices = data.indices().iter().map(|i| i + num_vertices).collect::<Vec<_>>();

        let index_buffer = self.vertex_array.index_buffer_mut().as_mut().unwrap();
        let num_indices = index_buffer.count();
        let index_size = std::mem::size_of::<u32>() as isize;
        index_buffer.bind();
        unsafe {
            gl::BufferSubData(
                BufferType::ElementArray.into(),
                num_indices as isize * index_size,
                index_size * indices.len() as isize,
                indices.as_ptr() as *const _
            )
        }
        index_buffer.set_count(index_buffer.count() + indices.len() as u32);
    }

    pub fn draw_vertices(&self, renderer: &mut Renderer) {
        // FIXME: only load shaders if needed
        renderer.load_shader(self.shader.clone());

        for (slot, texture) in self.textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + slot as u32);
                gl::BindTexture(gl::TEXTURE_2D, *texture);
            }
        }
        if !self.textures.is_empty() {
            self.shader.upload_uniform("u_Textures", &Self::TEXTURE_SLOTS.as_slice())
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
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        self.z_index == other.z_index
    }

    fn ne(&self, other: &Self) -> bool {
        self.z_index != other.z_index
    }
}

impl Eq for Batch {}

impl PartialOrd for Batch {
    fn ge(&self, other: &Self) -> bool {
        self.z_index.ge(&other.z_index)
    }

    fn gt(&self, other: &Self) -> bool {
        self.z_index.gt(&other.z_index)
    }

    fn le(&self, other: &Self) -> bool {
        self.z_index.le(&other.z_index)
    }

    fn lt(&self, other: &Self) -> bool {
        self.z_index.lt(&other.z_index)
    }
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.z_index.partial_cmp(&other.z_index)
    }
}

impl Ord for Batch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z_index.cmp(&other.z_index)
    }
}

pub struct VertexData<'a> {
    vertices: &'a mut [u8],
    indices: &'a[u32],
    layout: Rc<BufferLayout>,
    shader: shader::Program,
    z_index: u8,
    texture: Option<RenderID>,
}

impl<'a> VertexData<'a> {
    pub fn new(vertices: &'a mut [u8], indices: &'a[u32], layout: Rc<BufferLayout>, shader: shader::Program, z_index: u8) -> Self {
        Self {
            vertices,
            indices,
            layout,
            shader,
            z_index,
            texture: None,
        }
    }

    pub fn new_textured(vertices: &'a mut [u8], indices: &'a[u32], layout: Rc<BufferLayout>, shader: shader::Program, z_index: u8, texture: RenderID) -> Self {
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
        &self.layout
    }

    pub fn vertices(&self) -> &&'a mut [u8] {
        &self.vertices
    }

    pub fn num_vertices(&self) -> u32 {
        self.vertices.len() as u32 / self.layout.stride()
    }

    pub fn texture(&self) -> Option<RenderID> {
        self.texture
    }

    pub fn shader(&self) -> shader::Program {
        self.shader.clone()
    }

    fn patch_texture_id(&mut self, slot: u32) {
        let slot_bytes = slot.to_le_bytes();
        for element in self.layout.elements().iter().filter(|e| e.typ() == ShaderDataType::Sampler2D) {
            for i in 0..self.num_vertices() {
                let pos = (self.layout.stride() * i + element.offset()) as usize;
                (0..slot_bytes.len()).for_each(|i| self.vertices[i + pos] = slot_bytes[i]);
            }
        }
    }

    pub fn z_index(&self) -> u8 {
        self.z_index
    }
}
