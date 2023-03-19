use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BufferType {
    Array = gl::ARRAY_BUFFER as isize,
}

pub(super) struct Buffer(RenderID);

impl Buffer {
    pub(super) fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        if vbo != 0 {
            Some(Self(vbo))
        }
        else {
            None
        }
    }

    pub(super) fn data(ty: BufferType, data: &[u8], usage: gl::types::GLenum) {
        unsafe {
            gl::BufferData(
                ty as gl::types::GLenum,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage
            )
        }
    }

    pub(super) fn bind(&self, ty: BufferType) {
        unsafe { gl::BindBuffer(ty as gl::types::GLenum, self.0) }
    }

    pub(super) fn unbind(ty: BufferType) {
        unsafe { gl::BindBuffer(ty as gl::types::GLenum, 0) }
    }
}
