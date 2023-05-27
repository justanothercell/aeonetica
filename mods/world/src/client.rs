use std::collections::HashMap;
use aeonetica_client::renderer::builtin::{Line, TextArea};
use aeonetica_client::renderer::material::FlatTexture;
use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, data_store::DataStore, renderer::{window::{OpenGlContextProvider}, layer::Layer, context::RenderContext, Renderer, texture::{SpriteSheet, Texture}, builtin::Quad}};
use aeonetica_client::renderer::window::OpenGlRenderContextProvider;
use aeonetica_engine::{log, util::{id_map::IdMap, type_to_id}, math::{camera::Camera, vector::Vector2}, networking::messaging::ClientEntity, *};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::nullable::Nullable;

use crate::common::{Chunk, CHUNK_SIZE, WorldView};
use crate::server::world::World;
use crate::tiles::Tile;

#[allow(clippy::large_enum_variant)]
pub enum ClientChunk {
    Requested,
    Chunk(Chunk, Vec<Quad<FlatTexture>>)
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

    fn start<'a>(&self, store: &mut DataStore, provider: OpenGlRenderContextProvider<'a>) -> &'a mut RenderContext {
        let context = provider.make_context();
        println!("started worldmodclient");
        store.add_store(ClientWorld {
            chunks: Default::default(),
        });
        context.push(WorldLayer::new()).expect("duplicate layer");
        context.push(UILayer::new()).expect("duplicate layer");
        store.add_store(CameraPosition(Vector2::new(0.0, 0.0)));
        context
    }
}

pub struct ClientWorld {
    chunks: HashMap<Vector2<i32>, ClientChunk>
}

impl WorldView for ClientWorld {
    fn is_loaded(&self, pos: Vector2<i32>) -> bool {
        self.chunks.contains_key(&Self::chunk(pos))
    }

    fn get_tile_or_null(&self, pos: Vector2<i32>) -> Nullable<Tile> {
        if let ClientChunk::Chunk(chunk, _) = self.chunks.get(&Self::chunk(pos))? {
            return Nullable::Value(chunk.get_tile(Self::pos_in_chunk(pos)))
        }
        Nullable::Null
    }
}

pub(crate) struct WorldHandle {
    chunk_queue: Vec<Chunk>,
    tile_sprites: SpriteSheet,
}

impl WorldHandle {
    fn new() -> Self {
        Self {
            chunk_queue: vec![],
            tile_sprites: SpriteSheet::from_texture(
                Texture::from_bytes(include_bytes!("../assets/include/tilemap.png")).unwrap(),
                Vector2::new(16, 16)
            ).expect("error loading world spritesheet"),
        }
    }

    pub(crate) fn receive_chunk_data(&mut self, messenger: &mut ClientMessenger, renderer: Nullable<&mut Renderer>, store: &mut DataStore, chunk: Chunk) {
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
        let cam = store.get_store::<CameraPosition>().0;
        let chunks = &mut store.mut_store::<ClientWorld>().chunks;
        let center_chunk: Vector2<_> = (cam / Vector2::from((CHUNK_SIZE as f32, CHUNK_SIZE as f32))).floor().to_i32();
        for x in (center_chunk.x-2)..=(center_chunk.x+2) {
            for y in (center_chunk.y-1)..=(center_chunk.y+1) {
                let k = Vector2::from((x, y));
                chunks.entry(k).or_insert_with(|| {
                    messenger.call_server_fn(World::request_world_chunk, k, SendMode::Safe);
                    ClientChunk::Requested
                });
            }
        }

        chunks.retain(|k, v|{
            let d = *k - center_chunk;
            if d.x.abs() > 2 || d.y.abs() > 2 {
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

            for (i, tile) in chunk.tiles().iter().enumerate() {
                let index = tile.sprite_sheet_index();
                if index == 0 {
                    continue;
                }

                let x = (i % CHUNK_SIZE) as i32 + chunk.chunk_pos.x() * CHUNK_SIZE as i32;
                let y = (i / CHUNK_SIZE) as i32 + chunk.chunk_pos.y() * CHUNK_SIZE as i32;
                let mut quad = Quad::with_sprite(
                    Vector2::new(x as f32, y as f32), 
                    Vector2::new(1.0, 1.0), 
                    0, 
                    self.tile_sprites.get(index as u32 - 1).unwrap(),
                );
                renderer.add(&mut quad);
                quads.push(quad);
            }
            chunks.insert(chunk.chunk_pos, ClientChunk::Chunk(chunk, quads));
            log!(DEBUG, "loaded chunks: {}", chunks.len());
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
    fn attach(&mut self, renderer: &mut Renderer) {
        let mut line = Line::new(Vector2::new(0.0, 0.0), Vector2::new(20.0, 10.0), 0.1, 2, [1.0, 0.0, 1.0, 1.0]);
        renderer.add(&mut line);
    }

    fn instantiate_camera(&self) -> Camera {
        Camera::new(-24.0, 24.0, 13.5, -13.5, -1.0, 1.0)
    }

    fn update_camera(&mut self, store: &mut DataStore, camera: &mut Camera, _delta_time: f64) {
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
