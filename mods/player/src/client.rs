use std::ops::{Add, Mul, Sub};
use std::rc::Rc;
use aeonetica_client::ClientMod;
use aeonetica_client::data_store::DataStore;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_client::renderer::material::{FlatTexture};
use aeonetica_client::renderer::window::events::{Event, KeyCode};
use aeonetica_client::renderer::{Renderer, builtin::Quad};
use aeonetica_client::renderer::builtin::Line;
use aeonetica_client::renderer::context::RenderContext;
use aeonetica_client::renderer::layer::Layer;
use aeonetica_client::renderer::texture::Texture;
use aeonetica_client::renderer::window::OpenGlRenderContextProvider;
use aeonetica_engine::{log, TypeId};
use aeonetica_engine::time::Time;
use aeonetica_engine::math::camera::Camera;
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::nullable::Nullable;
use aeonetica_engine::util::nullable::Nullable::{Null, Value};
use aeonetica_engine::util::type_to_id;
use aeonetica_engine::math::vector::Vector2;
use debug_mod::Debug;
use world_mod::common::{GRAVITY, WorldView};
use world_mod::client::{ClientWorld, WorldLayer};
use world_mod::client::CameraData;
use world_mod::client::materials::WithTerrain;
use crate::server::Player;

pub struct PlayerModClient {

}

impl ClientMod for PlayerModClient {
    fn register_handlers(&self, handlers: &mut IdMap<fn() -> Box<dyn ClientHandle>>, _store: &mut DataStore) {
        handlers.insert(type_to_id::<PlayerHandle>(), || Box::new(PlayerHandle::new()));
        log!("registered  client player mod stuffs");
    }

    fn start<'a>(&self, store: &mut DataStore, provider: OpenGlRenderContextProvider<'a>) -> &'a mut RenderContext {
        let context = provider.make_context();

        context.push(OverlayLayer::new(), store).expect("duplicate layer");
        store.add_store(PlayerUIView {
            hover_energy: 1.0
        });
        context
    }
}

struct PlayerUIView {
    hover_energy: f32,
}

#[derive(Clone)]
struct PlayerTexture(Rc<Texture>);

impl PlayerTexture {
    fn load() -> Self {
        Self(Rc::new(Texture::from_bytes(include_bytes!("../assets/include/player.png")).expect("error loading player texture")))
    }   

    fn get(&self) -> &Rc<Texture> {
        &self.0
    }
}

pub struct PlayerHandle {
    is_controlling: bool,
    interpolation_delta: f32,
    p_position: Vector2<f32>,
    position: Vector2<f32>,

    // rendering stuff
    quad: Nullable<Quad<FlatTexture>>,

    // movement stuff
    key_left: bool,
    key_right: bool,
    key_hover: bool,
    speed: f32,
    hover_force: f32,
    hover_energy: f32,
    is_grounded: bool,
    velocity: Vector2<f32>
}

impl PlayerHandle {
    fn new() -> Self {
        Self {
            is_controlling: false,
            interpolation_delta: 0.0,
            p_position: Default::default(),
            position: Default::default(),
            quad: Null,

            key_left: false,
            key_right: false,
            key_hover: false,
            speed: 10.0,
            hover_force: 12.0,
            hover_energy: 1.0,
            is_grounded: false,
            velocity: Default::default(),
        }
    }

    pub(crate) fn set_controlling(&mut self, _messenger: &mut ClientMessenger, _renderer: Nullable<&mut Renderer>, _store: &mut DataStore, is_controlling: bool) {
        log!("got elevated to controller");
        self.is_controlling = is_controlling
    }

    pub(crate) fn receive_position(&mut self, _messenger: &mut ClientMessenger, _renderer: Nullable<&mut Renderer>, _store: &mut DataStore, (position, teleporting): (Vector2<f32>, bool)) {
        if !self.is_controlling {
            if teleporting {
                self.p_position = position;
                self.interpolation_delta = 1.0;
                self.position = position;
                let quad = &mut *self.quad;
                quad.set_position(self.position);
            } else {
                self.p_position = self.p_position + (self.position - self.p_position) * self.interpolation_delta;
                self.interpolation_delta = 0.0;
                self.position = position;
            }
        }
    }
}

impl ClientEntity for PlayerHandle {}

const PLAYER_SIZE: f32 = 0.9;

impl ClientHandle for PlayerHandle {
    fn owning_layer(&self) -> TypeId {
        type_to_id::<WorldLayer>()
    }

    fn start(&mut self, messenger: &mut ClientMessenger, _renderer: Nullable<&mut Renderer>, store: &mut DataStore) {
        messenger.register_receiver(Self::set_controlling);
        messenger.register_receiver(Self::receive_position);

        self.quad = Value(
            Quad::with_terrain_texture(
            self.position,
            Vector2::new(PLAYER_SIZE, PLAYER_SIZE),
            10,
            store.get_or_create(PlayerTexture::load).get().id(),
        ))
    }

    fn remove(&mut self, _messenger: &mut ClientMessenger, mut renderer: Nullable<&mut Renderer>, _store: &mut DataStore) {
        renderer.remove(&mut *self.quad);
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut Renderer, store: &mut DataStore, time: Time) {
        let quad = &mut *self.quad;
        if self.is_controlling {
            self.velocity.y -= GRAVITY * time.delta as f32;
            if self.key_hover {
                if self.hover_energy > 0.0 {
                    self.velocity.y = (self.velocity.y * (1.0 - time.delta as f32 * 10.0)) - self.hover_force * time.delta as f32 * 10.0;
                }
                self.hover_energy = self.hover_energy.sub(0.75 * time.delta as f32).max(0.0);
            } else {
                self.hover_energy = self.hover_energy.add( if self.is_grounded { 1.0 } else { 0.125 } * time.delta as f32).min(1.0);
            }
            store.mut_store::<PlayerUIView>().hover_energy = self.hover_energy;

            self.velocity.y -= self.velocity.y.abs().mul(0.25).max(0.025).mul(time.delta as f32).min(self.velocity.y.abs()).copysign(self.velocity.x);
            self.velocity.x -= self.velocity.x.abs().mul(0.25).max(0.025).mul(time.delta as f32).min(self.velocity.x.abs()).copysign(self.velocity.x);
            let v = if self.velocity.x.abs() < 0.05 {
                self.velocity.x = 0.0;
                self.velocity + Vector2::new(match (self.key_left, self.key_right) {
                    (true, false) => -self.speed,
                    (false, true) => self.speed,
                    _ => 0.0
                }, 0.0)
            } else { self.velocity };

            if v.mag_sq() > 0.0 {
                let world = store.get_store::<ClientWorld>();
                let p = self.position;
                let mov_delta = v * time.delta as f32;
                world.calc_move(&mut self.position, Vector2::new(PLAYER_SIZE, PLAYER_SIZE), mov_delta);
                let delta = self.position - p;
                if (delta - mov_delta).mag_sq() < 0.001 && (self.p_position - self.position).mag_sq() > 0.001 {
                    messenger.call_server_fn(Player::client_position_update, (self.position, false), SendMode::Safe);
                    self.p_position = self.position;
                }
                if delta.x.abs() < 0.01 * time.delta as f32 {
                    self.velocity.x = 0.0;
                }
                if delta.y.abs() < 0.01 * time.delta as f32 {
                    if self.velocity.y > 16.0 {
                        store.mut_store::<CameraData>().add_trauma((self.velocity.y / 50.0) * (self.velocity.y / 50.0));
                    }
                    self.velocity.y = 0.0;
                    let world = store.get_store::<ClientWorld>();
                    self.is_grounded = world.overlap_aabb(self.position + Vector2::new(0.0, 0.02), Vector2::new(PLAYER_SIZE, PLAYER_SIZE));
                } else {
                    self.is_grounded = false;
                }
            }

            if (self.position - self.p_position).mag_sq() > 0.05 {
                messenger.call_server_fn(Player::client_position_update, (self.position, false), SendMode::Quick);
                self.p_position = self.position;
            }
            quad.set_position(self.position);
            store.mut_store::<CameraData>().position = self.position;
        } else if self.interpolation_delta < 1.0 {
            let delta = self.position - self.p_position;
            let debug = store.get_store::<Debug<WorldLayer>>();
            let mut debug = debug.renderer();
            debug.rect(self.position, Vector2::new(0.9, 0.9), 0.1, [1.0, 0.0, 0.0, 1.0]);
            quad.set_position(self.p_position + delta * self.interpolation_delta);
            self.interpolation_delta = (time.delta as f32 * self.speed + self.interpolation_delta).min(1.0);
        }

        let _ = renderer.draw(quad);
    }

    fn event(&mut self, event: &Event, _messenger: &mut ClientMessenger, _renderer: &mut Renderer, _store: &mut DataStore) -> bool {
        if !self.is_controlling { return false }
        match event {
            Event::KeyPressed(KeyCode::Space) => {
                self.key_hover = true;
                true
            }
            Event::KeyPressed(KeyCode::A) => {
                self.key_left = true;
                true
            }
            Event::KeyPressed(KeyCode::D) => {
                self.key_right = true;
                true
            }
            Event::KeyReleased(KeyCode::Space) => {
                self.key_hover = false;
                true
            }
            Event::KeyReleased(KeyCode::A) => {
                self.key_left = false;
                true
            }
            Event::KeyReleased(KeyCode::D) => {
                self.key_right = false;
                true
            }
            _ => false
        }
    }
}


pub struct OverlayLayer {
    hover_energy_bar: Line,
    hover_energy_bar_bg: Line,
}

impl OverlayLayer {
    fn new() -> Self {
        Self {
            hover_energy_bar: Line::new(Vector2::new(-0.25, -0.5), Vector2::new(1.25, -0.5), 0.3,  11, [0.0, 0.3, 1.0, 1.0]),
            hover_energy_bar_bg: Line::new(Vector2::new(-0.25, -0.5), Vector2::new(1.25, -0.5), 0.3,  10, [0.5, 0.0, 0.0, 1.0]),
        }
    }
}

impl Layer for OverlayLayer {
    fn instantiate_camera(&self) -> Camera {
        Camera::new(-24.0, 24.0, 13.5, -13.5, -1.0, 1.0)
    }

    fn attach(&mut self, renderer: &mut Renderer, store: &mut DataStore) {
        log!(ERROR, "UI layer attached");
        renderer.add(&mut self.hover_energy_bar_bg);
    }

    fn post_handles_update(&mut self, store: &mut DataStore, renderer: &mut Renderer, _time: Time) {
        let hover_energy = store.get_store::<PlayerUIView>().hover_energy;

        self.hover_energy_bar.set_to(*self.hover_energy_bar.from() + Vector2::new(1.5 * hover_energy, 0.0));
        renderer.draw(&mut self.hover_energy_bar).expect("err drawing");
    }

    fn is_overlay(&self) -> bool { true }
}
