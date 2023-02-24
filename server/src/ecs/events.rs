use aeonetica_engine::Id;
use crate::ecs::Engine;
use crate::ecs::module::Module;

pub struct ConnectionListener {
    pub(crate) on_join: fn(id: &Id, user: &Id, engine: &mut Engine),
    pub(crate) on_leave: fn(id: &Id, user: &Id, engine: &mut Engine),
}

impl ConnectionListener {
    pub fn new(on_join: fn(id: &Id, user: &Id, engine: &mut Engine), on_leave: fn(id: &Id, user: &Id, engine: &mut Engine)) -> Self {
        Self {
            on_join,
            on_leave,
        }
    }
}

impl Module for ConnectionListener {}