pub mod events;

use std::{sync::mpsc::Receiver, collections::HashMap, rc::Rc};

use aeonetica_engine::log;
use crate::renderer::{context::Context, buffer::*, util};
use glfw::{*, Window as GlfwWindow, Context as GlfwContext};

use super::{framebuffer::FrameBuffer, shader, vertex_array::VertexArray};

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
pub(crate) struct Window {
    glfw_handle: Glfw,
    glfw_window: GlfwWindow,
    event_receiver: Receiver<(f64, WindowEvent)>,
    context_provider: OpenGlContextProvider,
    framebuffer: FrameBuffer,
    framebuffer_vao: VertexArray,
    default_post_processing_shader: shader::Program,
}

impl Window {
    const DEFAULT_WINDOW_WIDTH: u32 = 1280;
    const DEFAULT_WINDOW_HEIGHT: u32 = 720;
    const DEFAULT_WINDOW_TITLE: &'static str = "Aeonetica Game Engine";
    const DEFAULT_FRAMEBUFFER_WIDTH: u32 = 1920;
    const DEFAULT_FRAMEBUFFER_HEIGHT: u32 = 1080;

    pub(crate) fn new(full_screen: bool) -> Self {
        match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(mut glfw) => {
                glfw.window_hint(WindowHint::ContextVersion(4, 5));
                glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

                let (mut window, events) = glfw.with_primary_monitor(|glfw, monitor| {
                    glfw.create_window(
                    Self::DEFAULT_WINDOW_WIDTH,
                    Self::DEFAULT_WINDOW_HEIGHT,
                    Self::DEFAULT_WINDOW_TITLE,
                    if full_screen {
                        monitor.map_or(WindowMode::Windowed, |m| WindowMode::FullScreen(m))
                    } else {
                        WindowMode::Windowed
                    }
                )}).expect("Error creating GLFW window!");
                
                window.make_current();
                window.set_key_polling(true);
                
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
                let framebuffer = FrameBuffer::new(1920, 1080)
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
                
                let vertex_buffer = Buffer::new(BufferType::Array, util::to_raw_byte_slice!(vertices), Some(Rc::new(layout)), BufferUsage::STATIC)
                    .expect("Error creating Vertex Buffer");
                framebuffer_vao.set_vertex_buffer(vertex_buffer);
                
                const INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];
                let index_buffer = Buffer::new(BufferType::ElementArray, util::to_raw_byte_slice!(INDICES), None, BufferUsage::STATIC)
                    .expect("Error creating Index Buffer");
                framebuffer_vao.set_index_buffer(index_buffer);

                unsafe {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
                }

                Self {
                    glfw_handle: glfw,
                    glfw_window: window,
                    event_receiver: events,
                    framebuffer,
                    framebuffer_vao,
                    default_post_processing_shader,
                    context_provider
                }
            },
            Err(err) => panic!("Error creating window: {err}!") 
        }
    }

    pub(crate) fn poll_events(&mut self, context: &mut Context) {
        self.glfw_handle.poll_events();
        for (_, event) in flush_messages(&self.event_receiver) {
            let event = events::Event::from_glfw(event);

            match event.typ() {
                events::EventType::WindowClose() => self.glfw_window.set_should_close(true),
                _ => ()
            }

            context.on_event(event);
        }
    }

    pub(crate) fn render(&mut self, context: &mut Context, delta_time: usize) {
        // main frame rendering
        self.framebuffer.bind();
        
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.2, 1.0);                
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let [width, height]: [u32; 2] = (*self.framebuffer.size()).into();
            gl::Viewport(0, 0, width as i32, height as i32);

            gl::Enable(gl::DEPTH_TEST);

        }

        context.on_update(delta_time);

        self.framebuffer.unbind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);                
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let (width, height) = self.glfw_window.get_size();
            gl::Viewport(0, 0, width, height);

            gl::Disable(gl::DEPTH_TEST);
        }

        // post-processing
        let post_processing_shader = context.post_processing_layer()
            .as_ref()
            .map(|layer| layer.post_processing_shader())
            .unwrap_or(&self.default_post_processing_shader);

        post_processing_shader.bind();

        self.framebuffer.texture().bind(0);
        post_processing_shader.upload_uniform("u_Frame", &0);

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
