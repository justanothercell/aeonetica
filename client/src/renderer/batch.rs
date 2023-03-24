use super::{vertex_array::VertexArray, buffer::Buffer};

pub(super) struct Batch {
    vao: VertexArray
}

impl Batch {
    pub fn new() -> Option<Batch> {
        Some(Self {
            vao: VertexArray::new()?
        })
    }

    pub fn add_vertices(&mut self, vertices: Buffer, indices: &[i32]) {
        self.vao.append(vertices, indices);
    }

    pub fn vao(&self) -> &VertexArray {
        &self.vao
    }
}