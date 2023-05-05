use std::{collections::HashMap, rc::Rc, cell::{RefCell, RefMut}};

use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, data_store::DataStore, renderer::{window::{OpenGlContextProvider, events::Event}, layer::Layer, context::Context, Renderer, texture::{SpriteSheet, Texture}, Quad, TexturedQuad, SpriteQuad, shader}, client_runtime::ClientHandleBox};
use aeonetica_engine::{log, Id, util::{id_map::IdMap, type_to_id, camera::Camera, vector::Vector2}, networking::messaging::ClientEntity, log_warn};

use crate::common::{Chunk, CHUNK_SIZE};

pub struct WorldModClient {

}

impl ClientMod for WorldModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!");
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
        handlers.insert(type_to_id::<WorldHandle>(), || Box::new(WorldHandle::new()));
    }

    fn start(&self, context: &mut Context, store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
        gl_context_provider.make_context();

        context.push(Rc::new(WorldLayer::instantiate()));
    }
}

pub(crate) struct WorldHandle {
    chunks: Vec<Chunk>,
    chunk_queue: Vec<Chunk>,
    tile_sprites: SpriteSheet,
    shader: Rc<shader::Program>,
}

impl WorldHandle {
    fn new() -> Self {
        Self {
            chunks: vec![],
            chunk_queue: vec![],
            tile_sprites: SpriteSheet::from_texture(
                Texture::from_bytes(include_bytes!("../assets/include/stone.png")).unwrap(),
                Vector2::new(16, 16)
            ).expect("error loading world spritesheet"),
            shader: Rc::new(shader::Program::from_source(include_str!("../assets/shaders/world.glsl")).unwrap())
        }
    }

    pub(crate) fn receive_chunk_data(&mut self, data: Chunk) {
        log_warn!("receive_chunk_data() called");
        self.chunk_queue.push(data);
    }
}

impl ClientEntity for WorldHandle {

}

impl ClientHandle for WorldHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, store: &mut DataStore) {
        messenger.register_receiver(Self::receive_chunk_data);
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut RefMut<Renderer>, delta_time: f64) {
        self.chunk_queue.drain(..).for_each(|chunk| {
            for (i, tile) in chunk.tiles().iter().enumerate() {
                let x = (i % CHUNK_SIZE * 16) as i32 + chunk.chunk_pos.x() * CHUNK_SIZE as i32;
                let y = (i / CHUNK_SIZE * 16) as i32 + chunk.chunk_pos.y() * CHUNK_SIZE as i32;
                let mut quad = SpriteQuad::new(
                    Vector2::new(x as f32, y as f32), 
                    Vector2::new(16.0, 16.0), 
                    0, 
                    self.tile_sprites.get(*tile as u32).unwrap(), 
                    self.shader.clone()
                );
                renderer.add(&mut quad);
            }

            log!("new chunk detected {:?}", chunk);
            
            self.chunks.push(chunk);
        });
    }
}

pub struct WorldLayer {
    renderer: RefCell<Renderer>,
    camera: RefCell<Camera>,
}

impl Layer for WorldLayer {
    fn instantiate() -> Self where Self: Sized {
        Self {
            renderer: RefCell::new(Renderer::new()),
            camera: RefCell::new(Camera::new(0.0, 1280.0, 720.0, 0.0, -1.0, 1.0))
        }
    }

    fn on_attach(&self) {
        log!("WorldLayer attached");
    }

    fn on_detach(&self) {
        log!("WorldLayer detached");
    }

    fn on_update(&self, handles: &mut IdMap<ClientHandleBox>, delta_time: f64) {
        let mut renderer = self.renderer.borrow_mut();
        renderer.begin_scene(&*self.camera.borrow());
        handles.iter_mut().for_each(|(_, h)| h.update(&mut renderer, delta_time));
        renderer.draw_vertices();
        renderer.end_scene();
    }

    fn on_event(&self, _handles: &mut IdMap<ClientHandleBox>, _event: &Event) -> bool {
        false
    }
}