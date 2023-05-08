use std::cell::RefMut;
use std::rc::Rc;
use aeonetica_client::ClientMod;
use aeonetica_client::data_store::DataStore;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_client::renderer::context::Context;
use aeonetica_client::renderer::{Renderer, TexturedQuad, Renderable, Quad, shader};
use aeonetica_client::renderer::texture::Texture;
use aeonetica_client::renderer::window::OpenGlContextProvider;
use aeonetica_engine::log;
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::type_to_id;
use aeonetica_engine::util::vector::Vector2;
use crate::server::Player;

pub struct PlayerModClient {

}

impl ClientMod for PlayerModClient {
    fn start(&self, _context: &mut Context, _store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
        log!("started client player mod");
        gl_context_provider.make_context();
    }
    fn register_handlers(&self, handlers: &mut IdMap<fn() -> Box<dyn ClientHandle>>, _store: &mut DataStore) {
        handlers.insert(type_to_id::<PlayerHandle>(), || Box::new(PlayerHandle::new()));
        log!("registered  client player mod stuffs");
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
    p_position: Vector2<f32>,
    position: Vector2<f32>,

    // rendering stuff
    quad: Option<TexturedQuad>,
}

impl PlayerHandle {
    fn new() -> Self {
        Self {
            is_controlling: false,
            p_position: Default::default(),
            position: Default::default(),
            quad: None,
        }
    }

    pub(crate) fn set_controlling(&mut self, is_controlling: bool) {
        log!("got elevated to controller");
        self.is_controlling = is_controlling
    }

    pub(crate) fn receive_position(&mut self, position: Vector2<f32>) {
        if !self.is_controlling {
            log!("received position from foreign client");
            self.position = position
        } else {
            log!("received position pongback");
        }
    }
}

impl ClientEntity for PlayerHandle {}

impl ClientHandle for PlayerHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, store: &mut DataStore) {
        messenger.register_receiver(Self::set_controlling);
        messenger.register_receiver(Self::receive_position);

        self.quad = Some(
            TexturedQuad::new(
            self.position,
            Vector2::new(1.0, 1.0),
            1,
            store.get_or_create(|| PlayerTexture::load()).get().id(),
            store.get_or_create(|| PlayerShader::load()).get().clone()
        ))
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut RefMut<Renderer>, delta_time: f64) {
        let quad = self.quad.as_mut().unwrap();
        if self.is_controlling {
            self.position.x += delta_time as f32;
            if (self.position - self.p_position).mag_sq() > 70.0 {
                messenger.call_server_fn(Player::client_position_update, self.position, SendMode::Quick);
                log!("told server i moved");
                self.p_position = self.position;
            }

            quad.set_position(self.position.clone());
        }

        let _ = renderer.draw(quad);
    }
}
