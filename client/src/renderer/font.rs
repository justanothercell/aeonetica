use std::collections::HashMap;

use aeonetica_engine::util::vector::Vector2;

use super::{sprite_sheet::SpriteSheet, texture::Texture};

pub struct BitmapFont {
    sprite_sheet: SpriteSheet,
    characters: HashMap<char, u32>
}

impl BitmapFont {
    pub fn from_texture(texture: Texture, char_size: Vector2<u32>, characters: HashMap<char, u32>) -> Result<Self, String> {
        Ok(Self {
            sprite_sheet: SpriteSheet::from_texture(texture, char_size)?,
            characters
        })
    }

    pub fn char_idx(&self, c: char) -> Option<&u32> {
        self.characters.get(&c)
    }

    pub fn sprite_sheet(&self) -> &SpriteSheet {
        &self.sprite_sheet
    }

    pub fn char_size(&self, size: f32) -> Vector2<f32> {
        (
            (self.sprite_sheet.sprite_size().x() as f32 / self.sprite_sheet.sprite_size().y as f32) * size,
            size,
        ).into()
    }

    pub fn index_str(&self, string: &str) -> Option<Vec<u32>> {
        let mut indices = Vec::with_capacity(string.len());
        for c in string.chars() {
            indices.push(*self.characters.get(&c)?);
        }
        Some(indices)
    }
}
