use aeonetica_engine::error::*;

use crate::get_gl_str;

#[derive(Debug)]
pub struct GLError(pub String, pub u32);

impl GLError {
    #[cfg(debug_assertions)]
    pub fn from_gl_errno() -> Self {
        let errno = unsafe { gl::GetError() };
        Self(get_gl_str!(errno, "error fetching OpenGL errno value").to_string(), errno)
    }

    // checking for errors is slloowwww
    #[cfg(not(debug_assertions))]
    pub fn from_gl_errno() -> Self {
        Self("internal opengl error".to_string(), errno)
    }
}

impl ErrorValue for GLError {}

impl IntoError for GLError {
    fn into_error(self) -> Box<Error> {
        Error::new(self, Fatality::DEFAULT, true)
    }
}

impl std::fmt::Display for GLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenGL error: {}", self.0)?;
        if self.1 != 0 {
            write!(f, " ({})", self.1)?;
        }
        Ok(())
    }
}