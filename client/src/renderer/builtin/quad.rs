use core::f32;

use crate::renderer::{*, material::{Material, FlatColor, FlatTexture}};

const QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

pub struct Quad<M: Material> {
    position: Vector2<f32>,
    size: Vector2<f32>,
    rotation: f32,
    z_index: u8,

    material: Rc<M>,
    vertices: Option<[M::VertexTuple; 4]>,
    params: M::Data<4>,

    location: Option<VertexLocation>
}

impl Quad<FlatColor> {
    pub fn with_color(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, color: [f32; 4]) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            z_index,
            material: FlatColor::get(),
            vertices: None,
            params: color,
            location: None
        }
    } 
}

impl Quad<FlatTexture> {
    pub fn with_texture(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, texture: RenderID) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            z_index,
            material: FlatTexture::get(),
            vertices: None,
            params: ([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], texture),
            location: None
        }
    }

    pub fn with_sprite(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, sprite: Sprite) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            z_index,
            material: FlatTexture::get(),
            vertices: None,
            params: ([
                [sprite.left(),  sprite.top()   ],
                [sprite.right(), sprite.top()   ],
                [sprite.right(), sprite.bottom()],
                [sprite.left(),  sprite.bottom()]
            ], sprite.texture()),
            location: None
        }
    }

    pub fn uv_coords(&self) -> [[f32; 2]; 4] {
        self.params.0
    }

    pub fn set_uv_coords(&mut self, uv_coords: [[f32; 2]; 4]) {
        self.params.0 = uv_coords;
        self.vertices = None;
    }

    pub fn set_sprite(&mut self, sprite: Sprite) -> Result<(), ()> {
        if sprite.texture() == self.params.1 {
            self.params.0 = [
                [sprite.left(),  sprite.top()   ],
                [sprite.right(), sprite.top()   ],
                [sprite.right(), sprite.bottom()],
                [sprite.left(),  sprite.bottom()]
            ];
            self.vertices = None;
            Ok(())
        }
        else {
            Err(())
        }
    }
}

impl<M: Material> Quad<M> {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, z_index: u8, material: Rc<M>, params: M::Data<4>) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            z_index,
            material,
            vertices: None,
            params,
            location: None
        }
    }

    pub fn params(&self) -> &M::Data<4> {
        &self.params
    }

    pub fn set_dirty(&mut self) {
        self.vertices = None;
    }

    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    pub fn size(&self) -> &Vector2<f32> {
        &self.size
    }

    pub fn z_index(&self) -> u8 {
        self.z_index
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.set_dirty();
    }

    pub fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
        self.set_dirty();
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.set_dirty();
    }

    pub fn shader(&self) -> &shader::Program {
        self.material.shader()
    }

    fn recalculate_vertex_data(&mut self) {
        if self.rotation % f32::consts::TAU != 0.0 {
            let [(x1, y1), (x2, y2), (x3, y3), (x4, y4)] = self.rotate_edges();
            self.vertices = Some(self.material.vertices([
                    [x1, y1],
                    [x2, y2],
                    [x3, y3],
                    [x4, y4]
                ], 
                &self.params
            ));
        }
        else {
            let [x, y]: [f32; 2] = self.position.into();
            let [w, h]: [f32; 2] = self.size.into(); 

            self.vertices = Some(self.material.vertices([
                    [x,     y,   ],
                    [x + w, y,   ],
                    [x + w, y + h],
                    [x,     y + h]
                ], 
                &self.params
            ));
        }
    }

    pub fn rotate_edges(&self) -> [(f32, f32); 4] {
        let half_size = self.size.half();
        let center = self.position + half_size;

        let theta = self.rotation();
        [
            -half_size,
            half_size * Vector2::new(1.0, -1.0),
            half_size,
            half_size * Vector2::new(-1.0, 1.0),
        ].map(|v| (v.rotate(theta) + center).into())
    }
}

impl<M: Material> Renderable for Quad<M> {
    fn vertex_data(&mut self) -> VertexData<'_> {
        if self.is_dirty() {
            self.recalculate_vertex_data();
        }

        let vertices = self.vertices.as_ref().unwrap();

        VertexData::from_material(
            util::to_raw_byte_slice!(vertices),
            QUAD_INDICES.as_slice(),
            &self.material,
            &self.params,
            self.z_index
        )
    }

    fn texture_id(&self) -> Option<RenderID> {
        M::texture_id(&self.params)
    }

    fn location(&self) -> &Option<VertexLocation> {
        &self.location
    }

    fn set_location(&mut self, location: Option<VertexLocation>) {
        self.location = location;
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }

    fn has_location(&self) -> bool {
        self.location.is_some()
    }
}
