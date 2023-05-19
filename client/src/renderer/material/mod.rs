use std::{rc::Rc, cell::RefCell};
use aeonetica_engine::{error::ErrorResult, math::vector::Vector2};

use crate::uniform_str;

use super::{shader::{self, UniformStr}, buffer::{Vertex, Color, TexCoord, TextureID, BufferLayoutBuilder}, RenderID};

pub trait Material {
    type Attributes;
    type Data;

    fn shader(&self) -> &Rc<shader::Program>;
}

pub struct FlatColor {
    shader: Rc<shader::Program>
}

thread_local! {
    static FLAT_COLOR_SHADER: RefCell<Rc<shader::Program>> = RefCell::new(
        Rc::new(shader::Program::from_source(include_str!("../../../assets/flat-color-shader.glsl")).expect("failed loading flat color shader"))
    );
}

impl FlatColor {
    pub fn new() -> ErrorResult<Self> {
        Ok(Self {
            shader: FLAT_COLOR_SHADER.with_borrow(|f| f.clone())
        })
    } 

    pub fn with_shader(shader: Rc<shader::Program>) -> Self {
        Self {
            shader
        }
    }

    pub const COLOR_UNIFORM: UniformStr = uniform_str!("u_Color");
}

impl Material for FlatColor {
    type Attributes = BufferLayoutBuilder<(Vertex, Color)>;
    type Data = [f32; 4];

    fn shader(&self) -> &Rc<shader::Program> {
        &self.shader
    }
}

pub struct FlatTexture {
    shader: Rc<shader::Program>
}

thread_local! {
    static FLAT_TEXTURE_SHADER: RefCell<Rc<shader::Program>> = RefCell::new(
        Rc::new(shader::Program::from_source(include_str!("../../../assets/flat-texture-shader.glsl")).expect("failed loading flat color shader"))
    )
}

impl FlatTexture {
}

impl Material for FlatTexture {
    type Attributes = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;
    type Data = (Vector2<f32>, RenderID);

    fn shader(&self) -> &Rc<shader::Program> {
        &self.shader
    }
}