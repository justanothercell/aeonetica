use aeonetica_engine::util::vector::Vector2;

use super::{RenderID, texture::{Texture, ImageError}};

pub struct FrameBuffer {
    fbo_id: RenderID,
    rbo_id: RenderID,
    texture: Texture
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Result<Self, ImageError> {
        unsafe { 
            let mut fbo_id = 0;
            gl::GenFramebuffers(1, &mut fbo_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
            
            let texture = Texture::create((width, height).into());
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id(), 0); 

            let mut rbo_id = 0;
            gl::GenRenderbuffers(1, &mut rbo_id);
          
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width as i32, height as i32);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
          
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo_id);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                Err(ImageError::OpenGL())
            }
            else {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                Ok(Self {
                    fbo_id,
                    rbo_id,
                    texture
                })
            }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo_id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn fbo_id(&self) -> RenderID {
        self.fbo_id
    }

    pub fn delete(self) {
        unsafe { 
            gl::DeleteFramebuffers(1, &self.fbo_id);
            gl::DeleteRenderbuffers(1, &self.rbo_id);
        }
        self.texture.delete();
    }

    pub fn size(&self) -> &Vector2<u32> {
        &self.texture.size()
    }
}
