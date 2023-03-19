#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    // Just show the points.
    Point = gl::POINT as isize,
    // Just show the lines.
    Line = gl::LINE as isize,
    // Fill in the polygons.
    Fill = gl::FILL as isize,
}

#[allow(unused)]
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as gl::types::GLenum) };
}

#[macro_export]
macro_rules! to_raw_byte_slice {
    ($value:expr) => {
        unsafe { std::slice::from_raw_parts($value.as_ptr().cast(), std::mem::size_of_val(&$value)) }
    };
}
pub(crate) use to_raw_byte_slice;
