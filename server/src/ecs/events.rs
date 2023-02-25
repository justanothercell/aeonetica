use aeonetica_engine::Id;
use crate::ecs::Engine;
use crate::ecs::module::Module;

pub struct ConnectionListener {
    pub(crate) on_join: fn(id: &Id, engine: &mut Engine, user: &Id),
    pub(crate) on_leave: fn(id: &Id, engine: &mut Engine, user: &Id),
}

impl ConnectionListener {
    pub fn new(on_join: fn(id: &Id, engine: &mut Engine, user: &Id), on_leave: fn(id: &Id, engine: &mut Engine, user: &Id)) -> Self {
        Self {
            on_join,
            on_leave,
        }
    }
}

impl Module for ConnectionListener {}