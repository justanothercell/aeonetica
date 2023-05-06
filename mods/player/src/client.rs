use std::cell::RefMut;
use aeonetica_client::ClientMod;
use aeonetica_client::data_store::DataStore;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_client::renderer::Renderer;
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
    fn register_handlers(&self, handlers: &mut IdMap<fn() -> Box<dyn ClientHandle>>) {
        handlers.insert(type_to_id::<PlayerHandle>(), || Box::new(PlayerHandle { is_controlling: false, p_position: Default::default(), position: Default::default() }));
    }
}

pub struct PlayerHandle {
    is_controlling: bool,
    p_position: Vector2<f32>,
    position: Vector2<f32>,
}

impl PlayerHandle {
    pub(crate) fn set_controlling(&mut self, is_controlling: bool) {
        log!("got elevated to controller");
        self.is_controlling = is_controlling
    }

    pub(crate) fn receive_position(&mut self, position: Vector2<f32>) {
        if !self.is_controlling {
            log!("received position from foreign client");
            self.position = position
        }
    }
}

impl ClientEntity for PlayerHandle {}

impl ClientHandle for PlayerHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, store: &mut DataStore) {
        messenger.register_receiver(Self::set_controlling);
        messenger.register_receiver(Self::receive_position);
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut RefMut<Renderer>, delta_time: f64) {
        if self.is_controlling {
            self.position.x += 1.0 / delta_time as f32;
            if (self.position - self.p_position).mag_sq() > 0.01 {
                messenger.call_server_fn(Player::client_position_update, self.position, SendMode::Quick);
                log!("told server i moved");
                self.p_position = self.position;
            }
        }
    }
}