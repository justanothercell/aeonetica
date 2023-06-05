use std::array;

use aeonetica_engine::{math::vector::Vector2, error::builtin::DataError, util::generic_assert::{Assert, IsTrue}};

use crate::renderer::{*, material::{Material, FlatTexture}};

pub struct TextArea<const N: usize, const L: usize>
    where Assert<{L * 4 == N}>: IsTrue
{
    position: Vector2<f32>,
    z_index: u8,

    content: [char; L],
    font: Rc<BitmapFont>,
    font_size: f32,
    spacing: f32,
    
    material: Rc<FlatTexture>,
    vertices: Option<[<FlatTexture as Material>::VertexTuple; N]>,
    params: <FlatTexture as Material>::Data<N>,
    indices: Vec<u32>,

    location: Option<VertexLocation>
}

impl<const N: usize, const L: usize> Renderable for TextArea<N, L>
    where Assert<{L * 4 == N}>: IsTrue
{
    fn has_location(&self) -> bool {
        self.location.is_some()
    }

    fn is_dirty(&self) -> bool {
        self.vertices.is_none()
    }

    fn location(&self) -> &Option<VertexLocation> {
        &self.location
    }

    fn set_location(&mut self, location: Option<VertexLocation>) {
        self.location = location;
    }

    fn texture_id(&self) -> Option<RenderID> {
        FlatTexture::texture_id(&self.params)
    }

    fn vertex_data(&mut self) -> VertexData {
        if self.is_dirty() {
            self.recalculate_vertex_data();
        }

        let vertices  = self.vertices.as_ref().unwrap();
        VertexData::from_material(
            util::to_raw_byte_slice!(vertices), 
            self.indices.as_slice(), 
            &self.material,
            &self.params,
            self.z_index
        )
    }
}


impl<const N: usize, const L: usize> TextArea<N, L>
    where Assert<{L * 4 == N}>: IsTrue
{
    pub fn with_string<S: Into<String>>(position: Vector2<f32>, z_index: u8, font_size: f32, spacing: f32, font: Rc<BitmapFont>, material: Rc<FlatTexture>, string: S) -> ErrorResult<Self> {
        let string = string.into();
        let mut chars = string.chars();
        let content = array::from_fn(|_| chars.next().unwrap_or(' '));
        
        let mut params = material.default_data();
        params.1 = font.sprite_sheet().texture().id();

        let indices = Self::gen_indices();
        if indices.len() as u32 > Batch::MAX_BATCH_INDEX_COUNT {
            Err(Error::new(DataError(format!("TextArea string is too long. {} excessive characters.", (indices.len() as u32 - Batch::MAX_BATCH_INDEX_COUNT) / 6 + 1)), Fatality::DEFAULT, false))
        }
        else {
            Ok(Self {
                position,
                z_index,
                content,
                font,
                font_size,
                material,
                params,
                spacing,
                location: None,
                vertices: None,
                indices
            })
        }
    }

    fn gen_indices() -> Vec<u32> {
        let mut indices = Vec::with_capacity(N * 6);
        for i in 0 .. L {
            let i = i as u32 * 4;
            indices.extend_from_slice(&[i, i + 1, i + 2, i + 2, i + 3, i])
        }
        indices
    }

    pub fn len(&self) -> usize {
        L
    }

    pub fn string(&self) -> String {
        self.content.iter().collect::<String>()
    }

    // Sets the content of the text area. Fails if string length exceeds max_len
    pub fn set_string<S: Into<String>>(&mut self, string: S) {
        let string = string.into();
        let mut chars = string.chars();
        self.content.iter_mut().for_each(|c| *c = chars.next().unwrap_or(' '));
        self.set_dirty();
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.set_dirty();
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.set_dirty();
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
        self.set_dirty();
    }

    fn set_dirty(&mut self) {
        self.vertices = None;
    }

    pub fn recalculate_vertex_data(&mut self) {
        let size = self.font_size / self.font.char_size().y;
        let half_size = (self.font.char_size() * size).half();

        let mut x_offset = 0.0;
        
        let mut next_char = |i, c| {
            let position = Vector2::new(x_offset, self.position.y());

            let char_idx = self.font.char_index(c);
            if char_idx.is_none() {
                panic!(); // todo: error handling
            }
            let char_idx = *char_idx.unwrap();

            let width = self.font.index_width(char_idx) as f32;
            x_offset += width * size + self.spacing;

            let char_sprite = self.font.sprite_sheet().get(char_idx);
            if char_sprite.is_none() {
                panic!(); // todo: error
            }
            let char_sprite = char_sprite.unwrap();
            let i = i * 4;

            self.params.0[i]     = [char_sprite.left(),  char_sprite.top()   ];
            self.params.0[i + 1] = [char_sprite.right(), char_sprite.top()   ];
            self.params.0[i + 2] = [char_sprite.right(), char_sprite.bottom()];
            self.params.0[i + 3] = [char_sprite.left(),  char_sprite.bottom()];

            return self.material.vertices([
                [position.x() - half_size.x(), position.y() - half_size.y()],
                [position.x() + half_size.x(), position.y() - half_size.y()],
                [position.x() + half_size.x(), position.y() + half_size.y()],
                [position.x() - half_size.x(), position.y() + half_size.y()]
            ], &self.material.data_slice(&self.params, i));
        };

        let mut current_char = next_char(0, self.content[0]);

        self.vertices = Some(array::from_fn(|i| {
            if i % 4 == 0 && i != 0 {
                current_char = next_char(i / 4, self.content[i / 4]);
            }

            current_char[i % 4].clone()
        }))
    }
}
