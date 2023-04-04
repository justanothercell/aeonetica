use std::rc::Rc;

use super::{vertex_array::VertexArray, buffer::{Buffer, BufferLayout, BufferType, BufferUsage}, RenderID, shader, Renderer};

pub(super) struct Batch {
    layout: Rc<BufferLayout>,
    vertex_array: VertexArray,
    shader: shader::Program
}

impl Batch {
    const MAX_BATCH_VERTEX_COUNT: u32 = 1024;
    const MAX_BATCH_INDEX_COUNT: u32 = 1024;

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
            shader: data.shader()
        })
    }

    pub fn has_space_for(&self, data: &VertexData) -> bool {
        self.vertex_array.vertex_buffer().as_ref().unwrap().count() < Self::MAX_BATCH_VERTEX_COUNT &&
        self.vertex_array.index_buffer().as_ref().unwrap().count() + data.num_indices() <= Self::MAX_BATCH_INDEX_COUNT &&
        self.shader == data.shader() &&
        self.layout.eq(data.layout())
    }

    pub fn add_vertices(&mut self, data: &VertexData) {
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
        if renderer.shader().as_ref().filter(|s| s == &&self.shader).is_none() {
            renderer.unload_shader();
            renderer.load_shader(self.shader.clone());
        }

        self.vertex_array.bind();
        let num_indices = self.vertex_array.index_buffer().as_ref().unwrap().count() as i32;
        unsafe {
            gl::DrawElements(gl::TRIANGLES, num_indices, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

#[derive(Clone)]
pub struct VertexData<'a> {
    vertices: &'a[u8],
    indices: &'a[u32],
    layout: Rc<BufferLayout>,
    shader: shader::Program,
    texture: Option<RenderID>,
}

impl<'a> VertexData<'a> {
    pub fn new(vertices: &'a[u8], indices: &'a[u32], layout: Rc<BufferLayout>, shader: shader::Program) -> Self {
        Self {
            vertices,
            indices,
            layout,
            shader,
            texture: None,
        }
    }

    pub fn new_textured(vertices: &'a[u8], indices: &'a[u32], layout: Rc<BufferLayout>, shader: shader::Program, texture: RenderID) -> Self {
        Self {
            vertices,
            indices,
            layout,
            shader,
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

    pub fn vertices(&self) -> &'a[u8] {
        self.vertices
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
}
