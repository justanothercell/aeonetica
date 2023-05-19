use std::rc::Rc;

use super::buffer::framebuffer::FrameBuffer;

pub enum Target {
    FrameBuffer(Rc<FrameBuffer>),
    Window
}