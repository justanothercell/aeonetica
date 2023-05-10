use std::cell::RefMut;
use std::rc::Rc;
use aeonetica_client::ClientMod;
use aeonetica_client::data_store::DataStore;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_client::renderer::context::Context;
use aeonetica_client::renderer::window::events::{Event, KeyCode};
use aeonetica_client::renderer::{Renderer, TexturedQuad, Renderable, Quad, shader};
use aeonetica_client::renderer::texture::Texture;
use aeonetica_client::renderer::window::OpenGlContextProvider;
use aeonetica_engine::{log, log_warn};
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::nullable::Nullable;
use aeonetica_engine::util::nullable::Nullable::{Null, Value};
use aeonetica_engine::util::type_to_id;
use aeonetica_engine::util::vector::Vector2;
use crate::server::Player;

pub struct PlayerModClient {

}

impl ClientMod for PlayerModClient {
    fn register_handlers(&self, handlers: &mut IdMap<fn() -> Box<dyn ClientHandle>>, _store: &mut DataStore) {
        handlers.insert(type_to_id::<PlayerHandle>(), || Box::new(PlayerHandle::new()));
        log!("registered  client player mod stuffs");
    }
    fn start(&self, _context: &mut Context, _store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
        log!("started client player mod");
        gl_context_provider.make_context();
    }
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

#[derive(Clone)]
struct PlayerShader(Rc<shader::Program>);

impl PlayerShader {
    fn load() -> Self {
        Self(Rc::new(shader::Program::from_source(include_str!("../assets/shaders/player.glsl")).expect("error loading player shader")))
    }

    fn get(&self) -> &Rc<shader::Program> {
        &self.0
    }
}

pub struct PlayerHandle {
    is_controlling: bool,
    interpolation_delta: f32,
    p_position: Vector2<f32>,
    position: Vector2<f32>,

    // rendering stuff
    quad: Nullable<TexturedQuad>,

    // movement stuff
    key_state: [bool; 4],
    speed: f32,
}

impl PlayerHandle {
    fn new() -> Self {
        Self {
            is_controlling: false,
            interpolation_delta: 0.0,
            p_position: Default::default(),
            position: Default::default(),
            quad: Null,
            key_state: [false; 4],
            speed: 6.0
        }
    }

    pub(crate) fn set_controlling(&mut self, is_controlling: bool) {
        log!("got elevated to controller");
        self.is_controlling = is_controlling
    }

    pub(crate) fn receive_position(&mut self, position: Vector2<f32>) {
        if !self.is_controlling {
            self.p_position = self.p_position + (self.position - self.p_position) * self.interpolation_delta;
            self.interpolation_delta = 0.0;
            self.position = position;
            let quad = &mut *self.quad;
            quad.set_position(self.position);
        }
    }
}

impl ClientEntity for PlayerHandle {}

impl ClientHandle for PlayerHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, store: &mut DataStore) {
        messenger.register_receiver(Self::set_controlling);
        messenger.register_receiver(Self::receive_position);

        self.quad = Value(
            TexturedQuad::new(
            self.position,
            Vector2::new(1.0, 1.0),
            1,
            store.get_or_create(PlayerTexture::load).get().id(),
            store.get_or_create(PlayerShader::load).get().clone()
        ))
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut RefMut<Renderer>, delta_time: f64) {
        let quad = &mut *self.quad;
        if self.is_controlling {
            match self.key_state {
                [true, _, false, _] => self.position.y -= self.speed * delta_time as f32,
                [false, _, true, _] => self.position.y += self.speed * delta_time as f32,
                _ => {}
            }

            match self.key_state {
                [_, true, _, false] => self.position.x -= self.speed * delta_time as f32,
                [_, false, _, true] => self.position.x += self.speed * delta_time as f32,
                _ => {}
            }

            if (self.position - self.p_position).mag_sq() > 0.05 {
                messenger.call_server_fn(Player::client_position_update, self.position, SendMode::Quick);
                self.p_position = self.position;
            }

            quad.set_position(self.position);
        } else if self.interpolation_delta * self.interpolation_delta < 1.0 {
            let delta = self.position - self.p_position;
            quad.set_position(self.p_position + delta * self.interpolation_delta);
            self.interpolation_delta = (delta_time as f32 * self.speed + self.interpolation_delta).min(1.0);
        }

        let _ = renderer.draw(quad);
    }

    fn on_event(&mut self, event: &Event) -> bool {
        if !self.is_controlling { return false }
        match event {
            Event::KeyPressed(KeyCode::W) => {
                self.key_state[0] = true;
                true
            }
            Event::KeyPressed(KeyCode::A) => {
                self.key_state[1] = true;
                true
            }
            Event::KeyPressed(KeyCode::S) => {
                self.key_state[2] = true;
                true
            }
            Event::KeyPressed(KeyCode::D) => {
                self.key_state[3] = true;
                true
            }
            Event::KeyReleased(KeyCode::W) => {
                self.key_state[0] = false;
                true
            }
            Event::KeyReleased(KeyCode::A) => {
                self.key_state[1] = false;
                true
            }
            Event::KeyReleased(KeyCode::S) => {
                self.key_state[2] = false;
                true
            }
            Event::KeyReleased(KeyCode::D) => {
                self.key_state[3] = false;
                true
            }
            _ => false
        }
    }
}
