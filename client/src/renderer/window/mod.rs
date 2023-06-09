pub mod events;

use core::f32;
use std::{sync::mpsc::Receiver, collections::HashMap};

use aeonetica_engine::{log, math::vector::*, error::{*, builtin::IOError}, time::Time};
use crate::{renderer::{context::RenderContext, buffer::{framebuffer::Attachment, renderbuffer::RenderBuffer}, util::*, shader::UniformStr, texture::{Texture, Format}}, uniform_str, client_runtime::ClientRuntime, data_store::DataStore};
use glfw::{*, Window as GlfwWindow, Context as GlfwContext};
use image::{io::Reader as ImageReader, DynamicImage, EncodableLayout};

use self::events::Event;

use super::{buffer::framebuffer::FrameBuffer, shader, texture::ImageError};

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

    pub fn with_render<'a>(&'a self, context: &'a mut RenderContext) -> OpenGlRenderContextProvider<'a> {
        OpenGlRenderContextProvider(self, context)
    }
}

pub struct OpenGlRenderContextProvider<'a>(&'a OpenGlContextProvider, &'a mut RenderContext);

impl<'a> OpenGlRenderContextProvider<'a> {
    pub fn make_context(self) -> &'a mut RenderContext {
        gl::load_with(|s| self.0.get(s));
        self.1
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
        viewport(self.offset, self.size);
    }

    fn translate(&self, input: Vector2<f32>) -> Vector2<f32> {
        let fb_size = Window::FRAMEBUFFER_SIZE.to_f32();
        ((input - self.offset.to_f32()) / self.size.to_f32() * fb_size).clamp(Vector2::default(), fb_size)
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
    framebuffer_viewport: Viewport,

    default_post_processing_shader: shader::Program,
}

impl Window {
    const DEFAULT_WINDOW_WIDTH: u32 = 1280;
    const DEFAULT_WINDOW_HEIGHT: u32 = 720;
    const DEFAULT_WINDOW_TITLE: &'static str = "Aeonetica Game Engine";
    pub(super) const FRAMEBUFFER_SIZE: Vector2<u32> = Vector2 { x: 1920, y: 1080 };

    pub(crate) fn new(full_screen: bool) -> ErrorResult<Self> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("error creating window");
        
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
            )
        }).ok_or_else(|| aeonetica_engine::error::Error::new(IOError("error creating glfw window".to_string()), Fatality::FATAL, true))?;
                
        window.make_current();
        window.set_key_polling(true);
        window.set_icon_from_pixels(load_window_icons()?);
                
        let mut context_provider = OpenGlContextProvider::new();

        gl::load_with(|s| context_provider.insert(s, glfw.get_proc_address_raw(s)));
        glfw.set_swap_interval(if use_vsync() { glfw::SwapInterval::Adaptive } else { glfw::SwapInterval::None });
        window.set_all_polling(true);

        log!(r#"
==== OpenGL info ====
  -> Vendor: {}
  -> Renderer: {}
  -> Version: {}"#,
            get_gl_str!(gl::VENDOR), get_gl_str!(gl::RENDERER), get_gl_str!(gl::VERSION)
        );

        let default_post_processing_shader = shader::Program::from_source(include_str!("../../../assets/default-shader.glsl"))?;
        let framebuffer = FrameBuffer::new([
            Attachment::Color(Texture::create(Self::FRAMEBUFFER_SIZE, Format::RgbaF16)),
            Attachment::DepthStencil(RenderBuffer::new(Self::FRAMEBUFFER_SIZE)?)
        ], true)?;

        enable_blend_mode(true);
        blend_mode(BlendMode::One);

        let mut window = Self {
            glfw_handle: glfw,
            glfw_window: window,
            event_receiver: events,
            framebuffer,
            default_post_processing_shader,
            context_provider,
            framebuffer_viewport: Viewport::default()
        };

        window.framebuffer_viewport = Viewport::calculate(&window);

        Ok(window)
    }

    pub(crate) fn poll_events(&mut self, client: &mut ClientRuntime, context: &mut RenderContext, store: &mut DataStore) {
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
                Event::MouseMoved(pos) => *pos = self.framebuffer_viewport.translate(*pos),
                Event::Unknown() => handled = true,
                _ => ()
            }

            if !handled {
                context.on_event(client, event, store);
            }
        }
    }

    fn target_aspect_ratio(&self) -> f32 {
        let size = self.framebuffer.size().unwrap();
        size.x() as f32 / size.y() as f32
    }

    pub(crate) fn on_render(&mut self, context: &mut RenderContext, client: &mut ClientRuntime, store: &mut DataStore, time: Time) {
        // main frame rendering
        self.framebuffer.bind();
        self.framebuffer.clear([0.0, 0.0, 0.0, 1.0]);

        viewport(Vector2::default(), self.framebuffer.size().unwrap().map(|i| i as i32));
        enable_blend_mode(true);

        context.on_render(client, &Target::FrameBuffer(&self.framebuffer), store, time);

        self.framebuffer.unbind();
        
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);                
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.framebuffer_viewport.apply();
            enable_blend_mode(false);
        }

        // post-processing
        let post_processing_shader = context.post_processing_layer()
            .as_ref()
            .filter(|layer| layer.enabled())
            .map(|layer| layer.post_processing_shader())
            .unwrap_or(&self.default_post_processing_shader);

        context.post_processing_layer()
            .as_ref()
            .map(|layer| layer.uniforms())
            .unwrap_or(&[])
            .iter()
            .for_each(|(name, data)| post_processing_shader.upload_uniform(name, *data));
    
        const FRAME_UNIFORM_NAME: UniformStr = uniform_str!("u_Frame");

        self.framebuffer.render([(0, &FRAME_UNIFORM_NAME)], &Target::Raw, post_processing_shader);

        self.glfw_window.swap_buffers();
    }

    pub(crate) fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub(crate) fn context_provider(&self) -> &OpenGlContextProvider {
        &self.context_provider
    }

    pub(crate) fn finish(mut self) {
        self.glfw_window.close();
        self.default_post_processing_shader.delete();
        self.framebuffer.delete();
    }
}

fn pixels_from_bytes<const N: usize>(bytes: &[u8; N]) -> ErrorResult<glfw::PixelImage> {
    let cursor = std::io::Cursor::new(bytes);
    let icon = ImageReader::new(cursor)
        .with_guessed_format()?
        .decode().map_err(|e| ImageError::Decode(e.to_string()).into_error())?;

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
        _ => Err(ImageError::Unsupported(format!("image format {:?} not supported", icon)).into_error())
    }
}

fn load_window_icons() -> ErrorResult<Vec<glfw::PixelImage>> {
    Ok(vec![
        pixels_from_bytes(include_bytes!("../../../assets/logo-15.png"))?,
        pixels_from_bytes(include_bytes!("../../../assets/logo-30.png"))?,
        pixels_from_bytes(include_bytes!("../../../assets/logo-60.png"))?,
        pixels_from_bytes(include_bytes!("../../../assets/logo-120.png"))?
    ])
}

const VSYNC_ENVIRONMENT_VAR: &'static str = "AEONETICA_VSYNC";
fn use_vsync() -> bool {
    match std::env::var(VSYNC_ENVIRONMENT_VAR) {
        Ok(value) => matches!(value.to_uppercase().as_str(), "1" | "TRUE"),
        _ => false
    }
}