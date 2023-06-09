use super::*;

pub struct VertexArray {
    id: RenderID,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
}

#[allow(unused)]
impl VertexArray {
    pub fn new() -> ErrorResult<Self> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Ok(Self {
                id: vao,
                vertex_buffer: None,
                index_buffer: None
            })
        }
        else {
            Err(GLError::from_gl_errno().into_error())
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn set_vertex_buffer(&mut self, buffer: Buffer) {
        self.vertex_buffer = Some(buffer);
        let buffer = self.vertex_buffer.as_ref().unwrap();
        unsafe { gl::BindVertexArray(self.id) }
        buffer.bind();

        assert!(buffer.layout().is_some(), "Vertex Buffer has no Layout!");
        let layout = buffer.layout().as_ref().unwrap();

        assert!(!layout.elements().is_empty(), "Vertex Buffer has no layout!");

        let stride = layout.stride() as i32;
        for (i, element) in layout.elements().iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(i as u32);

                let i = i as u32;
                let size = element.component_count();
                let base_type = element.base_type();
                let normalized = element.normalized();
                let offset = element.offset() as *const _;

                match element.typ() {
                    t if t.base_is_fp() => 
                        gl::VertexAttribPointer(i, size, base_type, normalized, stride, offset),
                    t if t.base_is_int() =>
                        gl::VertexAttribIPointer(i, size, base_type, stride, offset),
                    t if t.base_is_long() =>
                        gl::VertexAttribLPointer(i, size, base_type, stride, offset),
                    _ => unreachable!("typ: {:?}", element.typ())
                }
            }
        }
    }

    pub fn vertex_buffer(&self) -> &Option<Buffer> {
        &self.vertex_buffer
    }

    pub fn vertex_buffer_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.vertex_buffer
    }

    pub fn set_index_buffer(&mut self, buffer: Buffer) {
        unsafe { gl::BindVertexArray(self.id) }
        buffer.bind();
        self.index_buffer = Some(buffer);
    }

    pub fn index_buffer(&self) -> &Option<Buffer> {
        &self.index_buffer
    }

    pub fn index_buffer_mut(&mut self) -> &mut Option<Buffer> {
        &mut self.index_buffer
    }

    pub fn draw(&self, num_indices: i32) {
        unsafe { gl::DrawElements(gl::TRIANGLES, num_indices, gl::UNSIGNED_INT, std::ptr::null()); }
    }

    pub fn delete(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteVertexArrays(1, &self.id) }
            self.id = 0;
        }
        self.index_buffer = None;
        self.vertex_buffer = None;
    }
}