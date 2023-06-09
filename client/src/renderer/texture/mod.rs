pub mod sprite_sheet;
pub use sprite_sheet::*;

pub mod font;

use aeonetica_engine::{math::vector::Vector2, error::ErrorResult};
use image::{io::Reader as ImageReader, DynamicImage};

use aeonetica_engine::error::{IntoError, Error, ErrorValue, Fatality};

use super::{RenderID, glerror::GLError};

#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Sampler2D(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
    RgbaU8  = gl::RGBA8  as isize,
    RgbU8   = gl::RGB8   as isize,
    RgbaF16 = gl::RGB16F as isize
}

impl Format {
    fn base_type(&self) -> gl::types::GLenum {
        match self {
            Self::RgbU8 | Self::RgbaU8 => gl::UNSIGNED_BYTE,
            Self::RgbaF16 => gl::FLOAT
        }
    }

    fn internal(&self) -> gl::types::GLenum {
        *self as gl::types::GLenum
    }

    fn data(&self) -> gl::types::GLenum {
        match self {
            Self::RgbU8 => gl::RGB,
            Self::RgbaU8 | Self::RgbaF16 => gl::RGBA
        }
    }
}

pub enum TexCoordFormat {
    LeftRightTopBottom,
    RightLeftTopBottom,
    LeftRightBottomTop,
    RightLeftBottomTop,
    // TODO: implement remaining
}

#[derive(Debug)]
pub enum ImageError {
    Io(std::io::Error),
    Decode(String),
    Unsupported(String),
}

impl IntoError for ImageError {
    fn into_error(self) -> Box<Error> {
        Error::new(self, Fatality::DEFAULT, true)
    }
}

impl ErrorValue for ImageError {

}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => f.write_str(format!("ImageError: IO error: {err}").as_str()),
            Self::Decode(err) => f.write_str(format!("ImageError: Decode error: {err}").as_str()),
            Self::Unsupported(err) => f.write_str(format!("ImageError: Unsupported error: {err}").as_str()),
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<image::ImageError> for ImageError {
    fn from(value: image::ImageError) -> Self {
        Self::Decode(value.to_string())
    }
}

pub struct Texture {
    id: RenderID,
    size: Vector2<u32>,
    internal_format: gl::types::GLenum,
    data_format: gl::types::GLenum
}

impl Texture {
    pub fn from_bytes(bytes: &[u8]) -> ErrorResult<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let img = ImageReader::new(cursor)
            .with_guessed_format()?
            .decode().map_err(|e| ImageError::Decode(e.to_string()).into_error())?;
         //   .flipv();
        Self::load(img)
    }

    pub fn from_file(img_path: &str) -> ErrorResult<Self> {
        let img = ImageReader::open(img_path)?
            .decode().map_err(|e| ImageError::Decode(e.to_string()).into_error())?;
          //  .flipv();
        Self::load(img)
    }

    pub fn data_format(&self) -> gl::types::GLenum {
        self.data_format
    }

    fn load(img: DynamicImage) -> ErrorResult<Self> {
        let mut t = Self {
            id: 0,
            size: (img.width(), img.height()).into(),
            internal_format: 0,
            data_format: 0
        };

        match img {
            image::DynamicImage::ImageRgb8(_) => {
                t.internal_format = gl::RGB8;
                t.data_format = gl::RGB;
            }
            image::DynamicImage::ImageRgba8(_) => {
                t.internal_format = gl::RGBA8;
                t.data_format = gl::RGBA;
            }
            _ => return Err(ImageError::Unsupported(format!("Image format {img:?} is unsupported")).into_error())
        }

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut t.id);
            if t.id == 0 {
                let mut err = GLError::from_gl_errno().into_error();
                err.add_info("error creating opengl texture");
                return Err(err);
            }
            gl::TextureStorage2D(t.id, 1, t.internal_format, t.size.x() as i32, t.size.y() as i32);

            gl::TextureParameteri(t.id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(t.id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TextureParameteri(t.id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(t.id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TextureSubImage2D(t.id, 0, 0, 0, t.size.x() as i32, t.size.y() as i32, t.data_format, gl::UNSIGNED_BYTE, img.into_bytes().as_ptr() as *const _)
        }

        Ok(t)
    }

    pub fn create(size: Vector2<u32>, format: Format) -> Self {
        let mut t = Self {
            id: 0,
            size,
            internal_format: format.internal(),
            data_format: format.data()
        };

        unsafe {
            gl::GenTextures(1, &mut t.id);
            gl::BindTexture(gl::TEXTURE_2D, t.id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, t.internal_format as i32, size.x() as i32, size.y() as i32, 0, t.data_format, format.base_type(), std::ptr::null());
        }

        t
    }

    pub fn set_data(&self, data: &[u8]) {
        let bytes_per_pixel = match self.data_format {
            gl::RGBA => 4,
            gl::RGB => 3,
            _ => panic!("unsupported texture data format {}", self.data_format)
        };

        assert_eq!(data.len() as u32, self.size.x() * self.size.y() * bytes_per_pixel, "wrong pixel data size for texture");
        unsafe {
            gl::TextureSubImage2D(
                self.id, 
                0, 
                0, 0, 
                self.size.x() as i32, self.size.y() as i32,
                self.data_format, gl::UNSIGNED_BYTE, 
                data.as_ptr() as *const _
            );
        }
    }

    pub fn bind(&self, slot: u32) {
        unsafe { gl::BindTextureUnit(slot, self.id); }
    }

    pub fn id(&self) -> RenderID {
        self.id
    }

    pub fn delete(&mut self) {
        if self.id != 0 {
            unsafe { gl::DeleteTextures(1, &self.id); }
            self.id = 0;
        }
    }

    pub fn size(&self) -> &Vector2<u32> {
        &self.size
    }

    pub fn bytes_per_pixel(&self) -> u32 {
        match self.data_format {
            gl::RGBA => 4,
            gl::RGB => 3,
            _ => panic!("unsupported pixel layout")
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.delete();
    }
}