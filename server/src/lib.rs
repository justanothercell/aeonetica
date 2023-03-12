#![feature(unboxed_closures)]

use std::ops::{Deref, DerefMut};
use aeonetica_engine::libloading::Library;
use crate::ecs::Engine;

pub mod ecs;
mod networking;
mod server_runtime;
pub mod server;

pub trait ServerMod {
    #[allow(unused_variables)]
    fn init(&mut self, flags: &Vec<String>){}
    #[allow(unused_variables)]
    fn start(&mut self, engine: &mut Engine) {}
}

pub struct ServerModBox {
    server_mod: Box<dyn ServerMod>,
    _library: Library
}

impl ServerModBox {
    pub fn new(server_mod: Box<dyn ServerMod>, library: Library) -> Self{
        Self {
            server_mod,
            _library: library,
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