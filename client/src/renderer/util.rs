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
#[inline]
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as gl::types::GLenum) };
}

#[macro_export]
macro_rules! to_raw_byte_slice {
    ($value: expr) => {
        unsafe { std::mem::transmute::<_, &[u8]>((&$value, std::mem::size_of_val(&$value))) }
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
