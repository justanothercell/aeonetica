use aeonetica_engine::Id;
use crate::ecs::Engine;
use crate::ecs::module::Module;

pub struct ConnectionListener {
    pub on_join: fn(id: &Id, user: &Id, engine: &mut Engine),
    pub on_leave: fn(id: &Id, user: &Id, engine: &mut Engine),
}

impl Module for ConnectionListener {

}