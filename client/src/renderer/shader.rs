use super::*;

extern crate regex;
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
        let re = Regex::new(r"(#type)( )+([a-zA-Z]+)").unwrap();
        let mut split = re.split(src).collect::<Vec<_>>();
        if split.len() < 3 {
            return Err(format!("Shaders have to be composed of both vertex and fragment shaders denoted with the `#type` directive; got: {split:?}"))
        }

        let index = src.find("#type").ok_or_else(|| "Could not find mandatory `#type` directive in shader soures".to_string())? + 6;
        let eol = src[index..].find(LINE_ENDING).ok_or_else(|| "`#type` cannot be on the last line".to_string())? + index;
        let first = src[index..eol].trim();
        
        let second_offset = eol + LINE_ENDING.len();
        let index = src[second_offset..].find("#type").ok_or_else(|| "Could not find mandatory `#type` directive in shader soures".to_string())? + 6 + second_offset;
        let eol = src[index..].find(LINE_ENDING).ok_or_else(|| "`#type` cannot be on the last line".to_string())? + index;
        let second = src[index..eol].trim();

        if first == "vertex" && second == "fragment" {
            Ok((split[1], split[2]))
        }
        else if first == "fragment" && second == "vertex" {
            Ok((split[2], split[1]))
        }
        else {
            Err("There has to be exactly one fragment and one vertex shader `#type` directive (got {first} and {second})".to_string())
        }
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