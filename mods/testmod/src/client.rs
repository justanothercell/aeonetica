use std::cell::{RefCell, Cell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::server::MyModule;

use aeonetica_client::client_runtime::ClientHandleBox;
use aeonetica_client::renderer::text_area::TextArea;
use aeonetica_client::{
    ClientMod,
    renderer::{*, shader::*, texture::{*, font::BitmapFont}, window::{OpenGlContextProvider, events::*}, layer::Layer},
    networking::messaging::{ClientHandle, ClientMessenger},
};
use aeonetica_client::data_store::DataStore;

use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::{
    Id, log,
    util::{camera::Camera, vector::*, type_to_id},
    networking::{SendMode, messaging::*}
};

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

    fn start(&self, context: &mut aeonetica_client::renderer::context::Context, _store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
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

    fn start(&mut self, messenger: &mut ClientMessenger, _store: &mut DataStore) {
        messenger.register_receiver(MyClientHandle::receive_server_msg);
        messenger.call_server_fn(MyModule::receive_client_msg, "Hello from client server call function".to_string(), SendMode::Safe);
        log!("receive_server_msg registered in start");
    }

    fn remove(&mut self, _messenger: &mut ClientMessenger, _store: &mut DataStore) {
        log!("my client handle removed")
    }

    fn update(&mut self, renderer: &std::cell::RefMut<Renderer>, delta_time: f64) {
        
    }

    fn on_event(&mut self, event: &Event) -> bool {
        false
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
    texture_shader: shader::Program,
    post_processing_shader: shader::Program,
    texture: Texture,
    texture2: Texture,
    spritesheet: SpriteSheet,
    aeonetica_font: Rc<BitmapFont>,

    post_processing_enabled: Cell<bool>,

    fps_display: RefCell<TextArea>,
    moving_quad: RefCell<Option<TexturedQuad>>
}

impl TestLayer {
    const TEST_SCREEN_WIDTH: f32 = 500.0;
}

impl PostProcessingLayer for TestLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}

    fn enabled(&self) -> bool {
        self.post_processing_enabled.get()
    }

    fn post_processing_shader(&self) -> &shader::Program {
        &self.post_processing_shader
    }
}

impl Layer for TestLayer {
    fn instantiate() -> Self {
        let texture_shader = shader::Program::from_source(include_str!("../assets/test_texture_shader.glsl")).expect("error loading texture shader");
        let aeonetica_font = Rc::new(BitmapFont::from_texture_and_fontdata(Texture::from_bytes(include_bytes!("../assets/aeonetica_font.png")).unwrap(), include_str!("../assets/aeonetica_font.bmf")).expect("error crating font"));

        Self {
            renderer: RefCell::new(Renderer::new()),
            camera: RefCell::new(Camera::new(0.0, 1280.0, 720.0, 0.0, -1.0, 1.0)),
            texture_shader,
            texture: Texture::from_bytes(include_bytes!("../assets/aeonetica_logo.png")).expect("error loading texture"),
            texture2: Texture::from_bytes(include_bytes!("../assets/directions.png")).expect("error loading texture"),
            spritesheet: SpriteSheet::from_texture(Texture::from_bytes(include_bytes!("../assets/spritesheet.png")).expect("error loading texture"), (15, 15).into()).expect("error loading spritesheet"),
            post_processing_shader: shader::Program::from_source(include_str!("../assets/postprocessing_shader.glsl")).expect("error loading post processing shader"),
            aeonetica_font: aeonetica_font.clone(),
            
            fps_display: RefCell::new(
                TextArea::from_string(
                    Vector2::new(10.0, 10.0), 
                    1, 
                    50.0, 2.0,
                    texture_shader, aeonetica_font, 
                    String::from("F")
                )
            ),
            post_processing_enabled: Cell::new(true),

            moving_quad: RefCell::new(None)
        }
    }

    fn on_attach(&self) {
        log!("TestLayer attached!");

        let mut k = 0;
        for i in -2..3 {
            for j in -2..3 {
                let pos = Vector2::new(i * 50, j * 50).map(|v| v as f32);
                //self.renderer.borrow_mut().static_quad(&pos, (40.0, 40.0).into(), if k % 2 == 0 { RED_COLOR } else { BLUE_COLOR }, self.shader.clone()),
                let texture_id = if k % 2 == 0 { self.texture.id() } else { self.texture2.id() };
                let mut quad = TexturedQuad::new(pos, Vector2::new(40.0, 40.0), 0, texture_id, self.texture_shader.clone());
                self.renderer.borrow_mut().add(&mut quad);
                k += 1;
            }
        }

        for i in -2..2 {
            let pos = Vector2::new(-150, i * 40).map(|v| v as f32);
            let sprite = self.spritesheet.get((i + 2) as u32).expect("error getting sprite");
            let mut quad = SpriteQuad::new(pos, Vector2::new(30.0, 30.0), 0, sprite, self.texture_shader.clone());
            self.renderer.borrow_mut().add(&mut quad);
        }

        for (i, row) in ["#![no_main]",
"",
"use std::fs::File;",
"use std::io::Write;",
"use std::os::unix::io::FromRawFd;",
"",
"fn stdout() -> File {",
"    unsafe { File::from_raw_fd(1) }",
"}",
"",
"#[no_mangle]",
"pub fn main(_argc: i32, _argv: *const *const u8) {",
"    let mut stdout = stdout();",
"    stdout.write(b\"Hello, world!\\n\").unwrap();",
"}"].into_iter().enumerate() {
            self.renderer.borrow_mut().static_string(row, &(-120.0, -40.0 + i as f32 * 10.0).into(), 10.0, 2.0, &self.aeonetica_font, self.texture_shader.clone(), 1);
        }

        let (fb_width, fb_height) = (640.0 / 2.0, 360.0 / 2.0);
        self.camera.borrow_mut().set_projection(-fb_width, fb_width, fb_height, -fb_height, -1.0, 1.0);

        let mut moving_quad = TexturedQuad::new(Vector2::new(-250.0, -170.0), Vector2::new(75.0, 75.0), 2, self.texture.id(), self.texture_shader.clone());
        self.renderer.borrow_mut().add(&mut moving_quad);
        *self.moving_quad.borrow_mut() = Some(moving_quad);

        self.renderer.borrow_mut().add(&mut *self.fps_display.borrow_mut());
    }

    fn on_detach(&self) {
        log!("TestLayer detached!");
    }

    fn on_update(&self, handles: &mut IdMap<ClientHandleBox>, delta_time: f64) {
        let mut borrow = self.moving_quad.borrow_mut();
        let moving_quad = borrow.as_mut().unwrap();
        moving_quad.set_rotation(moving_quad.rotation() + delta_time as f32);

        let mut renderer = self.renderer.borrow_mut();
        renderer.modify(moving_quad);

        for handle in handles.values_mut() {
            handle.update(&renderer, delta_time);   
        }

        renderer.begin_scene(&self.camera.borrow());
        renderer.draw_vertices();
        renderer.end_scene();
    }

    fn on_event(&self, handles: &mut IdMap<ClientHandleBox>, event: &Event) -> bool {
        for handle in handles.values_mut() {
            if handle.on_event(event) {
                return true
            }
        }
        
        match event.typ() {
            EventType::KeyPressed(33) => {
                /* key P */
                self.post_processing_enabled.set(!self.post_processing_enabled.get());
                true
            },
            _ => false
        }
    }
}
