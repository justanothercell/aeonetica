use std::cell::RefCell;
use std::collections::HashMap;
use aeonetica_client::ClientMod;
use aeonetica_client::renderer::Renderer;
use aeonetica_client::renderer::postprocessing::PostProcessingLayer;
use aeonetica_engine::util::camera::Camera;
use aeonetica_client::renderer::window::events::{Event, EventType};
use aeonetica_engine::util::vector::Vector2;
use aeonetica_engine::{Id, log};
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::type_to_id;
use aeonetica_client::renderer::layer::Layer;
use aeonetica_client::renderer::window::OpenGlContextProvider;
use aeonetica_client::renderer::shader;
use aeonetica_client::renderer::texture::Texture;
use crate::server::MyModule;
use std::rc::Rc;

pub struct TestModClient {

}

impl ClientMod for TestModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!");
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
        handlers.insert(type_to_id::<MyClientHandle>(), || Box::new(MyClientHandle { }));
    }

    fn init_context(&self, context: &mut aeonetica_client::renderer::context::Context, gl_context_provider: &OpenGlContextProvider) {
        gl_context_provider.make_context();
        let test_layer = Rc::new(TestLayer::instantiate());
        context.push(test_layer.clone());
        context.set_post_processing_layer(test_layer);
    }
}

pub(crate) struct MyClientHandle {

}

impl ClientEntity for MyClientHandle {}

impl ClientHandle for MyClientHandle {
    fn init(&mut self) {
        log!("my client handle initialized")
    }

    fn start(&mut self, messenger: &mut ClientMessenger) {
        messenger.register_receiver(MyClientHandle::receive_server_msg);
        messenger.call_server_fn(MyModule::receive_client_msg, "Hello from client server call function".to_string(), SendMode::Safe);
        log!("receive_server_msg registered in start");
    }

    fn remove(&mut self, _messenger: &mut ClientMessenger) {
        log!("my client handle removed")
    }


}

impl MyClientHandle {
    pub(crate) fn receive_server_msg(&mut self, msg: String){
        log!("received server msg: {msg}")
    }
}

struct TestLayer {
    renderer: RefCell<Renderer>,
    camera: RefCell<Camera>,
    shader: shader::Program,
    texture_shader: shader::Program,
    post_processing_shader: shader::Program,
    texture: Texture,
    texture2: Texture,
}

impl TestLayer {
    const TEST_SCREEN_WIDTH: f32 = 500.0;
}

impl PostProcessingLayer for TestLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}

    fn shader(&self) -> &shader::Program {
        &self.post_processing_shader
    }
}

impl Layer for TestLayer {
    fn instantiate() -> Self {
        Self {
            renderer: RefCell::new(Renderer::new()),
            camera: RefCell::new(Camera::new(0.0, 1280.0, 720.0, 0.0, -1.0, 1.0)),
            shader: shader::Program::from_source(include_str!("../assets/test_shader.glsl")).expect("error loading shader"),
            texture_shader: shader::Program::from_source(include_str!("../assets/test_texture_shader.glsl")).expect("error loading texture shader"),
            texture: Texture::from_bytes(include_bytes!("../assets/aeonetica_logo.png")).expect("error loading texture"),
            texture2: Texture::from_bytes(include_bytes!("../assets/directions.png")).expect("error loading texture"),
            post_processing_shader: shader::Program::from_source(include_str!("../assets/postprocessing_shader.glsl")).expect("error loading post processing shader")
        }
    }

    fn on_attach(&self) {
        log!("TestLayer attached!");

        const RED_COLOR: [f32; 4] = [0.7, 0.2, 0.2, 1.0];
        const BLUE_COLOR: [f32; 4] = [0.2, 0.2, 0.7, 1.0];

        let mut k = 0;
        for i in -2..3 {
            for j in -2..3 {
                let pos = Vector2::new(i * 50, j * 50).map(|v| v as f32);
                //self.renderer.borrow_mut().static_quad(&pos, (40.0, 40.0).into(), if k % 2 == 0 { RED_COLOR } else { BLUE_COLOR }, self.shader.clone()),
                self.renderer.borrow_mut().textured_quad(&pos, (40.0, 40.0).into(), if k % 2 == 0 { self.texture.id() } else { self.texture2.id() }, self.texture_shader.clone());
                k += 1;
            }
        }
    }

    fn on_detach(&self) {
        log!("TestLayer detached!");
    }

    fn on_update(&self) {
        let mut renderer = self.renderer.borrow_mut();
        renderer.begin_scene(&*self.camera.borrow());
        renderer.draw_vertices();
        renderer.end_scene();
    }

    fn on_event(&self, event: &aeonetica_client::renderer::window::events::Event) -> bool {
        match event.typ() {
            EventType::WindowResize(x, y) => {
                let aspect_ratio = *x as f32 / *y as f32;
                let screen_width = Self::TEST_SCREEN_WIDTH / 2.0;
                let screen_height = Self::TEST_SCREEN_WIDTH / 2.0 / aspect_ratio;
                self.camera.borrow_mut().set_projection(-screen_width, screen_width, screen_height, -screen_height, -1.0, 1.0);
                true
            }
            _ => false
        }
    }
}