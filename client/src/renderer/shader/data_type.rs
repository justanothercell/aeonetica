use aeonetica_engine::math::{matrix::Matrix4, vector::Vector2};

use crate::renderer::texture::Sampler2D;

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
    pub(crate) const fn size(&self) -> u32 {
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

    pub(crate) const fn component_count(&self) -> i32 {
        match self {
            Self::Float | Self::Int | Self::Bool | Self::Sampler2D => 1,
            Self::Float2 | Self::Int2 => 2,
            Self::Float3 | Self::Int3 => 3,
            Self::Float4 | Self::Int4 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16
        }
    }

    pub(crate) const fn base_type(&self) -> gl::types::GLenum {
        match self {
            Self::Float | Self::Float2 | Self::Float3 | Self::Float4 | Self::Mat3 | Self::Mat4 => gl::FLOAT,
            Self::Int | Self::Int2 | Self::Int3 | Self::Int4 | Self::Sampler2D => gl::INT,
            Self::Bool => gl::BOOL
        }
    }

    pub(crate) const fn base_is_fp(&self) -> bool {
        match self {
            Self::Float | Self::Float2 | Self::Float3 | Self::Float4 | Self::Mat3 | Self::Mat4 => true,
            _ => false
        }
    }

    pub(crate) const fn base_is_int(&self) -> bool {
        match self {
            Self::Int | Self::Int2 | Self::Int3 | Self::Int4 | Self::Sampler2D | Self::Bool => true,
            _ => false
        }
    }

    pub(crate) const fn base_is_long(&self) -> bool {
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
