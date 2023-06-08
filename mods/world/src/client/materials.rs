use std::rc::Rc;

use aeonetica_client::{renderer::{buffer::{BufferLayout, BufferLayoutBuilder, Vertex, TexCoord, TextureID, Color, VertexTuple4}, shader, material::{Material, FlatTexture}, RenderID, texture::{Sampler2D, Sprite}, builtin::Quad}, vertex, data_store::DataStore};
use aeonetica_engine::math::vector::Vector2;
use aeonetica_engine::error::ExpectLog;

struct TerrainMaterial(Rc<FlatTexture>);
struct TerrainShader(Rc<shader::Program>);

thread_local! {
    static GLOW_TEXTURE_LAYOUT: Rc<BufferLayout> = Rc::new(<GlowTexture as Material>::Layout::build());
}

fn create_terrain_shader() -> TerrainShader {
    TerrainShader(Rc::new(shader::Program::from_source(include_str!("../../assets/terrain-shader.glsl")).expect_log()))
}

pub fn terrain_material(store: &mut DataStore) -> Rc<FlatTexture> {
    let shader = store.get_or_create(create_terrain_shader).0.clone();
    store.get_or_create(|| TerrainMaterial(Rc::new(FlatTexture::with_shader(shader)))).0.clone()
}

pub fn terrain_shader(store: &mut DataStore) -> Rc<shader::Program> {
    store.get_or_create(create_terrain_shader).0.clone()
}

pub trait WithTerrain {
    fn with_terrain_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, material: Rc<FlatTexture>) -> Self;
    fn with_terrain_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, material: Rc<FlatTexture>) -> Self;
}

impl WithTerrain for Quad<FlatTexture> {
    fn with_terrain_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, material: Rc<FlatTexture>) -> Self {
        Self::new(position, size, z_index, material, ([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], texture))
    }

    fn with_terrain_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, material: Rc<FlatTexture>) -> Self {
        Self::new(position, size, z_index, material, ([
            [sprite.left(),  sprite.top()   ],
            [sprite.right(), sprite.top()   ],
            [sprite.right(), sprite.bottom()],
            [sprite.left(),  sprite.bottom()]
        ], sprite.texture()))
    }
}

struct GlowTextureShader(Rc<shader::Program>);

pub struct GlowTexture {
    shader: Rc<shader::Program>
}

impl GlowTexture {
    pub fn get(store: &mut DataStore) -> Rc<Self> {
        let shader = store.get_or_create(|| GlowTextureShader(Rc::new(shader::Program::from_source(include_str!("../../assets/glow-shader.glsl")).expect_log()))).0.clone();
        store.get_or_create(|| Rc::new(Self { shader })).clone()
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

pub trait WithGlow {
    fn with_glow_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, glow_color: [f32; 4], material: Rc<GlowTexture>) -> Self;
    fn with_glow_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, glow_color: [f32; 4], material: Rc<GlowTexture>) -> Self;

    fn light_color(&self) -> [f32; 4];
}

impl WithGlow for Quad<GlowTexture> {
    fn with_glow_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, glow_color: [f32; 4], material: Rc<GlowTexture>) -> Self {
        Self::new(position, size, z_index, material, ([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], texture, glow_color))
    }

    fn with_glow_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, glow_color: [f32; 4], material: Rc<GlowTexture>) -> Self {
        Self::new(position, size, z_index, material, ([
            [sprite.left(),  sprite.top()   ],
            [sprite.right(), sprite.top()   ],
            [sprite.right(), sprite.bottom()],
            [sprite.left(),  sprite.bottom()]
        ], sprite.texture(), glow_color))
    }

    fn light_color(&self) -> [f32; 4] {
        self.params().2
    }
}

pub type WaterTexture = FlatTexture;

struct WaterShader(Rc<shader::Program>);
struct WaterMaterial(Rc<WaterTexture>);

fn create_water_shader() -> WaterShader {
    WaterShader(Rc::new(shader::Program::from_source(include_str!("../../assets/water-shader.glsl")).expect_log()))
}

pub fn water_material(store: &mut DataStore) -> Rc<WaterTexture> {
    let shader = store.get_or_create(create_water_shader).0.clone();
    store.get_or_create(|| WaterMaterial(Rc::new(WaterTexture::with_shader(shader)))).0.clone()
}

pub fn water_shader(store: &mut DataStore) -> Rc<shader::Program> {
    store.get_or_create(create_water_shader).0.clone()
}

pub trait WithWater {
    fn with_water_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, material: Rc<WaterTexture>) -> Self;
    fn with_water_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, material: Rc<WaterTexture>) -> Self;
}

impl WithWater for Quad<WaterTexture> {
    fn with_water_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID, material: Rc<WaterTexture>) -> Self {
        Self::new(position, size, z_index, material, ([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], texture))
    }

    fn with_water_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, material: Rc<WaterTexture>) -> Self {
        Self::new(position, size, z_index, material, ([
            [sprite.left(),  sprite.top()   ],
            [sprite.right(), sprite.top()   ],
            [sprite.right(), sprite.bottom()],
            [sprite.left(),  sprite.bottom()]
        ], sprite.texture()))
    }
}