use std::collections::HashMap;
use aeonetica_engine::Id;
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_engine::util::type_to_id;
use aeonetica_client::ClientMod;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};

pub(crate) struct PlayerModClient {

}

impl ClientMod for PlayerModClient {
    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        handlers.insert(type_to_id::<PlayerHandle>(), || Box::new(PlayerHandle { is_owner: false, pos: (0.0, 0.0), p_pos: (0.0, 0.0) }));
    }
}


pub(crate) struct PlayerHandle {
    /// allows the client to control this player instance
    is_owner: bool,
    pos: (f32, f32),
    p_pos: (f32, f32),
}

impl PlayerHandle {
    pub(crate) fn set_owning(&mut self, owning: bool) {
        self.is_owner = owning;
    }

    pub(crate) fn server_position_update(&mut self, pos: (f32, f32)) {
        if !self.is_owner {
            self.p_pos = self.pos;
            self.pos = pos;
        }
    }
}

impl ClientEntity for PlayerHandle {}

// TODO: render/update loop
// TODO: input if self.is_owner
// TODO: use self.p_pos and self.pos to interpolate position
// TODO: Timing functions to get speed, probably also as arg in the receiver functions above
impl ClientHandle for PlayerHandle {
    fn start(&mut self, messenger: &mut ClientMessenger) {
        messenger.register_receiver(PlayerHandle::set_owning);
    }
}