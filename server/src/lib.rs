use std::ops::{Deref, DerefMut};
use aeonetica_engine::libloading::Library;

pub trait ServerMod {
    #[allow(unused_variables)]
    fn init(&mut self, flags: &Vec<String>){

    }
}

pub struct ServerModBox {
    server_mod: Box<dyn ServerMod>,
    library: Library
}

impl ServerModBox {
    pub fn new(server_mod: Box<dyn ServerMod>, library: Library) -> Self{
        Self {
            server_mod,
            library,
        }
    }
}

impl Deref for ServerModBox {
    type Target = Box<dyn ServerMod>;
    fn deref(&self) -> &Self::Target { &self.server_mod }
}

impl DerefMut for ServerModBox {
    fn deref_mut(&mut self) -> &mut Box<dyn ServerMod> { &mut self.server_mod }
}