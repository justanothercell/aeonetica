use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::server::MyModule;

use aeonetica_client::{
    ClientMod,
    renderer::{*, shader::*, texture::{*, font::BitmapFont}, window::{OpenGlContextProvider, events::*}, layer::Layer},
    networking::messaging::{ClientHandle, ClientMessenger},
};
use aeonetica_client::data_store::DataStore;

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

    fn start(&self, context: &mut aeonetica_client::renderer::context::Context, store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
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

    fn start(&mut self, messenger: &mut ClientMessenger, store: &mut DataStore) {
        messenger.register_receiver(MyClientHandle::receive_server_msg);
        messenger.call_server_fn(MyModule::receive_client_msg, "Hello from client server call function".to_string(), SendMode::Safe);
        log!("receive_server_msg registered in start");
    }

    fn remove(&mut self, _messenger: &mut ClientMessenger, store: &mut DataStore) {
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
    texture_shader: shader::Program,
    post_processing_shader: shader::Program,
    texture: Texture,
    texture2: Texture,
    spritesheet: SpriteSheet,
    font: BitmapFont,
    aeonetica_font: BitmapFont,
    
    moving_quad: RefCell<Option<TexturedQuad>>
}

impl TestLayer {
    const TEST_SCREEN_WIDTH: f32 = 500.0;
}

impl PostProcessingLayer for TestLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}

    fn post_processing_shader(&self) -> &shader::Program {
        &self.post_processing_shader
    }
}

impl Layer for TestLayer {
    fn instantiate() -> Self {
        let charmap = HashMap::from([
            ('A', 0),
            ('B', 1),
            ('C', 2),
            ('D', 3),
            ('E', 4),
            ('F', 5),
            ('G', 6),
            ('H', 7),
            ('I', 8),
            ('J', 9),
            ('K', 10),
            ('L', 11),
            ('M', 12),
            ('N', 13),
            ('O', 14),
            ('P', 15),
            ('Q', 16),
            ('R', 17),
            ('S', 18),
            ('T', 19),
            ('U', 10),
            ('V', 21),
            ('W', 22),
            ('X', 23),
            ('Y', 24),
            ('Z', 25),
            (',', 26),
            ('!', 27),
            (' ', 28)
        ]);

        Self {
            renderer: RefCell::new(Renderer::new()),
            camera: RefCell::new(Camera::new(0.0, 1280.0, 720.0, 0.0, -1.0, 1.0)),
            texture_shader: shader::Program::from_source(include_str!("../assets/test_texture_shader.glsl")).expect("error loading texture shader"),
            texture: Texture::from_bytes(include_bytes!("../assets/aeonetica_logo.png")).expect("error loading texture"),
            texture2: Texture::from_bytes(include_bytes!("../assets/directions.png")).expect("error loading texture"),
            spritesheet: SpriteSheet::from_texture(Texture::from_bytes(include_bytes!("../assets/spritesheet.png")).expect("error loading texture"), (15, 15).into()).expect("error loading spritesheet"),
            post_processing_shader: shader::Program::from_source(include_str!("../assets/postprocessing_shader.glsl")).expect("error loading post processing shader"),
            font: BitmapFont::from_texture(Texture::from_bytes(include_bytes!("../assets/bitmapfont.png")).unwrap(), (5, 8).into(), charmap, false).expect("error crating font"),
            aeonetica_font: BitmapFont::from_texture_and_fontdata(Texture::from_bytes(include_bytes!("../assets/aeonetica_font.png")).unwrap(), include_str!("../assets/aeonetica_font.bmf")).expect("error crating font"),
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

        self.renderer.borrow_mut().static_string("HELLO WORLD", &(-80.0, -130.0).into(), 20.0, 4.0, &self.font, self.texture_shader.clone(), 1);
        self.renderer.borrow_mut().static_string("WWWWWWWWWW", &(-80.0, -100.0).into(), 20.0, 4.0, &self.font, self.texture_shader.clone(), 1);
        self.renderer.borrow_mut().static_string("IIIIIIIIII", &(-80.0, -70.0).into(), 20.0, 4.0, &self.font, self.texture_shader.clone(), 1);


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
    }

    fn on_detach(&self) {
        log!("TestLayer detached!");
    }

    fn on_update(&self, delta_time: f64) {
        let mut borrow = self.moving_quad.borrow_mut();
        let moving_quad = borrow.as_mut().unwrap();
        moving_quad.set_rotation(moving_quad.rotation() + delta_time as f32);

        let mut renderer = self.renderer.borrow_mut();
        renderer.modify(moving_quad);

        renderer.begin_scene(&self.camera.borrow());
        renderer.draw_vertices();
        renderer.end_scene();
    }

    fn on_event(&self, event: &Event) -> bool {
        false
    }
}