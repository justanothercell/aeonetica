use std::collections::HashMap;
use aeonetica_engine::Id;
use aeonetica_server::ecs::module::Module;

pub(crate) struct Player {

}

impl Player {
    pub(crate) fn new(id: &Id) -> Self {
        Self {}
    }
}

impl Module for Player {

}

pub(crate) struct PlayerHandler {
    pub players: HashMap<Id, Id>
}

impl Module for PlayerHandler {

}