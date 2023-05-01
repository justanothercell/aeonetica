pub mod events;

use core::f32;
use std::{sync::mpsc::Receiver, collections::HashMap, rc::Rc};

use aeonetica_engine::{log, log_err, util::vector::{Vector2, IntoVector}};
use crate::{renderer::{context::Context, buffer::*, util, shader::UniformStr}, uniform_str, client_runtime::ClientRuntime};
use glfw::{*, Window as GlfwWindow, Context as GlfwContext};
use image::{io::Reader as ImageReader, DynamicImage, EncodableLayout};

use self::events::Event;

use super::{buffer::{framebuffer::FrameBuffer, vertex_array::VertexArray}, shader, texture::ImageError};

pub struct OpenGlContextProvider(HashMap<&'static str, GLProc>);

impl OpenGlContextProvider {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, name: &'static str, ptr: GLProc) -> GLProc {
        self.0.insert(name, ptr);
        ptr
    }

    fn get(&self, name: &str) -> GLProc {
        *self.0.get(name).unwrap_or(&std::ptr::null())
    }

    pub fn make_context(&self) {
        gl::load_with(|s| self.get(s));
    }
}

struct Viewport {
    offset: Vector2<i32>,
    size: Vector2<i32>,
}

impl Viewport {
    fn calculate(window: &Window) -> Self {
        let size = window.glfw_window.get_size().into_vector();
        let aspect_ratio = window.target_aspect_ratio();

        let mut aspect = Vector2::new(size.x(), (size.x() as f32 / aspect_ratio) as i32);
        if aspect.y() > size.y() {
            aspect.y = size.y();
            aspect.x = (size.y() as f32 * aspect_ratio) as i32;
        }

        Self {
            offset: size.half() - aspect.half(),
            size: aspect
        }
    }

    fn apply(&self) {
        unsafe { gl::Viewport(self.offset.x(), self.offset.y(), self.size.x(), self.size.y()) }
    }

    fn translate(&self, input: Vector2<f32>) -> Vector2<f32> {
        (input - self.offset.to_f32()) / self.size.to_f32() * Window::FRAMEBUFFER_SIZE.to_f32()
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self { offset: Vector2::default(), size: Vector2::new(1920, 1080) } 
    }
}

pub(crate) struct Window {
    glfw_handle: Glfw,
    glfw_window: GlfwWindow,
    
    event_receiver: Receiver<(f64, WindowEvent)>,
    context_provider: OpenGlContextProvider,

    framebuffer: FrameBuffer,
    framebuffer_vao: VertexArray,
    framebuffer_viewport: Viewport,

    default_post_processing_shader: shader::Program,
}

impl Window {
    const DEFAULT_WINDOW_WIDTH: u32 = 1280;
    const DEFAULT_WINDOW_HEIGHT: u32 = 720;
    const DEFAULT_WINDOW_TITLE: &'static str = "Aeonetica Game Engine";
    const FRAMEBUFFER_SIZE: Vector2<u32> = Vector2 { x: 1920, y: 1080 };

    pub(crate) fn new(full_screen: bool) -> Self {
        match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(mut glfw) => {
                glfw.window_hint(WindowHint::ContextVersion(4, 5));
                glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
                glfw.window_hint(WindowHint::DoubleBuffer(true));

                let (mut window, events) = glfw.with_primary_monitor(|glfw, monitor| {
                    glfw.create_window(
                    Self::DEFAULT_WINDOW_WIDTH,
                    Self::DEFAULT_WINDOW_HEIGHT,
                    Self::DEFAULT_WINDOW_TITLE,
                    if full_screen {
                        monitor.map_or(WindowMode::Windowed, WindowMode::FullScreen)
                    } else {
                        WindowMode::Windowed
                    }
                )}).expect("Error creating GLFW window!");
                
                window.make_current();
                window.set_key_polling(true);

                match load_window_icons() {
                    Ok(icons) => window.set_icon_from_pixels(icons),
                    Err(err) => log_err!("error loading window icon: {}", err.to_string())
                }
                
                let mut context_provider = OpenGlContextProvider::new();

                gl::load_with(|s| context_provider.insert(s, glfw.get_proc_address_raw(s)));
                glfw.set_swap_interval(glfw::SwapInterval::None);
                window.set_all_polling(true);

                log!(r#"
==== OpenGL info ====
  -> Vendor: {}
  -> Renderer: {}
  -> Version: {}"#, 
                    unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8).to_str().unwrap() },
                    unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8).to_str().unwrap() },
                    unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_str().unwrap() }
                );

                let default_post_processing_shader = shader::Program::from_source(include_str!("../../../assets/default-shader.glsl"))
                    .expect("error loading default post processing shader");
                let framebuffer = FrameBuffer::new(Self::FRAMEBUFFER_SIZE)
                    .expect("error creating framebuffer");

                let mut framebuffer_vao = VertexArray::new().expect("Error creating vertex array");
                framebuffer_vao.bind();
    
                type Vertices = BufferLayoutBuilder<(Vertex, TexCoord)>;
                let layout = Vertices::build();
                let vertices = Vertices::array([
                    vertex!([-1.0, -1.0, 0.0], [0.0, 0.0]),
                    vertex!([1.0,  -1.0, 0.0], [1.0, 0.0]),
                    vertex!([1.0,  1.0,  0.0], [1.0, 1.0]),
                    vertex!([-1.0, 1.0,  0.0], [0.0, 1.0])
                ]);
                
                let vertex_buffer = Buffer::new(BufferType::Array, util::to_raw_byte_slice!(&vertices), Some(Rc::new(layout)), BufferUsage::STATIC)
                    .expect("Error creating Vertex Buffer");
                framebuffer_vao.set_vertex_buffer(vertex_buffer);
                
                const INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];
                let index_buffer = Buffer::new(BufferType::ElementArray, util::to_raw_byte_slice!(&INDICES), None, BufferUsage::STATIC)
                    .expect("Error creating Index Buffer");
                framebuffer_vao.set_index_buffer(index_buffer);

                unsafe {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
                }

                let mut window = Self {
                    glfw_handle: glfw,
                    glfw_window: window,
                    event_receiver: events,
                    framebuffer,
                    framebuffer_vao,
                    default_post_processing_shader,
                    context_provider,
                    framebuffer_viewport: Viewport::default()
                };

                window.framebuffer_viewport = Viewport::calculate(&window);

                window
            },
            Err(err) => panic!("Error creating window: {err}!") 
        }
    }

    pub(crate) fn poll_events(&mut self, client: &mut ClientRuntime, context: &mut Context) {
        self.glfw_handle.poll_events();
        for (_, event) in flush_messages(&self.event_receiver) {
            let mut event = Event::from_glfw(event);
            let mut handled = false;

            match &mut event {
                Event::WindowClose() => self.glfw_window.set_should_close(true),
                Event::WindowResize(_) => {
                    self.framebuffer_viewport = Viewport::calculate(self);
                    handled = true
                }
                Event::MouseMoved(pos) => *pos = self.framebuffer_viewport.translate(pos.clone()),
                Event::Unknown() => handled = true,
                _ => ()
            }

            if !handled {
                context.on_event(client, event);
            }
        }
    }

    fn target_aspect_ratio(&self) -> f32 {
        let size = self.framebuffer.size();
        size.x() as f32 / size.y() as f32
    }

    pub(crate) fn render(&mut self, context: &mut Context, client: &mut ClientRuntime, delta_time: f64) {
        // main frame rendering
        self.framebuffer.bind();
        
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);                
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let [width, height]: [u32; 2] = (*self.framebuffer.size()).into();
            gl::Viewport(0, 0, width as i32, height as i32);

            gl::Enable(gl::BLEND);
        }

        context.on_update(client, delta_time);

        self.framebuffer.unbind();
        
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);                
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.framebuffer_viewport.apply();
            gl::Disable(gl::BLEND);
        }

        // post-processing
        let post_processing_shader = context.post_processing_layer()
            .as_ref()
            .filter(|layer| layer.enabled())
            .map(|layer| layer.post_processing_shader())
            .unwrap_or(&self.default_post_processing_shader);

        post_processing_shader.bind();

        self.framebuffer.texture().bind(0);

        const FRAME_UNIFORM_NAME: UniformStr = uniform_str!("u_Frame");
        post_processing_shader.upload_uniform(&FRAME_UNIFORM_NAME, &0);

        context.post_processing_layer()
            .as_ref()
            .map(|layer| layer.uniforms())
            .unwrap_or(&[])
            .iter()
            .for_each(|(name, data)| post_processing_shader.upload_uniform(name, *data));

        self.framebuffer_vao.bind();
        self.framebuffer_vao.draw();
        self.framebuffer_vao.unbind();

        post_processing_shader.unbind();

        self.glfw_window.swap_buffers();
    }

    pub(crate) fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub(crate) fn context_provider(&self) -> &OpenGlContextProvider {
        &self.context_provider
    }

    pub(crate) fn finish(self) {
        self.glfw_window.close();
        self.default_post_processing_shader.delete();
        self.framebuffer_vao.delete();
        self.framebuffer.delete();
    }
}

fn pixels_from_bytes<const N: usize>(bytes: &[u8; N]) -> Result<glfw::PixelImage, ImageError> {
    let cursor = std::io::Cursor::new(bytes);
    let icon = ImageReader::new(cursor)
        .with_guessed_format()?
        .decode()?;

    let (width, height) = (icon.width(), icon.height());

    match icon {
        DynamicImage::ImageRgba8(bytes) => {
            let bytes = bytes.as_bytes();

            let mut pixels = Vec::with_capacity((width * height) as usize * std::mem::size_of::<u32>());
            for i in (0..bytes.len()).step_by(4) {
                let pixel = &bytes[i..i+4];
                pixels.push(u32::from_ne_bytes(pixel.try_into().unwrap())); 
            }

            Ok(PixelImage {width, height, pixels})
        }
        _ => Err(ImageError::Unsupported(format!("image format {:?} not supported", icon)))
    }
}

fn load_window_icons() -> Result<Vec<glfw::PixelImage>, ImageError> {
    Ok(vec![
        pixels_from_bytes(include_bytes!("../../../assets/logo-15.png"))?,
        pixels_from_bytes(include_bytes!("../../../assets/logo-30.png"))?,
        pixels_from_bytes(include_bytes!("../../../assets/logo-60.png"))?,
        pixels_from_bytes(include_bytes!("../../../assets/logo-120.png"))?
    ])
}
