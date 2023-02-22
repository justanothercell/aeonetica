use std::any::TypeId;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use aeonetica_engine::libloading::Library;
use crate::messaging::ClientHandle;

pub mod events;
pub mod layers;
pub mod messaging;

pub trait ClientMod {
    #[allow(unused_variables)]
    fn init(&mut self, flags: &Vec<String>){}
    #[allow(unused_variables)]
    fn register_handlers(&self, handlers: &mut HashMap<TypeId, fn() -> Box<dyn ClientHandle>>) {}
}

pub struct ClientModBox {
    client_mod: Box<dyn ClientMod>,
    _library: Library
}

impl ClientModBox {
    pub fn new(client_mod: Box<dyn ClientMod>, library: Library) -> Self{
        Self {
            client_mod,
            _library: library,
        }
    }
}

impl Deref for ClientModBox {
    type Target = Box<dyn ClientMod>;
    fn deref(&self) -> &Self::Target { &self.client_mod }
}

impl DerefMut for ClientModBox {
    fn deref_mut(&mut self) -> &mut Box<dyn ClientMod> { &mut self.client_mod }
}