use super::*;

pub(super) struct VertexArray {
    id: RenderID
}

impl VertexArray {
    pub(super) fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self {
                id: vao
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
}
