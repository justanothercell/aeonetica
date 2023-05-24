use aeonetica_engine::{math::vector::Vector2, error::{ErrorResult, IntoError}};

use crate::renderer::{RenderID, glerror::GLError};

pub struct RenderBuffer {
    id: RenderID,
    size: Vector2<u32>
}

impl RenderBuffer {
    pub fn new(size: Vector2<u32>) -> ErrorResult<Self> {
        let mut id = 0;

        unsafe {
            gl::GenRenderbuffers(1, &mut id);

            gl::BindRenderbuffer(gl::RENDERBUFFER, id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, size.x() as i32, size.y as i32);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }

        if id == 0 {
            Err(GLError::from_gl_errno().into_error())
        }
        else {
            Ok(Self {id, size})
        }
    }

    pub fn size(&self) -> &Vector2<u32> {
        &self.size
    }

    pub fn id(&self) -> RenderID {
        self.id
    }

    pub fn bind(&self) {
        unsafe { gl::BindRenderbuffer(gl::RENDERBUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindRenderbuffer(gl::RENDERBUFFER, 0); }
    }

    pub fn delete(self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.id);
        }
    }
}
