use std::rc::Rc;

use crate::{uniform_str, vertex};

use super::{shader::{self, UniformStr}, buffer::{Vertex, Color, TexCoord, TextureID, BufferLayoutBuilder, BufferLayout, VertexTuple2, VertexTuple3}, RenderID, texture::Sampler2D};

pub trait Material {
    type Layout;
    type Data<const N: usize>;
    type VertexTuple;

    fn shader(&self) -> &Rc<shader::Program>;
    fn texture_id<const N: usize>(data: &Self::Data<N>) -> Option<RenderID>;
    fn layout<'a>() -> &'a Rc<BufferLayout>;
    fn vertices<const N: usize>(&self, vertices: [[f32; 3]; N], data: &Self::Data<N>) -> [Self::VertexTuple; N];
}

pub struct FlatColor {
    shader: Rc<shader::Program>
}

thread_local! {
    static FLAT_COLOR_LAYOUT: Rc<BufferLayout> = Rc::new(<FlatColor as Material>::Layout::build());
    static FLAT_COLOR_SHADER: Rc<shader::Program> = Rc::new(shader::Program::from_source(include_str!("../../../assets/flat-color-shader.glsl")).expect("failed loading flat color shader"));
    static FLAT_COLOR_INSTANCE: Rc<FlatColor> = Rc::new(FlatColor::new());
}

impl FlatColor {
    fn new() -> Self {
        Self {
            shader: FLAT_COLOR_SHADER.with(|shader| shader.clone())
        }
    } 

    pub fn get() -> Rc<Self> {
        FLAT_COLOR_INSTANCE.with(|instance| instance.clone())
    }

    pub fn with_shader(shader: Rc<shader::Program>) -> Self {
        Self {
            shader
        }
    }

    pub const COLOR_UNIFORM: UniformStr = uniform_str!("u_Color");
}

impl Material for FlatColor {
    type Layout = BufferLayoutBuilder<(Vertex, Color)>;
    type Data<const N: usize> = [f32; 4];
    type VertexTuple = VertexTuple2<[f32; 3], [f32; 4]>;

    fn shader(&self) -> &Rc<shader::Program> {
        &self.shader
    }

    fn texture_id<const N: usize>(_: &Self::Data<N>) -> Option<RenderID> {
        None
    }

    fn layout<'a>() -> &'a Rc<BufferLayout> {
        unsafe {
            let x: *const Rc<BufferLayout> = FLAT_COLOR_LAYOUT.with(|l| l as *const _);
            x.as_ref().unwrap_unchecked()
        }   
    }

    fn vertices<const N: usize>(&self, vertices: [[f32; 3]; N], data: &Self::Data<N>) -> [Self::VertexTuple; N] {
        Self::Layout::array(std::array::from_fn(|i| vertex!(vertices[i], *data)))
    }
}

pub struct FlatTexture {
    shader: Rc<shader::Program>
}

thread_local! {
    static FLAT_TEXTURE_LAYOUT: Rc<BufferLayout> = Rc::new(<FlatTexture as Material>::Layout::build());
    static FLAT_TEXTURE_SHADER: Rc<shader::Program> = Rc::new(shader::Program::from_source(include_str!("../../../assets/flat-texture-shader.glsl")).expect("failed loading flat color shader"));
    static FLAT_TEXTURE_INSTANCE: Rc<FlatTexture> = Rc::new(FlatTexture::new());
}

impl FlatTexture {
    fn new() -> Self {
        Self {
            shader: FLAT_TEXTURE_SHADER.with(|shader| shader.clone())
        }
    }

    pub fn with_shader(shader: Rc<shader::Program>) -> Self {
        Self {
            shader
        }
    }

    pub fn get() -> Rc<Self> {
        FLAT_TEXTURE_INSTANCE.with(|instance| instance.clone())
    }
}

impl Material for FlatTexture {
    type Layout = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;
    type Data<const N: usize> = ([[f32; 2]; N], RenderID);
    type VertexTuple = VertexTuple3<[f32; 3], [f32; 2], Sampler2D>;

    fn shader(&self) -> &Rc<shader::Program> {
        &self.shader
    }

    fn texture_id<const N: usize>(data: &Self::Data<N>) -> Option<RenderID> {
        Some(data.1)
    }

    fn layout<'a>() -> &'a Rc<BufferLayout> {
        unsafe {
            let x: *const Rc<BufferLayout> = FLAT_TEXTURE_LAYOUT.with(|l| l as *const _);
            x.as_ref().unwrap_unchecked()
        }   
    }

    fn vertices<const N: usize>(&self, vertices: [[f32; 3]; N], data: &Self::Data<N>) -> [Self::VertexTuple; N] {
        Self::Layout::array(std::array::from_fn(|i| vertex!(vertices[i], data.0[i], Sampler2D(0))))
    }
}