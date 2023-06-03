use std::collections::HashMap;
use noise::{Fbm, NoiseFn, Perlin};
use aeonetica_client::renderer::material::FlatTexture;
use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, data_store::DataStore, renderer::{layer::Layer, context::RenderContext, Renderer, texture::{SpriteSheet, Texture}, builtin::Quad}};
use aeonetica_client::renderer::window::events::{Event, KeyCode};
use aeonetica_client::renderer::window::OpenGlRenderContextProvider;
use aeonetica_engine::{log, TypeId};
use aeonetica_engine::math::camera::Camera;
use aeonetica_engine::math::vector::Vector2;
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::nullable::Nullable;
use aeonetica_engine::util::type_to_id;
use aeonetica_engine::error::ExpectLog;
use aeonetica_engine::time::Time;

use crate::client::pipeline::WorldRenderPipeline;
use crate::client::materials::{WithGlow, WithTerrain};

use crate::common::{Chunk, CHUNK_SIZE, WorldView};
use crate::server::world::World;
use crate::tiles::Tile;

use debug_mod::Debug;

use self::materials::GlowTexture;

mod pipeline;
pub mod materials;

#[allow(clippy::large_enum_variant)]
pub enum ClientChunk {
    Requested,
    Chunk(Chunk, Vec<Block>)
}

#[derive(PartialEq)]
pub struct CameraData {
    pub position: Vector2<f32>,
    trauma: f32
}


impl CameraData {
    pub fn add_trauma(&mut self, trauma: f32) {
        self.trauma = (self.trauma + trauma).clamp(0.0, 1.0);
    }

    pub fn clear_trauma(&mut self) {
        self.trauma = 0.0;
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
        store.add_default::<Debug<WorldLayer>>();
        store.add_store(CameraData {
            position: Vector2::new(0.0, 0.0),
            trauma: 0.0,
        });
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
    tile_sprites: SpriteSheet,
}

impl WorldHandle {
    fn new() -> Self {
        Self {
            tile_sprites: SpriteSheet::from_texture(
                Texture::from_bytes(include_bytes!("../../assets/include/tilemap.png")).unwrap(),
                Vector2::new(16, 16)
            ).expect("error loading world spritesheet"),
        }
    }

    pub(crate) fn receive_chunk_data(&mut self, _messenger: &mut ClientMessenger, mut renderer: Nullable<&mut Renderer>, store: &mut DataStore, chunk: Chunk) {
        let mut quads = vec![];
        let chunks = &mut store.mut_store::<ClientWorld>().chunks;
        for (i, tile) in chunk.tiles().iter().enumerate() {
            let index = tile.sprite_sheet_index();
            if index == 0 {
                continue;
            }

            let x = (i % CHUNK_SIZE) as i32 + chunk.chunk_pos.x() * CHUNK_SIZE as i32;
            let y = (i / CHUNK_SIZE) as i32 + chunk.chunk_pos.y() * CHUNK_SIZE as i32;

            if index == Tile::Lamp as u16 {
                let mut quad = Quad::with_glow_sprite(
                    Vector2::new(x as f32, y as f32), 
                    Vector2::new(1.0, 1.0), 
                    1, 
                    self.tile_sprites.get(index as u32 - 1).unwrap(),
                [0.9, 0.8, 0.5, 1.0]
                );
                renderer.add(&mut quad);
                quads.push(Block::Glowing(quad));
            }
            else {
                let mut quad = Quad::with_terrain_sprite(
                    Vector2::new(x as f32, y as f32), 
                    Vector2::new(1.0, 1.0), 
                    0, 
                    self.tile_sprites.get(index as u32 - 1).unwrap(),
                );
                renderer.add(&mut quad);
                quads.push(Block::Default(quad));
            }
        }
        chunks.insert(chunk.chunk_pos, ClientChunk::Chunk(chunk, quads));
    }
}

impl ClientEntity for WorldHandle {

}

pub enum Block {
    Default(Quad<FlatTexture>),
    Glowing(Quad<GlowTexture>)
}

impl Block {
    fn remove_from(&mut self, renderer: &mut Renderer) {
        match self {
            Self::Default(quad) => renderer.remove(quad),
            Self::Glowing(quad) => renderer.remove(quad)
        }
    }
}

impl ClientHandle for WorldHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, _renderer: Nullable<&mut Renderer>, _store: &mut DataStore) {
        messenger.register_receiver(Self::receive_chunk_data);
    }

    fn owning_layer(&self) -> TypeId {
        type_to_id::<WorldLayer>()
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut Renderer, store: &mut DataStore, _time: Time) {
        let cam = store.get_store::<CameraData>().position;
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
                        quad.remove_from(renderer);
                    }
                }
                false
            } else { true }
        });
    }
}

pub struct WorldLayer {
    shake_noise: Box<dyn NoiseFn<f64, 2>>,
    manual_shake_queued: bool,
}

impl WorldLayer {
    fn new() -> Self {
        Self {
            shake_noise: Box::new(Fbm::<Perlin>::new(0)),
            manual_shake_queued: false,
        }
    }
}

impl Layer for WorldLayer {
    fn attach(&mut self, renderer: &mut Renderer) {
        renderer.set_pipeline(WorldRenderPipeline::new().expect_log());
    }

    fn instantiate_camera(&self) -> Camera {
        Camera::new(-24.0, 24.0, 13.5, -13.5, -1.0, 1.0)
    }

    fn update_camera(&mut self, store: &mut DataStore, camera: &mut Camera, time: Time) {
        let mut cam = store.mut_store::<CameraData>();
        if self.manual_shake_queued {
            cam.add_trauma(0.2);
            self.manual_shake_queued = false;
        }
        // easing f(t) = t - t² + t³
        let shake = cam.trauma - cam.trauma * cam.trauma + cam.trauma * cam.trauma * cam.trauma;
        let pos = cam.position + Vector2::new(self.shake_noise.get([time.time as f64 * 5.0, 0.0]) as f32, self.shake_noise.get([time.time as f64 * 5.0, 123.51]) as f32) * shake * 1.5;
        camera.set_position(pos);

        cam.trauma = (cam.trauma - time.delta as f32 / 3.0).clamp(0.0, 1.0);
        camera.set_rotation(self.shake_noise.get([time.time as f64 * 5.0, 732.183]) as f32 * shake * 0.0);
        cam.trauma = (cam.trauma - time.delta as f32 / 3.0).clamp(0.0, 1.0);
    }

    fn pre_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, _time: Time) {
        store.mut_store::<Debug<WorldLayer>>().renderer().start_render(renderer);
    }

    fn post_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, _time: Time) {
        store.mut_store::<Debug<WorldLayer>>().renderer().finish_render(renderer);
    }

    fn event(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyPressed(KeyCode::Enter) => {
                self.manual_shake_queued = true;
                true
            }
            _ => false
        }
    }
}