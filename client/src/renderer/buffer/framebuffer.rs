use std::rc::Rc;

use aeonetica_engine::{math::vector::Vector2, error::{*, builtin::DataError}};

use crate::{renderer::{glerror::GLError, util::Target, shader::{self, UniformStr}, buffer::{BufferLayoutBuilder, Vertex, TexCoord, Buffer, BufferType, BufferUsage}}, vertex, to_raw_byte_slice};
use super::{RenderID, texture::Texture, renderbuffer::RenderBuffer, vertex_array::VertexArray};

pub enum Attachment {
    Color(Texture),
    DepthStencil(RenderBuffer)
}

impl Attachment {
    fn attach(self, fb: &mut FrameBuffer) -> ErrorResult<()> {
        match self {
            Attachment::Color(texture) => unsafe {
                // TODO: check if enough free color attachment pointers exist
                let idx = fb.textures.len() as u32;
                let attachment = gl::COLOR_ATTACHMENT0 + idx;

                gl::FramebufferTexture(gl::FRAMEBUFFER, attachment, texture.id(), 0);

                fb.textures.push(texture);
            },
            Attachment::DepthStencil(rb) => unsafe {
                if fb.renderbuffer.is_some() {
                    return Err(Error::new(DataError("renderbuffer already exists in framebuffer".to_string()), Fatality::DEFAULT, true));
                }
                
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rb.id());
                fb.renderbuffer = Some(rb);
            }
        }

        Ok(())
    }
}

pub struct FrameBuffer {
    id: RenderID,

    renderbuffer: Option<RenderBuffer>,
    textures: Vec<Texture>,

    vao: Option<VertexArray>
}

fn new_framebuffer_vao() -> ErrorResult<VertexArray> {
    let mut vao = VertexArray::new()?;
    vao.bind();

    type Vertices = BufferLayoutBuilder<(Vertex, TexCoord)>;
    let layout = Vertices::build();
    let vertices = Vertices::array([
        vertex!([-1.0, -1.0], [0.0, 0.0]),
        vertex!([ 1.0, -1.0], [1.0, 0.0]),
        vertex!([ 1.0,  1.0], [1.0, 1.0]),
        vertex!([-1.0,  1.0], [0.0, 1.0])
    ]);

    let vertex_buffer = Buffer::new(BufferType::Array, to_raw_byte_slice!(&vertices), Some(Rc::new(layout)), BufferUsage::STATIC)?;
    vao.set_vertex_buffer(vertex_buffer);
    
    const INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];
    let index_buffer = Buffer::new(BufferType::ElementArray, to_raw_byte_slice!(&INDICES), None, BufferUsage::STATIC)?;
    vao.set_index_buffer(index_buffer);

    Ok(vao)
}

impl FrameBuffer {
    pub fn new<const N: usize>(attachments: [Attachment; N], freestanding: bool) -> ErrorResult<Self> {
        let mut fb = Self {
            id: 0,
            renderbuffer: None,
            textures: vec![],
            vao: if freestanding { Some(new_framebuffer_vao()?) } else { None }
        };
        
        unsafe {
            gl::GenFramebuffers(1, &mut fb.id);
            if fb.id == 0 {
                return Err(GLError::from_gl_errno().into_error());
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, fb.id);
        }

        for attachment in attachments {
            attachment.attach(&mut fb)?
        }
        
        unsafe {
            let n_attachments =  fb.textures.len() as i32;
            let tex_attachments = (gl::COLOR_ATTACHMENT0 .. gl::COLOR_ATTACHMENT0 + n_attachments as u32).collect::<Vec<_>>();
            gl::DrawBuffers(n_attachments, tex_attachments.as_ptr());

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                let mut err = GLError::from_gl_errno().into_error();
                err.add_info("error creating framebuffer".to_string());
                Err(err)
            }
            else {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                Ok(fb)
            }
        }
    }

    pub fn bind(&self) {
        unsafe { 
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) }
    }

    pub fn textures(&self) -> &Vec<Texture> {
        &self.textures
    }

    pub fn delete(&mut self) {
        if self.id != 0 {
            unsafe { 
                gl::DeleteFramebuffers(1, &self.id);
            }
            self.id = 0;
        }
    }

    pub fn size(&self) -> Option<Vector2<u32>> {
        self.renderbuffer.as_ref().map(|rb| *rb.size())
    }

    pub fn render<const N: usize>(&self, texture_attachments: [(usize, &UniformStr); N], target: &Target, shader: &shader::Program) {
        debug_assert!(self.vao.is_some());

        shader.bind();

        if let Target::FrameBuffer(fb) = target {
            fb.bind();
        }

        for (i, (attachment, uniform)) in texture_attachments.iter().enumerate() {
            self.textures[*attachment].bind(i as u32);
            shader.upload_uniform(uniform, &(i as i32));
        }

        let vao = self.vao.as_ref().unwrap();
        vao.bind();
        vao.draw(6);
        vao.unbind();

        shader.unbind();
    }

    pub fn clear(&self, color: [f32; 4]) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        self.delete();
    }
}