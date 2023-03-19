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

