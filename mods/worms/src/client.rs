use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, renderer::{Renderer, texture::{SpriteSheet, Texture}, builtin::Quad, material::FlatTexture}, data_store::DataStore};
use aeonetica_engine::{networking::messaging::ClientEntity, util::{type_to_id, nullable::Nullable}, math::vector::Vector2};
use world_mod::client::WorldLayer;

use crate::server::Worm;


pub struct WormsModClient {

}

impl WormsModClient {
    pub(crate) fn new() -> Self {
        println!("created wormmodclient");
        Self {}
    }
}

impl ClientMod for WormsModClient {
    fn register_handlers(&self, handlers: &mut aeonetica_engine::util::id_map::IdMap<fn() -> Box<dyn ClientHandle>>, store: &mut DataStore) {
        handlers.insert(type_to_id::<WormHandle>(),  WormHandle::new_boxed);
    }
}

struct WormSheet(SpriteSheet);

impl WormSheet {
    fn load() -> Self {
        println!("loaded textures");
        Self(SpriteSheet::from_texture(Texture::from_bytes(include_bytes!("../assets/include/wormsheet.png")).expect("err loading texture"),
            Vector2::new(16, 16)).expect("err loading worm sheet"))
    }
}

pub(crate) struct WormHandle {
    quads: Vec<Quad<FlatTexture>>,
    segments: Vec<Vector2<f32>>,
    p_segments: Vec<Vector2<f32>>,
    interpolation_delta: f32,
}

impl WormHandle {
    fn new_boxed() -> Box<dyn ClientHandle> {
        Box::new(Self{
            quads: vec![],
            segments: vec![],
            p_segments: vec![],
            interpolation_delta: 1.0,
        })
    }

    pub(crate) fn receive_position(&mut self, _messenger: &mut ClientMessenger, mut renderer: Nullable<&mut Renderer>, store: &mut DataStore, (segments, teleporting): (Vec<Vector2<f32>>, bool)) {
        if self.segments.len() == 0 {
            let sheet = store.get_or_create(WormSheet::load);
            self.p_segments = segments.clone();
            self.segments = segments.clone();
            self.interpolation_delta = 1.0;
            for (i, segment) in self.segments.iter().enumerate() {
                let quad = Quad::with_sprite(
                    *segment,
                    Vector2::new(1.0, 1.0),
                    100,
                    sheet.0.get(match i { 0 => 0, _ if i == self.segments.len() - 1 => 2, _ => 1 }).unwrap(),
                );
                self.quads.push(quad);
            }
            self.quads.iter_mut().rev().for_each(|quad| renderer.draw(quad).expect("unable to draw quad"));
        } else {
            for (i, segment) in self.segments.iter().enumerate() {
                self.quads[i].set_position(*segment);
                renderer.draw(&mut self.quads[i]).expect("unable to draw quad");
            }
        }

        if teleporting {
            self.p_segments = segments.clone();
            self.interpolation_delta = 1.0;
            self.segments = segments;
        } else {
            self.p_segments = self.p_segments.iter().zip(&self.segments).map(|(&ps, &s)| ps + (s - ps) * self.interpolation_delta).collect();
            self.interpolation_delta = 0.0;
            self.segments = segments;
        }
    }
}

impl ClientEntity for WormHandle {}

impl ClientHandle for WormHandle {
    fn owning_layer(&self) -> aeonetica_engine::TypeId {
        type_to_id::<WorldLayer>()
    }

    fn start(&mut self, messenger: &mut ClientMessenger, _renderer: Nullable<&mut Renderer>, _store: &mut DataStore) {
        messenger.register_receiver(WormHandle::receive_position)
    }
}