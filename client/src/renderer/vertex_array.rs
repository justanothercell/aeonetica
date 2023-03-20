use super::*;

pub struct VertexArray {
    id: RenderID,
    vertex_buffers: Vec<Buffer>,
    index_buffer: Option<Buffer>,
}

impl VertexArray {
    pub(super) fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self {
                id: vao,
                vertex_buffers: vec![],
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

    pub fn add_vertex_buffer(&mut self, buffer: Buffer) {
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
        self.vertex_buffers.push(buffer);
    }

    pub fn set_index_buffer(&mut self, buffer: Buffer) {
        unsafe { gl::BindVertexArray(self.id) }
        buffer.bind();
        self.index_buffer = Some(buffer);
    }

    pub fn index_buffer(&self) -> &Option<Buffer> {
        &self.index_buffer
    }
}
