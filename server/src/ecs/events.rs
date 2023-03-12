use aeonetica_engine::{ClientId, EntityId, Id};
use crate::ecs::Engine;
use crate::ecs::module::Module;

pub struct ConnectionListener {
    pub(crate) on_join: fn(id: &EntityId, engine: &mut Engine, user: &ClientId),
    pub(crate) on_leave: fn(id: &EntityId, engine: &mut Engine, user: &ClientId),
}

impl ConnectionListener {
    pub fn new(on_join: fn(id: &EntityId, engine: &mut Engine, user: &ClientId), on_leave: fn(id: &EntityId, engine: &mut Engine, user: &ClientId)) -> Self {
        Self {
            on_join,
            on_leave,
        }
    }
}

impl Module for ConnectionListener {}