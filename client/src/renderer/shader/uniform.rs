use std::collections::{hash_map, HashMap};

use aeonetica_engine::math::{vector::{Vector2, Vector3}, matrix::Matrix4};

use crate::renderer::texture::Texture;

#[macro_export]
macro_rules! uniform_str {
    ($value:literal) => {
        UniformStr(concat!($value, '\0').as_ptr())
    };
}
pub use uniform_str;

pub struct UniformStr(pub *const u8);

pub trait Uniform {
    fn upload(&self, location: i32);
}

impl Uniform for Matrix4<f32> {
    fn upload(&self, location: i32) {
        unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, self.value_ptr()) }
    }
}

impl Uniform for u32 {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform1ui(location, *self) }
    }
}

impl Uniform for i32 {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform1i(location, *self) }
    }
}

impl Uniform for f32 {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform1f(location, *self) }
    }
}

impl Uniform for (f32, f32, f32, f32) {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform4f(location, self.0, self.1, self.2, self.3) }
    }
}

impl Uniform for Texture {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform1i(location, self.id() as i32) }
    }
}

impl Uniform for [f32; 3] {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform3f(location, self[0], self[1], self[2]) }
    }
}

impl Uniform for Vector2<f32> {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform2f(location, self.x(), self.y()) }
    }
}

impl Uniform for Vector3<f32> {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform3f(location, self.x(), self.y(), self.z()) }
    }
}

impl<U: Uniform> Uniform for [U] {
    fn upload(&self, location: i32) {
        self.iter().enumerate().for_each(|(i, u)| u.upload(location + i as i32));
    }
}
