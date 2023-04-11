use super::*;

const QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

pub trait Quad {
    type Layout;
    fn layout() -> Rc<BufferLayout>;

    fn position(&self) -> &Vector2<f32>;
    fn size(&self) -> &Vector2<f32>;
    fn z_index(&self) -> u8;

    fn set_position(&mut self, position: Vector2<f32>);
    fn set_size(&mut self, size: Vector2<f32>);

    fn shader(&self) -> &shader::Program;

    fn recalculate_vertex_data(&mut self);

    fn is_dirty(&self) -> bool;
}

#[derive(Clone)]
pub struct ColoredQuad {
    position: Vector2<f32>,
    size: Vector2<f32>,
    z_index: u8,

    shader: shader::Program,
    color: [f32; 4],
    vertices: Option<[VertexTuple2<[f32; 3], [f32; 4]>; 4]>
}

impl ColoredQuad {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, color: [f32; 4], shader: shader::Program) -> Self {
        Self {
            position,
            size,
            z_index,
            color,
            shader,
            vertices: None
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
    static COLORED_QUAD_LAYOUT: Rc<BufferLayout> = Rc::new(<ColoredQuad as quad::Quad>::Layout::build());
}

impl Quad for ColoredQuad {
    type Layout = BufferLayoutBuilder<(Vertex, Color)>;

    fn layout() -> Rc<BufferLayout> {
        COLORED_QUAD_LAYOUT.with(|layout| layout.clone())
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

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.vertices = None;
    }

    fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.vertices = None;
    }

    fn shader(&self) -> &shader::Program {
        &self.shader
    }

    fn recalculate_vertex_data(&mut self) {
        let [x, y]: [f32; 2] = self.position.into();
        let [w, h]: [f32; 2] = self.size.into(); 

        self.vertices = Some(Self::Layout::array([
            vertex!([x,     y,     0.0], self.color),
            vertex!([x + w, y,     0.0], self.color),
            vertex!([x + w, y + h, 0.0], self.color),
            vertex!([x,     y + h, 0.0], self.color)
        ]));
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }
}

impl Renderable for ColoredQuad {
    fn vertex_data<'a>(&'a mut self) -> VertexData<'a> {
        if self.vertices.is_none() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::new(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            Self::layout(),
            self.shader.clone(),
            self.z_index
        )
    }
}

#[derive(Clone)]
pub struct TexturedQuad {
    position: Vector2<f32>,
    size: Vector2<f32>,
    z_index: u8,

    shader: shader::Program,
    texture_id: RenderID,
    vertices: Option<[VertexTuple3<[f32; 3], [f32; 2], Sampler2D>; 4]>
}

impl TexturedQuad {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture_id: RenderID, shader: shader::Program) -> Self {
        Self {
            position,
            size,
            z_index,
            texture_id,
            shader,
            vertices: None
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
    static TEXTURED_QUAD_LAYOUT: Rc<BufferLayout> = Rc::new(<TexturedQuad as quad::Quad>::Layout::build());
}

impl Quad for TexturedQuad {
    type Layout = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;

    fn layout() -> Rc<BufferLayout> {
        TEXTURED_QUAD_LAYOUT.with(|layout| layout.clone())
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

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.vertices = None;
    }

    fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.vertices = None;
    }

    fn shader(&self) -> &shader::Program {
        &self.shader
    }

    fn recalculate_vertex_data(&mut self) {
        let [x, y]: [f32; 2] = self.position.into();
        let [w, h]: [f32; 2] = self.size.into(); 

        self.vertices = Some(Self::Layout::array([
            vertex!([x,     y,     0.0], [0.0, 0.0], Sampler2D(0)),
            vertex!([x + w, y,     0.0], [1.0, 0.0], Sampler2D(0)),
            vertex!([x + w, y + h, 0.0], [1.0, 1.0], Sampler2D(0)),
            vertex!([x,     y + h, 0.0], [0.0, 1.0], Sampler2D(0))
        ]));
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }
}

impl Renderable for TexturedQuad {
    fn vertex_data<'a>(&'a mut self) -> VertexData<'a> {
        if self.is_dirty() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::new_textured(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            Self::layout(), 
            self.shader.clone(),
            self.z_index,
            self.texture_id
        )
    }
}

#[derive(Clone)]
pub struct SpriteQuad {
    position: Vector2<f32>,
    size: Vector2<f32>,
    z_index: u8,

    shader: shader::Program,
    sprite: Sprite,
    vertices: Option<[VertexTuple3<[f32; 3], [f32; 2], Sampler2D>; 4]>
}

impl SpriteQuad {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite, shader: shader::Program) -> Self {
        Self {
            position,
            size,
            z_index,
            sprite,
            shader,
            vertices: None
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
    static SPRITE_QUAD_LAYOUT: Rc<BufferLayout> = Rc::new(<SpriteQuad as quad::Quad>::Layout::build());
}

impl Quad for SpriteQuad {
    type Layout = BufferLayoutBuilder<(Vertex, TexCoord, TextureID)>;

    fn layout() -> Rc<BufferLayout> {
        SPRITE_QUAD_LAYOUT.with(|layout| layout.clone())
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

    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.vertices = None;
    }

    fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.vertices = None;
    }

    fn shader(&self) -> &shader::Program {
        &self.shader
    }

    fn recalculate_vertex_data(&mut self) {
        let [x, y]: [f32; 2] = self.position.into();
        let [w, h]: [f32; 2] = self.size.into(); 

        self.vertices = Some(Self::Layout::array([
            vertex!([x,     y,     0.0], [self.sprite.left(),  self.sprite.top()   ], Sampler2D(0)),
            vertex!([x + w, y,     0.0], [self.sprite.right(), self.sprite.top()   ], Sampler2D(0)),
            vertex!([x + w, y + h, 0.0], [self.sprite.right(), self.sprite.bottom()], Sampler2D(0)),
            vertex!([x,     y + h, 0.0], [self.sprite.left(),  self.sprite.bottom()], Sampler2D(0))
        ]));
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }
}

impl Renderable for SpriteQuad {
    fn vertex_data<'a>(&'a mut self) -> VertexData<'a> {
        if self.is_dirty() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::new_textured(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            Self::layout(), 
            self.shader.clone(),
            self.z_index,
            self.sprite.texture()
        )
    }
}
