pub mod postprocessing;
pub use postprocessing::*;

use std::collections::HashMap;
use super::*;

use aeonetica_engine::util::{matrix::Matrix4, vector::Vector2};
use regex::Regex;

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

impl Uniform for &[i32] {
    fn upload(&self, location: i32) {
        unsafe { gl::Uniform1iv(location, self.len() as i32, self.as_ptr()) }
    }
}

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize
}

#[derive(Clone)]
pub struct Shader(RenderID);
impl Shader {
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty)
            .ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }

    pub(super) fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(ty as gl::types::GLenum) };
        if shader != 0 {
            Some(Self(shader))
        }
        else {
            None
        }
    }

    pub(super) fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap())
            );
        }
    }

    pub(super) fn compile(&self) {
        unsafe { gl::CompileShader(self.0) }
    }

    pub(super) fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
        compiled == gl::TRUE.into()
    }

    pub(super) fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0;
        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast()
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub(super) fn delete(self) {
        unsafe { gl::DeleteShader(self.0) }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Program(RenderID);
impl Program {
    pub(super) fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };
        if prog != 0 {
            Some(Self(prog))
        }
        else {
            None
        }
    }

    pub(super) fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.0, shader.0) }
    }

    pub(super) fn link(&self) {
        unsafe { gl::LinkProgram(self.0) }
    }

    pub(super) fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
        success == gl::TRUE.into()
    }

    pub(super) fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast()
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub(super) fn bind(&self) {
        unsafe { gl::UseProgram(self.0) }
    }

    pub(super) fn unbind(&self) {
        unsafe { gl::UseProgram(0) }
    }

    pub(super) fn delete(self) {
        unsafe { gl::DeleteProgram(self.0) }
    }

    pub(super) fn upload_uniform<U: Uniform + ?Sized>(&self, name: &UniformStr, data: &U) {
        unsafe {
            let location = gl::GetUniformLocation(self.0, name.0 as *const i8);
            data.upload(location);
        }
    }

    fn preprocess_sources(src: &str) -> Result<(String, String), String> {
        // remove all block comments /* */
        let comment_re = Regex::new(r"(?m)/\*[\s\S]*?\*/").unwrap();
        let src = comment_re.replace(src, "").to_string();
        // capture all #[<name>] at start of line
        let region_re = Regex::new(r"(?m)^(?:^|\n)#\[(\w+)]").unwrap();
        let mut start = 0;
        let mut regions = HashMap::new();
        let mut opt_match = region_re.find_at(&src, start);
        while let Some(m) = opt_match {
            let name = m.as_str().trim();
            // remove the #[...]
            let name = &name[2..name.len()-1];
            start = m.end();
            opt_match = region_re.find_at(&src, start);
            if let Some(m2) = opt_match {
                // is not last region
                regions.insert(name, &src[m.end()..m2.start()]);
            } else {
                // is last region
                regions.insert(name, &src[m.end()..]);
            };
        }
        Ok((regions.remove("vertex").ok_or("did not find vertex region in shader".to_string())?.to_string(),
            regions.remove("fragment").ok_or("did not find fragment region in shader".to_string())?.to_string()))
    }

    pub fn from_source(src: &str) -> Result<Self, String> {
        // find first `#type`
        let (vertex_src, fragment_src) = Self::preprocess_sources(src)?;
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, &vertex_src)
            .map_err(|e| format!("Vertex Shader Compile Error: {e}"))?;
        let f = Shader::from_source(ShaderType::Fragment, &fragment_src)
            .map_err(|e| format!("Fragment Shader Compile Error: {e}"))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        }
        else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderDataType {
    Float = gl::FLOAT as isize,
    Float2 = gl::FLOAT_VEC2 as isize,
    Float3 = gl::FLOAT_VEC3 as isize,
    Float4 = gl::FLOAT_VEC4 as isize,
    Mat3 = gl::FLOAT_MAT3 as isize,
    Mat4 = gl::FLOAT_MAT4 as isize,
    Int = gl::INT as isize,
    Int2 = gl::INT_VEC2 as isize,
    Int3 = gl::INT_VEC3 as isize,
    Int4 = gl::INT_VEC4 as isize,
    Bool = gl::BOOL as isize,
    Sampler2D = gl::SAMPLER_2D as isize,
}

impl ShaderDataType {
    pub(super) const fn size(&self) -> u32 {
        use {std::mem::size_of, gl::types::*};
        (match self {
            Self::Float => size_of::<GLfloat>(),
            Self::Float2 => size_of::<GLfloat>() * 2,
            Self::Float3 => size_of::<GLfloat>() * 3,
            Self::Float4 => size_of::<GLfloat>() * 4,
            Self::Mat3 => size_of::<GLfloat>() * 9,
            Self::Mat4 => size_of::<GLfloat>() * 16,
            Self::Int => size_of::<GLint>(),
            Self::Int2 => size_of::<GLint>() * 2,
            Self::Int3 => size_of::<GLint>() * 3,
            Self::Int4 => size_of::<GLint>() * 4,
            Self::Bool => size_of::<GLboolean>(),
            Self::Sampler2D => size_of::<GLint>(),
        }) as u32
    }

    pub(super) const fn component_count(&self) -> i32 {
        match self {
            Self::Float | Self::Int | Self::Bool | Self::Sampler2D => 1,
            Self::Float2 | Self::Int2 => 2,
            Self::Float3 | Self::Int3 => 3,
            Self::Float4 | Self::Int4 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16
        }
    }

    pub(super) const fn base_type(&self) -> gl::types::GLenum {
        match self {
            Self::Float | Self::Float2 | Self::Float3 | Self::Float4 | Self::Mat3 | Self::Mat4 => gl::FLOAT,
            Self::Int | Self::Int2 | Self::Int3 | Self::Int4 | Self::Sampler2D => gl::INT,
            Self::Bool => gl::BOOL
        }
    }

    pub(super) const fn base_is_fp(&self) -> bool {
        match self {
            Self::Float | Self::Float2 | Self::Float3 | Self::Float4 | Self::Mat3 | Self::Mat4 => true,
            _ => false
        }
    }

    pub(super) const fn base_is_int(&self) -> bool {
        match self {
            Self::Int | Self::Int2 | Self::Int3 | Self::Int4 | Self::Sampler2D | Self::Bool => true,
            _ => false
        }
    }

    pub(super) const fn base_is_long(&self) -> bool {
        false
    }
}

pub trait IntoShaderDataType {
    const DATA_TYPE: ShaderDataType;
}

impl IntoShaderDataType for f32 {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Float;
}

impl IntoShaderDataType for [f32; 2] {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Float2;
}

impl IntoShaderDataType for [f32; 3] {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Float3;
}

impl IntoShaderDataType for [f32; 4] {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Float4;
}

impl IntoShaderDataType for Matrix4<f32> {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Mat4;
}

impl IntoShaderDataType for i32 {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Int;
}

impl IntoShaderDataType for [i32; 2] {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Int2;
}

impl IntoShaderDataType for [i32; 3] {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Int3;
}

impl IntoShaderDataType for [i32; 4] {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Int4;
}

impl IntoShaderDataType for Vector2<f32> {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Float2;
}

impl IntoShaderDataType for bool {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Bool;
}

impl IntoShaderDataType for Sampler2D {
    const DATA_TYPE: ShaderDataType = ShaderDataType::Sampler2D;
}

pub trait ShaderLayoutType {
    type Type: IntoShaderDataType;
}
