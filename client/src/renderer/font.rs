use std::collections::HashMap;

use aeonetica_engine::util::vector::Vector2;

use super::{sprite_sheet::SpriteSheet, texture::Texture};

pub struct BitmapFont {
    sprite_sheet: SpriteSheet,
    characters: HashMap<char, u32>,
    widths: Vec<u32>
}

impl BitmapFont {
    pub fn from_texture(texture: Texture, char_size: Vector2<u32>, characters: HashMap<char, u32>, monospaced: bool) -> Result<Self, String> {
        let mut widths = vec![];
        let pix = texture.size().area() as usize;
        let mut buffer = vec![0u8;pix*4];
        let [w, h]: [u32; 2] = (*texture.size() / char_size).into();
        for _ in 0..h {
            for _ in 0..w {
                widths.push(char_size.x)
            }
        }
        return Ok(Self {
            sprite_sheet: SpriteSheet::from_texture(texture, char_size)?,
            widths,
            characters
        });
        println!("a");
        println!(": {} == {}", texture.data_format(), gl::RGBA);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id());

            let mut width = 0.0;
            gl::GetTexLevelParameterfv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut width);
            let mut height = 0.0;
            gl::GetTexLevelParameterfv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut height);
            let mut align = 0.0;
            gl::GetTexLevelParameterfv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut align);
            let mut texbufoffset = 0;
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_BUFFER_OFFSET, &mut texbufoffset);
            let mut texbufsize = 0;
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_BUFFER_SIZE, &mut texbufsize);
            let mut maxtexbufsize = 0;
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::MAX_TEXTURE_BUFFER_SIZE, &mut maxtexbufsize);
            println!("size={width}, {height} align={align} texbufoffset={texbufoffset} texbufsize={texbufsize} maxtexbufsize={maxtexbufsize}");
            gl::GetTexImage(gl::TEXTURE_2D, 0, texture.data_format(), gl::UNSIGNED_INT, buffer.as_mut_ptr() as *mut _);
            //gl::GetnTexImage(gl::TEXTURE_2D, 0, texture.data_format(), gl::UNSIGNED_INT,  buffer.len() as i32, buffer.as_mut_ptr() as *mut _);
            //gl::GetTextureImage(texture.id(), 0, texture.data_format(), gl::UNSIGNED_INT, buffer.len() as i32, &mut buffer as *mut _ as *mut c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        println!("{buffer:?}");
        println!("b");
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
                    println!("{x} {y}");
                    'l: {
                        for dx in (0..char_size.x).rev() {
                            for dy in 0..char_size.y {
                                let p = ((y * char_size.y + dy) * texture.size().x + (x * char_size.x + dx)) as usize;
                                if buffer[p * 4] > 0 || buffer[p * 4 + 1] > 0 || buffer[p * 4 + 2] > 0 {
                                    widths.push(dx);
                                    println!("width: {dx}");
                                    break 'l
                                }
                            }
                        }
                        println!("width: 0");
                        widths.push(0)
                    }
                }
            }
        }

        Ok(Self {
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
