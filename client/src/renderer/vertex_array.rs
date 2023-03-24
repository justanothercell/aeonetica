use super::*;

pub struct VertexArray {
    id: RenderID,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
}

impl VertexArray {
    pub(super) fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self {
                id: vao,
                vertex_buffer: None,
                index_buffer: None
            })
        }
        else {
            None
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind() {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn id(&self) -> RenderID {
        self.id
    }

    fn init_vertex_buffer(&mut self) {
        let buffer = self.vertex_buffer.as_ref().unwrap();
        unsafe { gl::BindVertexArray(self.id) }
        buffer.bind();

        assert!(buffer.layout().is_some(), "Vertex Buffer has no Layout!");
        let layout = buffer.layout().as_ref().unwrap();

        assert!(!layout.elements().is_empty(), "Vertex Buffer has no layout!");

        let stride = layout.stride();
        for (i, element) in layout.elements().iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32, 
                    element.component_count(), 
                    element.base_type(),
                    element.normalized(),
                    stride as i32,
                    element.offset() as *const _
                );
            }
        }
    }

    pub fn set_vertex_buffer(&mut self, buffer: Buffer) {
        self.vertex_buffer = Some(buffer);
        self.init_vertex_buffer();
    }

    pub fn set_index_buffer(&mut self, buffer: Buffer) {
        unsafe { gl::BindVertexArray(self.id) }
        buffer.bind();
        self.index_buffer = Some(buffer);
    }

    pub fn index_buffer(&self) -> &Option<Buffer> {
        &self.index_buffer
    }

    pub fn append(&mut self, vertices: Buffer, new_indices: &[i32]) {
        if let Some(index_buffer) = &mut self.index_buffer {
            let num_vertices = self.vertex_buffer.iter().map(|buffer| {
                let num_bytes = buffer.num_bytes() as u32;
                if let Some(layout) = buffer.layout() {
                    num_bytes / layout.total_size()
                }
                else {
                    buffer.count()
                }
            }).sum::<u32>();

           // let mut indices = index_buffer.data::<i32>();
            let indices = new_indices.iter().map(|i| *i + num_vertices as i32).collect::<Vec<_>>();
            
            index_buffer.grow(util::to_raw_byte_slice!(indices));
            index_buffer.bind();
        }
        else {
            let index_buffer = Buffer::new(BufferType::ElementArray, unsafe { std::slice::from_raw_parts(
                new_indices.as_ptr() as *const u8,
                new_indices.len() * std::mem::size_of::<i32>()
            ) }, None)
                .expect("Error creating index buffer");
            self.index_buffer = Some(index_buffer);
        }

        if let Some(vertex_buffer) = &mut self.vertex_buffer {
            vertex_buffer.concat(vertices);
            self.init_vertex_buffer();
        }
        else {
            self.set_vertex_buffer(vertices);
        }
    }
}
