use core::f32;


use super::*;

const QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

pub trait Quad {
    type Layout;

    fn layout<'a>() -> &'a Rc<BufferLayout>;
    
    fn position(&self) -> &Vector2<f32>;
    fn size(&self) -> &Vector2<f32>;
    fn z_index(&self) -> u8;
    fn rotation(&self) -> f32;

    fn set_position(&mut self, position: Vector2<f32>);
    fn set_size(&mut self, size: Vector2<f32>);
    fn set_rotation(&mut self, rotation: f32);

    fn shader(&self) -> &shader::Program;

    fn recalculate_vertex_data(&mut self);

    fn is_dirty(&self) -> bool;

    fn rotate_edges(&self) -> [(f32, f32); 4] {
        let center = *self.position() + *self.size() / Vector2::new(2.0, 2.0);

        let theta = self.rotation();
        [
            (-(*self.size()).half()),
            ((*self.size()).half() * Vector2::new(1.0, -1.0)),
            (*self.size()).half(),
            ((*self.size()).half() * Vector2::new(-1.0, 1.0)),
        ].map(|v| (v.rotate(theta) + center).into())
    }
}

#[derive(Clone)]
pub struct ColoredQuad {
    position: Vector2<f32>,
    size: Vector2<f32>,
    rotation: f32,
    z_index: u8,

    shader: Rc<shader::Program>,
    color: [f32; 4],
    vertices: Option<[VertexTuple2<[f32; 3], [f32; 4]>; 4]>,

    location: Option<VertexLocation>,
}

impl ColoredQuad {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, color: [f32; 4], shader: Rc<shader::Program>) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            z_index,
            color,
            shader,
            vertices: None,
            location: None
        }
    }

    pub fn color(&self) -> &[f32; 4] {
        &self.color
    }

    pub fn set_dirty(&mut self) {
        self.vertices = None;
    }
}

thread_local! {
    static COLORED_QUAD_LAYOUT: Box<Rc<BufferLayout>> = Box::new(Rc::new(<ColoredQuad as quad::Quad>::Layout::build()));
}

//const COLORED_QUAD_LAYOUT: BufferLayout = <ColoredQuad as quad::Quad>::Layout::build();

impl Quad for ColoredQuad {
    type Layout = BufferLayoutBuilder<(Vertex, Color)>;

    fn layout<'a>() -> &'a Rc<BufferLayout> {
        unsafe {
            let x: *const Rc<BufferLayout> = COLORED_QUAD_LAYOUT.with(|l| &**l as *const _);
            x.as_ref().unwrap_unchecked()
        }
    }

    fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    fn size(&self) -> &Vector2<f32> {
        &self.size
    }

    fn z_index(&self) -> u8 {
        self.z_index
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.vertices = None;
    }

    fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.vertices = None;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.vertices = None;
    }

    fn shader(&self) -> &shader::Program {
        &self.shader
    }

    fn recalculate_vertex_data(&mut self) {
        if self.rotation % f32::consts::TAU != 0.0 {
            let [(x1, y1), (x2, y2), (x3, y3), (x4, y4)] = self.rotate_edges();
            self.vertices = Some(Self::Layout::array([
                vertex!([x1, y1, 0.0], self.color),
                vertex!([x2, y2, 0.0], self.color),
                vertex!([x3, y3, 0.0], self.color),
                vertex!([x4, y4, 0.0], self.color)
            ]));
        }
        else {
            let [x, y]: [f32; 2] = self.position.into();
            let [w, h]: [f32; 2] = self.size.into(); 

            self.vertices = Some(Self::Layout::array([
                vertex!([x,     y,     0.0], self.color),
                vertex!([x + w, y,     0.0], self.color),
                vertex!([x + w, y + h, 0.0], self.color),
                vertex!([x,     y + h, 0.0], self.color)
            ]));
        }
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }
}

impl Renderable for ColoredQuad {
    fn vertex_data(&mut self) -> VertexData<'_> {
        if self.vertices.is_none() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::new(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            &Self::layout(),
            &self.shader,
            self.z_index
        )
    }

    fn texture_id(&self) -> Option<RenderID> {
        None
    }

    fn location(&self) -> &Option<VertexLocation> {
        &self.location
    }

    fn set_location(&mut self, location: VertexLocation) {
        self.location = Some(location);
    }
}

#[derive(Clone)]
pub struct TexturedQuad {
    position: Vector2<f32>,
    size: Vector2<f32>,
    z_index: u8,
    rotation: f32,

    shader: Rc<shader::Program>,
    texture_id: RenderID,
    vertices: Option<[VertexTuple3<[f32; 3], [f32; 2], Sampler2D>; 4]>,
    location: Option<VertexLocation>,
}

impl TexturedQuad {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture_id: RenderID, shader: Rc<shader::Program>) -> Self {
        Self {
            position,
            size,
            z_index,
            rotation: 0.0,
            texture_id,
            shader,
            vertices: None,
            location: None
        }
    }

    pub fn texture_id(&self) -> RenderID {
        self.texture_id
    }

    pub fn set_dirty(&mut self) {
        self.vertices = None;
    }
}

thread_local! {
    static TEXTURED_QUAD_LAYOUT: Box<Rc<BufferLayout>> = Box::new(Rc::new(<TexturedQuad as quad::Quad>::Layout::build()));
}

impl Quad for TexturedQuad {
    type Layout = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;

    fn layout<'a>() -> &'a Rc<BufferLayout> {
        unsafe {
            let x: *const Rc<BufferLayout> = TEXTURED_QUAD_LAYOUT.with(|l| &**l as *const _);
            x.as_ref().unwrap_unchecked()
        }
    }

    fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    fn size(&self) -> &Vector2<f32> {
        &self.size
    }

    fn z_index(&self) -> u8 {
        self.z_index
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.vertices = None;
    }

    fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.vertices = None;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.vertices = None;
    }

    fn shader(&self) -> &shader::Program {
        &self.shader
    }

    fn recalculate_vertex_data(&mut self) {
        if self.rotation % f32::consts::TAU != 0.0 {
            let [(x1, y1), (x2, y2), (x3, y3), (x4, y4)] = self.rotate_edges();
            self.vertices = Some(Self::Layout::array([
                vertex!([x1, y1, 0.0], [0.0, 0.0], Sampler2D(0)),
                vertex!([x2, y2, 0.0], [1.0, 0.0], Sampler2D(0)),
                vertex!([x3, y3, 0.0], [1.0, 1.0], Sampler2D(0)),
                vertex!([x4, y4, 0.0], [0.0, 1.0], Sampler2D(0))
            ]));
        }
        else {
            let [x, y]: [f32; 2] = self.position.into();
            let [w, h]: [f32; 2] = self.size.into(); 

            self.vertices = Some(Self::Layout::array([
                vertex!([x,     y,     0.0], [0.0, 0.0], Sampler2D(0)),
                vertex!([x + w, y,     0.0], [1.0, 0.0], Sampler2D(0)),
                vertex!([x + w, y + h, 0.0], [1.0, 1.0], Sampler2D(0)),
                vertex!([x,     y + h, 0.0], [0.0, 1.0], Sampler2D(0))
            ]));
        }
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }
}

impl Renderable for TexturedQuad {
    fn vertex_data(&mut self) -> VertexData<'_> {
        if self.is_dirty() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::new_textured(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            &Self::layout(), 
            &self.shader,
            self.z_index,
            self.texture_id
        )
    }

    fn texture_id(&self) -> Option<RenderID> {
        Some(self.texture_id)
    }

    fn location(&self) -> &Option<VertexLocation> {
        &self.location
    }

    fn set_location(&mut self, location: VertexLocation) {
        self.location = Some(location);
    }
}

#[derive(Clone)]
pub struct SpriteQuad {
    position: Vector2<f32>,
    size: Vector2<f32>,
    z_index: u8,
    rotation: f32,

    shader: Rc<shader::Program>,
    sprite: Sprite,
    vertices: Option<[VertexTuple3<[f32; 3], [f32; 2], Sampler2D>; 4]>,

    location: Option<VertexLocation>,
}

impl SpriteQuad {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, shader: Rc<shader::Program>) -> Self {
        Self {
            position,
            size,
            z_index,
            rotation: 0.0,
            sprite,
            shader,
            vertices: None,
            location: None
        }
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn set_dirty(&mut self) {
        self.vertices = None;
    }

    pub fn set_sprite(&mut self, sprite: Sprite) -> Result<(), ()> {
        if sprite.texture() == self.sprite.texture() {
            self.sprite = sprite;
            self.vertices = None;
            Ok(())
        }
        else {
            Err(())
        }
    }
}

thread_local! {
    static SPRITE_QUAD_LAYOUT: Box<Rc<BufferLayout>> = Box::new(Rc::new(<SpriteQuad as quad::Quad>::Layout::build()));
}

impl Quad for SpriteQuad {
    type Layout = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;

    fn layout<'a>() -> &'a Rc<BufferLayout> {
        unsafe {
            let x: *const Rc<BufferLayout> = TEXTURED_QUAD_LAYOUT.with(|l| &**l as *const _);
            x.as_ref().unwrap_unchecked()
        }
    }

    fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    fn size(&self) -> &Vector2<f32> {
        &self.size
    }

    fn z_index(&self) -> u8 {
        self.z_index
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.vertices = None;
    }

    fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.vertices = None;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.vertices = None;
    }

    fn shader(&self) -> &shader::Program {
        &self.shader
    }

    fn recalculate_vertex_data(&mut self) {
        if self.rotation % f32::consts::TAU != 0.0 {
            let [(x1, y1), (x2, y2), (x3, y3), (x4, y4)] = self.rotate_edges();
            self.vertices = Some(Self::Layout::array([
                vertex!([x1, y1, 0.0], [self.sprite.left(),  self.sprite.top()   ], Sampler2D(0)),
                vertex!([x2, y2, 0.0], [self.sprite.right(), self.sprite.top()   ], Sampler2D(0)),
                vertex!([x3, y3, 0.0], [self.sprite.right(), self.sprite.bottom()], Sampler2D(0)),
                vertex!([x4, y4, 0.0], [self.sprite.left(),  self.sprite.bottom()], Sampler2D(0))
            ]));
        }
        else {
            let [x, y]: [f32; 2] = self.position.into();
            let [w, h]: [f32; 2] = self.size.into(); 

            self.vertices = Some(Self::Layout::array([
                vertex!([x,     y,     0.0], [self.sprite.left(),  self.sprite.top()   ], Sampler2D(0)),
                vertex!([x + w, y,     0.0], [self.sprite.right(), self.sprite.top()   ], Sampler2D(0)),
                vertex!([x + w, y + h, 0.0], [self.sprite.right(), self.sprite.bottom()], Sampler2D(0)),
                vertex!([x,     y + h, 0.0], [self.sprite.left(),  self.sprite.bottom()], Sampler2D(0))
            ]));
        }
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }
}

impl Renderable for SpriteQuad {
    fn vertex_data(&mut self) -> VertexData<'_> {
        if self.is_dirty() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::new_textured(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            &Self::layout(), 
            &self.shader,
            self.z_index,
            self.sprite.texture()
        )
    }

    fn texture_id(&self) -> Option<RenderID> {
        Some(self.sprite.texture())
    }

    fn location(&self) -> &Option<VertexLocation> {
        &self.location
    }

    fn set_location(&mut self, location: VertexLocation) {
        self.location = Some(location);
    }
}
