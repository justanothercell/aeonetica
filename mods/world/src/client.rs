use std::{rc::Rc};
use std::collections::HashMap;
use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, data_store::DataStore, renderer::{window::{OpenGlContextProvider, events::Event}, layer::Layer, context::RenderContext, Renderer, texture::{SpriteSheet, Texture}, Quad, TexturedQuad, SpriteQuad, shader}, client_runtime::ClientHandleBox};
use aeonetica_engine::{log, util::{id_map::IdMap, type_to_id}, math::{camera::Camera, vector::Vector2}, networking::messaging::ClientEntity, *};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::nullable::Nullable;

use crate::common::{Chunk, CHUNK_SIZE};
use crate::server::world::World;

#[allow(clippy::large_enum_variant)]
pub enum ClientChunk {
    Requested,
    Chunk(Chunk, Vec<SpriteQuad>)
}

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
        println!("started worldmodclient");
        context.push(WorldLayer::new()).expect("duplicate layer");
        context.push(UILayer::new()).expect("duplicate layer");
        store.add_store(CameraPosition(Vector2::new(0.0, 0.0)));
    }
}

pub(crate) struct WorldHandle {
    chunks: HashMap<Vector2<i32>, ClientChunk>,
    chunk_queue: Vec<Chunk>,
    tile_sprites: SpriteSheet,
    shader: Rc<shader::Program>,
}

impl WorldHandle {
    fn new() -> Self {
        Self {
            chunks: Default::default(),
            chunk_queue: vec![],
            tile_sprites: SpriteSheet::from_texture(
                Texture::from_bytes(include_bytes!("../assets/include/stone.png")).unwrap(),
                Vector2::new(16, 16)
            ).expect("error loading world spritesheet"),
            shader: Rc::new(shader::Program::from_source(include_str!("../assets/shaders/world.glsl")).unwrap())
        }
    }

    pub(crate) fn receive_chunk_data(&mut self, chunk: Chunk) {
        log!(DEBUG, "receive_chunk_data {:?}", chunk.chunk_pos);
        self.chunk_queue.push(chunk);
    }
}

impl ClientEntity for WorldHandle {

}

impl ClientHandle for WorldHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, _renderer: Nullable<&mut Renderer>, _store: &mut DataStore) {
        messenger.register_receiver(Self::receive_chunk_data);
    }

    fn owning_layer(&self) -> TypeId {
        type_to_id::<WorldLayer>()
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut Renderer, store: &mut DataStore, _delta_time: f64) {
        let center_chunk: Vector2<_> = store.get_store::<CameraPosition>().0.ceil().round_i32() / Vector2::from((CHUNK_SIZE as i32, CHUNK_SIZE as i32));
        for x in (center_chunk.x-1)..=(center_chunk.x+1) {
            for y in (center_chunk.y-1)..=(center_chunk.y+1) {
                let k = Vector2::from((x, y));
                self.chunks.entry(k).or_insert_with(|| {
                    messenger.call_server_fn(World::request_world_chunk, k, SendMode::Safe);
                    ClientChunk::Requested
                });
            }
        }

        self.chunks.retain(|k, v|{
            if (*k - center_chunk).mag_sq() > 2 {
                log!("unloading chunk {:?}", k);
                if let ClientChunk::Chunk(_, quads) = v {
                    for quad in quads {
                        renderer.remove(quad);
                    }
                }
                false
            } else { true }
        });

        for chunk in self.chunk_queue.drain(..) {
            let mut quads = vec![];
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
                quads.push(quad);
            }
            self.chunks.insert(chunk.chunk_pos, ClientChunk::Chunk(chunk, quads));
            log!(DEBUG, "loaded chunks: {}", self.chunks.len());
        }
    }
}

pub struct WorldLayer;

impl WorldLayer {
    fn new() -> Self {
        Self {}
    }
}

impl Layer for WorldLayer {
    fn instantiate_camera(&self) -> Camera {
        Camera::new(-24.0, 24.0, 13.5, -13.5, -1.0, 1.0)
    }

    fn update_camera(&mut self, store: &mut DataStore, camera: &mut Camera, delta_time: f64) {
        let new_pos = store.get_store::<CameraPosition>().0;
        if new_pos != *camera.position() {
            camera.set_position(new_pos);
        }
    }
}

pub struct UILayer {

}

impl UILayer {
    fn new() -> Self {
        Self {
             
        }
    }
}

impl Layer for UILayer {
    fn instantiate_camera(&self) -> Camera {
        Camera::new(-24.0, 24.0, 13.5, -13.5, -1.0, 1.0)
    }

    fn attach(&mut self, _renderer: &mut Renderer) {
        log!(ERROR, "UI layer attached")
    }

    fn is_overlay(&self) -> bool { true }
}
