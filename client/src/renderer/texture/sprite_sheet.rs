use std::ops::Div;

use aeonetica_engine::util::vector::Vector2;

use super::{Texture, TexCoordFormat, RenderID};

#[derive(Debug, Clone)]
pub struct Sprite {
    texture_id: RenderID,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Sprite {
    pub fn new(texture_id: RenderID, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            texture_id,
            left,
            right,
            bottom,
            top
        }
    }

    pub fn texture(&self) -> RenderID {
        self.texture_id
    }

    pub fn into_tex_coords(&self, format: TexCoordFormat) -> [[f32; 2]; 4] {
        match format {
            TexCoordFormat::LeftRightTopBottom => [
                [self.left, self.top],
                [self.right, self.top],
                [self.left, self.bottom],
                [self.right, self.bottom]
            ],
            TexCoordFormat::LeftRightBottomTop => [
                [self.left, self.bottom],
                [self.right, self.bottom],
                [self.left, self.top],
                [self.right, self.top]
            ],
            TexCoordFormat::RightLeftTopBottom => [
                [self.right, self.top],
                [self.left, self.top],
                [self.right, self.bottom],
                [self.left, self.bottom]
            ],
            TexCoordFormat::RightLeftBottomTop => [
                [self.right, self.bottom],
                [self.left, self.bottom],
                [self.right, self.top],
                [self.left, self.top]
            ]
        }
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn right(&self) -> f32 {
        self.right
    }
}

pub struct SpriteSheet {
    texture: Texture,
    sprite_size: Vector2<u32>,
    num_sprites: u32,
}

impl SpriteSheet {
    pub fn from_texture(texture: Texture, sprite_size: Vector2<u32>) -> Result<Self, String> {
        if texture.size().x() % sprite_size.x() != 0 ||
            texture.size().y() % sprite_size.y() != 0 {
            Err(format!("Texture size is not multiple of sprite size. ({:?} | {:?})", texture.size(), sprite_size))
        }
        else {
            let num_sprites = (*texture.size()).div(sprite_size);
            Ok(Self {
                texture,
                sprite_size,
                num_sprites: num_sprites.x() * num_sprites.y()
            })
        }
    }

    pub fn sprite_size(&self) -> &Vector2<u32> {
        &self.sprite_size
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn num_sprites(&self) -> u32 {
        self.num_sprites
    }

    pub fn get(&self, idx: u32) -> Option<Sprite> {
        if idx >= self.num_sprites {
            return None;
        }

        let total_x = self.texture.size().x();
        let total_y = self.texture.size().y();

        let x_idx = idx % (total_x / self.sprite_size.x());
        let y_idx = idx / (total_x / self.sprite_size.x());

        let left_x = x_idx * self.sprite_size.x();
        let right_x = (x_idx + 1) * self.sprite_size.x();
        let top_y = y_idx * self.sprite_size.y();
        let bottom_y = (y_idx + 1) * self.sprite_size.y();

        let total_x = total_x as f32;
        let total_y = total_y as f32;

        Some(Sprite {
            texture_id: self.texture.id(),
            left: (left_x as f32 / total_x),
            right: (right_x as f32 / total_x),
            top: (top_y as f32 / total_y),
            bottom: (bottom_y as f32 / total_y)
        })
    }
}

