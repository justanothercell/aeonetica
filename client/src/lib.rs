use std::ops::{Deref, DerefMut};
use aeonetica_engine::libloading::Library;

pub trait ClientMod {
    #[allow(unused_variables)]
    fn init(&mut self, flags: &Vec<String>){}
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