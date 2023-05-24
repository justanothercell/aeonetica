use super::buffer::framebuffer::FrameBuffer;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    // Just show the points.
    Point = gl::POINT as isize,
    // Just show the lines.
    Line = gl::LINE as isize,
    // Fill in the polygons.
    Fill = gl::FILL as isize
}

#[allow(unused)]
#[inline]
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as gl::types::GLenum) };
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    One = gl::ONE as isize,
    Alpha = gl::SRC_ALPHA as isize,
    Multiply = gl::DST_COLOR as isize
}

#[allow(unused)]
#[inline]
pub fn blend_mode(mode: BlendMode) {
    unsafe { gl::BlendFunc(mode as gl::types::GLenum, gl::ONE_MINUS_SRC_ALPHA) };
}

pub fn enable_blend_mode(enabled: bool) {
    unsafe {
        if enabled { gl::Enable(gl::BLEND) } else { gl::Disable(gl::BLEND) }
    }
}

pub enum Target<'a> {
    Raw,
    FrameBuffer(&'a FrameBuffer)
}

#[macro_export]
macro_rules! to_raw_byte_slice {
    ($value: expr) => {
        unsafe { std::mem::transmute::<_, &mut [u8]>(($value, std::mem::size_of_val($value))) }
    };
}
pub use to_raw_byte_slice;

#[macro_export]
macro_rules! to_raw_byte_vec {
    ($value: expr) => {
        $crate::renderer::util::to_raw_byte_slice!($value).to_owned()
    }
}
pub use to_raw_byte_vec;

#[macro_export]
macro_rules! get_gl_str {
    ($name: expr) => {
        unsafe { std::ffi::CStr::from_ptr(gl::GetString($name) as *const i8).to_str().unwrap_or("no such value") }
    };
}
pub(crate) use get_gl_str;