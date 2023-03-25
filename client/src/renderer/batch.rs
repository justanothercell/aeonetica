use std::rc::Rc;

use super::{vertex_array::VertexArray, buffer::{Buffer, BufferLayout, BufferType, BufferUsage}};

pub(super) struct Batch {
    layout: Rc<BufferLayout>,
    vertex_array: VertexArray,
}

impl Batch {
    const MAX_BATCH_VERTEX_COUNT: u32 = 1024;
    const MAX_BATCH_INDEX_COUNT: u32 = 1024;

    pub fn new(layout: Rc<BufferLayout>) -> Option<Batch> {
        let mut vertex_array = VertexArray::new()?;

        let vertex_buffer = Buffer::new_sized(
            BufferType::Array, 
            (Self::MAX_BATCH_VERTEX_COUNT * layout.stride()) as isize,
            Some(layout.clone()), 
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
            layout,
            vertex_array,
        })
    }

    pub fn has_space_for(&self, layout: &Rc<BufferLayout>, num_indices: u32) -> bool {
        self.vertex_array.vertex_buffer().as_ref().unwrap().count() < Self::MAX_BATCH_VERTEX_COUNT &&
        self.vertex_array.index_buffer().as_ref().unwrap().count() + num_indices <= Self::MAX_BATCH_INDEX_COUNT &&
        self.layout.eq(layout)
    }

    pub fn add_vertices(&mut self, vertices: &[u8], indices: &[u32]) {
        let num_new_vertices = vertices.len() as u32 / self.layout.stride();

        let vertex_buffer = self.vertex_array.vertex_buffer_mut().as_mut().unwrap();
        let num_vertices = vertex_buffer.count();
        vertex_buffer.bind();
        unsafe {
            gl::BufferSubData(
                BufferType::Array.into(), 
                (self.layout.stride() * num_vertices) as isize, 
                vertices.len() as isize, 
                vertices.as_ptr() as *const _
            );
        }     
        vertex_buffer.set_count(num_vertices + num_new_vertices);

        let indices = indices.iter().map(|i| i + num_vertices).collect::<Vec<_>>();

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

    pub fn draw_vertices(&self) {
        self.vertex_array.bind();
        let num_indices = self.vertex_array.index_buffer().as_ref().unwrap().count() as i32;
        unsafe {
            gl::DrawElements(gl::TRIANGLES, num_indices, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
