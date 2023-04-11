use aeonetica_engine::{util::vector::Vector2, log};
use image::{io::Reader as ImageReader, DynamicImage};

use super::RenderID;

#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Sampler2D(pub i32);

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
    OpenGL()
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => f.write_str(format!("ImageError: IO error: {err}").as_str()),
            Self::Decode(err) => f.write_str(format!("ImageError: Decode error: {err}").as_str()),
            Self::Unsupported(err) => f.write_str(format!("ImageError: Unsupported error: {err}").as_str()),
            Self::OpenGL() => f.write_str("ImageError: OpenGL error")
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
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ImageError> {
        let cursor = std::io::Cursor::new(bytes);
        let img = ImageReader::new(cursor)
            .with_guessed_format()?
            .decode()?;
         //   .flipv();
        Self::load(img)
    }

    pub fn from_file(img_path: &str) -> Result<Self, ImageError> {
        let img = ImageReader::open(img_path)?
            .decode()?;
          //  .flipv();
        Self::load(img)
    }

    pub fn data_format(&self) -> gl::types::GLenum {
        self.data_format
    }

    fn load(img: DynamicImage) -> Result<Self, ImageError> {
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
                log!("only rgb  :(");
            }
            image::DynamicImage::ImageRgba8(_) => {
                t.internal_format = gl::RGBA8;
                t.data_format = gl::RGBA;
            }
            _ => return Err(ImageError::Unsupported(format!("Image format {img:?} is unsupported")))
        }

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut t.id);
            if t.id == 0 {
                return Err(ImageError::OpenGL());
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

    pub fn create(size: Vector2<u32>) -> Self {
        let mut t = Self {
            id: 0,
            size,
            internal_format: gl::RGB,
            data_format: gl::RGB
        };

        unsafe {
            gl::GenTextures(1, &mut t.id);
            gl::BindTexture(gl::TEXTURE_2D, t.id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, size.x() as i32, size.y() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
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

    pub fn delete(self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }

    pub fn size(&self) -> &Vector2<u32> {
        &self.size
    }
}