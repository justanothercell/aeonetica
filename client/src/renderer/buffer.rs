use std::marker::PhantomData;

use super::*;

macro_rules! shader_tuple_impls {
    ($($name:ident)+) => {
        impl<$($name: ShaderLayoutType),+> Layout for ($($name,)+) {
            type Type = ($($name::Type,)+);
            fn layout() -> Vec<ShaderDataType> {
                vec![$($name::Type::DATA_TYPE,)+]
            }
        }
    };
}

pub struct Vertex;
impl ShaderLayoutType for Vertex {
    type Type = [f32; 3];
}

pub struct TexCoord;
impl ShaderLayoutType for TexCoord {
    type Type = [f32; 2];
}

pub struct Color;
impl ShaderLayoutType for Color {
    type Type = [f32; 4];
}

pub struct TextureID;
impl ShaderLayoutType for TextureID {
    type Type = Sampler2D;
}

pub(super) trait Layout {
    type Type;
    fn layout() -> Vec<ShaderDataType>;
}

pub(super) struct BufferLayoutBuilder<T>(PhantomData<T>);

impl<T: Layout> BufferLayoutBuilder<T> {
    pub(super) fn build() -> BufferLayout {
        BufferLayout::new(T::layout().iter().map(|d| (*d).into()).collect())
    }

    pub(super) const fn array<const N: usize>(arr: [T::Type; N]) -> [T::Type; N] {
        arr
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BufferUsage {
    STATIC = gl::STATIC_DRAW as isize,
    DYNAMIC = gl::DYNAMIC_DRAW as isize,
}

impl Into<gl::types::GLenum> for BufferUsage {
    fn into(self) -> gl::types::GLenum {
        self as gl::types::GLenum
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BufferType {
    Array = gl::ARRAY_BUFFER as isize,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize
}

impl Into<gl::types::GLenum> for BufferType {
    fn into(self) -> gl::types::GLenum {
        self as gl::types::GLenum
    }
}

pub struct Buffer {
    id: RenderID,
    typ: BufferType,
    layout: Option<Rc<BufferLayout>>,
    count: u32,
}

impl Buffer {
    pub(super) fn new(typ: BufferType, data: &[u8], layout: Option<Rc<BufferLayout>>, usage: BufferUsage) -> Option<Self> {
        let mut id = 0;
        unsafe { 
            gl::CreateBuffers(1, &mut id);
            gl::BindBuffer(typ.into(), id);
            gl::BufferData(typ.into(), data.len() as isize, data.as_ptr() as *const _, usage.into());
        }
        if id != 0 {
            Some(Self {
                id,
                typ,
                layout,
                count: (data.len() / std::mem::size_of::<gl::types::GLuint>()) as u32,
            })
        }
        else {
            None
        }
    }

    pub(super) fn new_sized(typ: BufferType, num_bytes: isize, layout: Option<Rc<BufferLayout>>, usage: BufferUsage) -> Option<Self> {
        let mut id = 0;
        unsafe { 
            gl::CreateBuffers(1, &mut id);
            gl::BindBuffer(typ.into(), id);
            gl::BufferData(typ.into(), num_bytes, std::ptr::null(), usage.into());
        }
        if id != 0 {
            Some(Self {
                id,
                typ,
                layout,
                count: 0,
            })
        }
        else {
            None
        }
    }

    pub(super) fn delete(self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }

    pub(super) fn bind(&self) {
        unsafe { gl::BindBuffer(self.typ.into(), self.id) }
    }

    pub(super) fn unbind(&self) {
        unsafe { gl::BindBuffer(self.typ.into(), 0) }
    } 

    pub(super) fn layout(&self) -> &Option<Rc<BufferLayout>> {
        &self.layout
    }

    pub(super) fn count(&self) -> u32 {
        self.count
    }

    pub(super) fn set_count(&mut self, count: u32) {
        self.count = count;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferElement {
    typ: ShaderDataType,
    offset: u32,
    normalized: bool
}

impl From<ShaderDataType> for BufferElement {
    fn from(value: ShaderDataType) -> Self {
        Self::new(value)
    }
}

impl BufferElement {
    pub(super) fn new(typ: ShaderDataType) -> Self {
        Self {
            typ,
            offset: 0,
            normalized: false
        }
    }

    pub(super) fn typ(&self) -> ShaderDataType {
        self.typ
    }

    pub(super) fn size(&self) -> u32 {
        self.typ.size()
    }

    pub(super) fn component_count(&self) -> i32 {
        self.typ.component_count()
    }

    pub(super) fn base_type(&self) -> gl::types::GLenum {
        self.typ.base_type()
    }

    pub(super) fn offset(&self) -> u32 {
        self.offset
    }

    pub(super) fn set_offset(&mut self, offset: u32) {
        self.offset = offset
    }

    pub(super) fn normalized(&self) -> gl::types::GLboolean {
        if self.normalized {
            gl::TRUE
        }
        else {
            gl::FALSE
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferLayout {
    elements: Vec<BufferElement>,
    stride: u32
}

impl BufferLayout {
    pub fn new(elements: Vec<BufferElement>) -> Self {
        let mut buffer = Self {
            elements,
            stride: 0
        };
        buffer.calculate_offsets_and_stride();
        buffer
    }
    
    pub(super) fn stride(&self) -> u32 {
        self.stride
    }

    pub(super) fn elements(&self) -> &Vec<BufferElement> {
        &self.elements
    }

    fn calculate_offsets_and_stride(&mut self) {
        let mut offset = 0;
        self.stride = 0;
        for element in self.elements.iter_mut() {
            element.set_offset(offset);
            offset += element.size();
            self.stride += element.size();
        }
    }
}

shader_tuple_impls! { A }
shader_tuple_impls! { A B }
shader_tuple_impls! { A B C }
shader_tuple_impls! { A B C D }
shader_tuple_impls! { A B C D E }
shader_tuple_impls! { A B C D E F }
shader_tuple_impls! { A B C D E F G }
shader_tuple_impls! { A B C D E F G H }
shader_tuple_impls! { A B C D E F G H I }
shader_tuple_impls! { A B C D E F G H I J }
shader_tuple_impls! { A B C D E F G H I J K }
shader_tuple_impls! { A B C D E F G H I J K L }
