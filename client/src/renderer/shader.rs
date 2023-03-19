use std::collections::HashMap;
use super::*;

use regex::Regex;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub(super) enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize
}

pub struct Shader(RenderID);
impl Shader {
    pub(super) fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty)
            .ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }

    pub(super) fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(ty as gl::types::GLenum) };
        if shader != 0 {
            Some(Self(shader))
        }
        else {
            None
        }
    }

    pub(super) fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap())
            );
        }
    }

    pub(super) fn compile(&self) {
        unsafe { gl::CompileShader(self.0) }
    }

    pub(super) fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
        compiled == gl::TRUE.into()
    }

    pub(super) fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0;
        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast()
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub(super) fn delete(self) {
        unsafe { gl::DeleteShader(self.0) }
    }
}

pub struct Program(RenderID);
impl Program {
    pub(super) fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };
        if prog != 0 {
            Some(Self(prog))
        }
        else {
            None
        }
    }

    pub(super) fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.0, shader.0) }
    }

    pub(super) fn link(&self) {
        unsafe { gl::LinkProgram(self.0) }
    }

    pub(super) fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
        success == gl::TRUE.into()
    }

    pub(super) fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast()
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub(super) fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) }
    }

    pub(super) fn delete(self) {
        unsafe { gl::DeleteProgram(self.0) }
    }

    fn preprocess_sources(src: &str) -> Result<(&str, &str), String> {
        // remove all block comments /* */
        let comment_re = Regex::new(r"(?m)/\*[\s\S]*?\*/").unwrap();
        let src = comment_re.replace(src, "").to_string();
        // capture all #[<name>] at start of line
        let region_re = Regex::new(r"(?m)^(?:^|\n)#\[(\w+)]").unwrap();
        let mut start = 0;
        let mut regions = HashMap::new();
        let mut opt_match = region_re.find_at(&src, start);
        while let Some(m) = opt_match {
            let name = m.as_str().trim();
            // remove the #[...]
            let name = &name[2..name.len()-1];
            start = m.end();
            opt_match = region_re.find_at(&src, start);
            if let Some(m2) = opt_match {
                // is not last region
                regions.insert(name, &src[m.end()..m2.start()]);
            } else {
                // is last region
                regions.insert(name, &src[m.end()..]);
            };
        }
        Ok((regions.remove("vertex").ok_or("did not find vertex region in shader".to_string())?,
            regions.remove("fragment").ok_or("did not find fragment region in shader".to_string())?))
    }

    pub fn from_source(src: &str) -> Result<Self, String> {
        // find first `#type`
        let (vertex_src, fragment_src) = Self::preprocess_sources(src)?;
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vertex_src)
            .map_err(|e| format!("Vertex Shader Compile Error: {e}"))?;
        let f = Shader::from_source(ShaderType::Fragment, fragment_src)
            .map_err(|e| format!("Fragment Shader Compile Error: {e}"))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        }
        else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }
}