use std::rc::Rc;

use aeonetica_client::{renderer::{buffer::{BufferLayout, BufferLayoutBuilder, Vertex, TexCoord, TextureID, Color, VertexTuple4}, shader, material::{Material, FlatTexture}, RenderID, texture::{Sampler2D, Sprite}, builtin::Quad}, vertex};
use aeonetica_engine::math::vector::Vector2;
use aeonetica_engine::error::ExpectLog;

pub struct GlowTexture {
    shader: Rc<shader::Program>
}

thread_local! {
    static TERRAIN_SHADER: Rc<shader::Program> = Rc::new(shader::Program::from_source(include_str!("../../assets/terrain-shader.glsl")).expect_log());
    static TERRAIN_MATERIAL_INSTANCE: Rc<FlatTexture> = Rc::new(FlatTexture::with_shader(TERRAIN_SHADER.with(|shader| shader.clone())));

    static GLOW_TEXTURE_LAYOUT: Rc<BufferLayout> = Rc::new(<GlowTexture as Material>::Layout::build());
    static GLOW_TEXTURE_SHADER: Rc<shader::Program> = Rc::new(shader::Program::from_source(include_str!("../../assets/glow-shader.glsl")).expect_log());
    static GLOW_TEXTURE_INSTANCE: Rc<GlowTexture> = Rc::new(GlowTexture::new());
}

pub(super) fn terrain_material() -> Rc<FlatTexture> {
    TERRAIN_MATERIAL_INSTANCE.with(|material| material.clone())
}

pub trait WithTerrain {
    fn with_terrain_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID) -> Self;
    fn with_terrain_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite) -> Self;
}

impl WithTerrain for Quad<FlatTexture> {
    fn with_terrain_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID) -> Self {
        Self::new(position, size, z_index, terrain_material(), ([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], texture))
    }

    fn with_terrain_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite) -> Self {
        Self::new(position, size, z_index, terrain_material(), ([
            [sprite.left(),  sprite.top()   ],
            [sprite.right(), sprite.top()   ],
            [sprite.right(), sprite.bottom()],
            [sprite.left(),  sprite.bottom()]
        ], sprite.texture()))
    }
}

impl GlowTexture {
    fn new() -> Self {
        Self {
            shader: GLOW_TEXTURE_SHADER.with(|shader| shader.clone())
        }
    }

    pub fn get() -> Rc<Self> {
        GLOW_TEXTURE_INSTANCE.with(|instance| instance.clone())
    }
}

impl Material for GlowTexture {
    type Layout = BufferLayoutBuilder<(Vertex, TexCoord, TextureID, Color)>;

    type Data<const N: usize> = ([[f32; 2]; N], RenderID, [f32; 4]);

    type VertexTuple = VertexTuple4<[f32; 2], [f32; 2], Sampler2D, [f32; 4]>;

    fn shader(&self) -> &Rc<shader::Program> {
        &self.shader
    }

    fn texture_id<const N: usize>(data: &Self::Data<N>) -> Option<RenderID> {
        Some(data.1)
    }

    fn layout<'a>() -> &'a Rc<BufferLayout> {
        unsafe {
            let x: *const Rc<BufferLayout> = GLOW_TEXTURE_LAYOUT.with(|l| l as *const _);
            x.as_ref().unwrap_unchecked()
        }
    }

    fn vertices<const N: usize>(&self, vertices: [[f32; 2]; N], data: &Self::Data<N>) -> [Self::VertexTuple; N] {
        Self::Layout::array(std::array::from_fn(|i| vertex!(vertices[i], data.0[i], Sampler2D(0), data.2)))
    }

    fn data_slice<const N: usize, const NN: usize>(&self, data: &Self::Data<N>, offset: usize) -> Self::Data<NN> {
        (std::array::from_fn(|i| data.0[offset + i]), data.1, data.2)
    }

    fn default_data<const N: usize>(&self) -> Self::Data<N> {
        (std::array::from_fn(|_| [0.0; 2]), 0, [0.0; 4])
    }
}

pub(super) trait WithGlow {
    fn with_glow_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, glow_color: [f32; 4]) -> Self;
    fn with_glow_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, glow_color: [f32; 4]) -> Self;
}

impl WithGlow for Quad<GlowTexture> {
    fn with_glow_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, glow_color: [f32; 4]) -> Self {
        Self::new(position, size, z_index, GlowTexture::get(), ([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], texture, glow_color))
    }

    fn with_glow_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, glow_color: [f32; 4]) -> Self {
        Self::new(position, size, z_index, GlowTexture::get(), ([
            [sprite.left(),  sprite.top()   ],
            [sprite.right(), sprite.top()   ],
            [sprite.right(), sprite.bottom()],
            [sprite.left(),  sprite.bottom()]
        ], sprite.texture(), glow_color))
    }
}