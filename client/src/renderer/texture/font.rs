use std::collections::HashMap;
use aeonetica_engine::error::ErrorResult;
use aeonetica_engine::error::builtin::{IOError, DataError};
use aeonetica_engine::nanoserde::{DeRon, SerRon};
use aeonetica_engine::nanoserde;

use aeonetica_engine::util::vector::Vector2;

use super::*;

#[derive(Debug, SerRon, DeRon)]
struct BMPFontFile {
    texture: String,
    monospaced: bool,
    char_size: (u32, u32),
    characters: HashMap<String, u32>
}

pub struct BitmapFont {
    char_size: Vector2<u32>,
    sprite_sheet: SpriteSheet,
    characters: HashMap<char, u32>,
    widths: Vec<u32>
}

impl BitmapFont {
    pub fn from_texture_and_fontdata(texture: Texture, fontdata: &str) -> ErrorResult<Self> {
        let font_data = BMPFontFile::deserialize_ron(fontdata).map_err(|e| Error::new(IOError(e.to_string()), Fatality::DEFAULT, true))?;
        Self::from_texture(texture,
                           font_data.char_size.into(),
                           font_data.characters.into_iter().map(|(k, v)| {
                               let c: Vec<_> = k.chars().collect();
                               if c.len() != 1 {
                                   return Err(Error::new(DataError(format!("key '{}' in font data las length {}, expected 1 char", k, c.len())), Fatality::DEFAULT, true))
                               }
                               let c = c[0];
                               Ok((c, v))
                           }).collect::<ErrorResult<HashMap<char, u32>>>()?,
                           font_data.monospaced
        )
    }

    pub fn from_texture(texture: Texture, char_size: Vector2<u32>, characters: HashMap<char, u32>, monospaced: bool) -> ErrorResult<Self> {
        let mut widths = vec![];
        let pix = texture.size().area() as usize;
        let bytes_per_pixel = texture.bytes_per_pixel() as usize;
        let mut buffer = vec![0u8; pix * bytes_per_pixel];

        unsafe {
            gl::GetTextureSubImage(texture.id(), 0, 0, 0, 0, texture.size().x as i32, texture.size().y as i32, 1, texture.data_format(), gl::UNSIGNED_BYTE, buffer.len() as i32, buffer.as_mut_ptr() as * mut _)
        }

        let [w, h]: [u32; 2] = (*texture.size() / char_size).into();
        if monospaced {
            for _ in 0..h {
                for _ in 0..w {
                    widths.push(char_size.x)
                }
            }
        } else {
            for y in 0..h {
                for x in 0..w {
                    'l: {
                        for dx in (0..char_size.x).rev() {
                            for dy in 0..char_size.y {
                                let p = ((y * char_size.y + dy) * texture.size().x + (x * char_size.x + dx)) as usize;
                                if buffer[p * bytes_per_pixel] > 0 || buffer[p * bytes_per_pixel + 1] > 0 || buffer[p * bytes_per_pixel + 2] > 0 {
                                    widths.push(dx);
                                    break 'l
                                }
                            }
                        }
                        widths.push(char_size.x)
                    }
                }
            }
        }

        Ok(Self {
            char_size,
            sprite_sheet: SpriteSheet::from_texture(texture, char_size)?,
            widths,
            characters
        })
    }

    pub fn char_index(&self, c: char) -> Option<&u32> {
        self.characters.get(&c)
    }

    pub fn index_width(&self, i: u32) -> u32 {
        self.widths[i as usize]
    }

    pub fn sprite_sheet(&self) -> &SpriteSheet {
        &self.sprite_sheet
    }

    pub fn char_size(&self) -> Vector2<f32> {
        [self.char_size.x as f32, self.char_size.y as f32].into()
    }

    pub fn index_str(&self, string: &str) -> Option<Vec<u32>> {
        let mut indices = Vec::with_capacity(string.len());
        for c in string.chars() {
            indices.push(*self.characters.get(&c)?);
        }
        Some(indices)
    }
}
