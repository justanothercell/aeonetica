use std::rc::Rc;
use crate::uniform_str;

use super::shader::{self, UniformStr};

pub trait Material {
    fn shader(&self) -> &Rc<shader::Program>;
}

pub struct FlatColor {
    color: [f32; 3],
    shader: Rc<shader::Program>
}

impl FlatColor {
    pub fn new(color: [f32; 3]) -> Result<Self, String> {
        Ok(Self {
            color,
            shader: Rc::new(shader::Program::from_source(include_str!("../../../assets/default-shader.glsl"))?)
        })
    } 

    pub fn with_shader(color: [f32; 3], shader: Rc<shader::Program>) -> Self {
        Self {
            color,
            shader
        }
    }

    const COLOR_UNIFORM: UniformStr = uniform_str!("u_Color");
}

impl Material for FlatColor {
    fn shader(&self) -> &Rc<shader::Program> {
        self.shader.upload_uniform(&Self::COLOR_UNIFORM, &self.color);
        &self.shader
    }
}
