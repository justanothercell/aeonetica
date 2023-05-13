use std::{collections::HashMap, rc::Rc, cell::{RefCell, RefMut}};

use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, data_store::DataStore, renderer::{window::{OpenGlContextProvider, events::Event}, layer::Layer, context::RenderContext, Renderer, texture::{SpriteSheet, Texture}, Quad, TexturedQuad, SpriteQuad, shader}, client_runtime::ClientHandleBox};
use aeonetica_client::renderer::context::LayerHandles;
use aeonetica_engine::{log, Id, util::{id_map::IdMap, type_to_id}, math::{camera::Camera, vector::Vector2}, networking::messaging::ClientEntity, log_warn, TypeId};

use crate::common::{Chunk, CHUNK_SIZE};

#[derive(PartialEq)]
pub struct CameraPosition(Vector2<f32>);

impl CameraPosition {
    pub fn set(&mut self, position: Vector2<f32>) {
        self.0 = position;
    }
}

pub struct WorldModClient {

}

impl ClientMod for WorldModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!");
    }

    fn register_handlers(&self, handlers: &mut IdMap<fn() -> Box<dyn ClientHandle>>, _store: &mut DataStore) {
        log!("handles registered");
        handlers.insert(type_to_id::<WorldHandle>(), || Box::new(WorldHandle::new()));
    }

    fn start(&self, context: &mut RenderContext, store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
        gl_context_provider.make_context();

        context.push(Rc::new(WorldLayer::instantiate())).expect("duplicate layer");
        store.add_store(CameraPosition(Vector2::new(0.0, 0.0)));
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

    fn owning_layer(&self) -> TypeId {
        type_to_id::<WorldLayer>()
    }

    fn update(&mut self, _messenger: &mut ClientMessenger, renderer: &mut RefMut<Renderer>, _store: &mut DataStore, _delta_time: f64) {
        self.chunk_queue.drain(..).for_each(|chunk| {
            log!("loading chunk {:?}", chunk.chunk_pos);

            for (i, tile) in chunk.tiles().iter().enumerate() {
                if *tile == 0 {
                    continue;
                }

                let x = (i % CHUNK_SIZE) as i32 + chunk.chunk_pos.x() * CHUNK_SIZE as i32;
                let y = (i / CHUNK_SIZE) as i32 + chunk.chunk_pos.y() * CHUNK_SIZE as i32;
                let mut quad = SpriteQuad::new(
                    Vector2::new(x as f32, y as f32), 
                    Vector2::new(1.0, 1.0), 
                    0, 
                    self.tile_sprites.get(*tile as u32 - 1).unwrap(), 
                    self.shader.clone()
                );
                renderer.add(&mut quad);
            }

            
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
            camera: RefCell::new(Camera::new(-24.0, 24.0, 13.5, -13.5, -1.0, 1.0))
        }
    }

    fn on_attach(&self) {
        log!("WorldLayer attached");
    }

    fn on_quit(&self) {
        log!("WorldLayer detached");
    }

    fn on_update(&self, store: &mut DataStore, handles: LayerHandles, delta_time: f64) {
        let mut renderer = self.renderer.borrow_mut();
        let mut camera = self.camera.borrow_mut();

        let new_pos = store.get_store::<CameraPosition>().0;
        if new_pos != *camera.position() {
            camera.set_position(new_pos);
        }

        renderer.begin_scene(&camera);
        handles.update(&mut renderer, store, delta_time);
        renderer.draw_vertices();
        renderer.end_scene();
    }

    fn on_event(&self, handles: &mut IdMap<ClientHandleBox>, event: &Event) -> bool {
        handles.iter_mut().any(|(_, h)| h.on_event(event))
    }
}