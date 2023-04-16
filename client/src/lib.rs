#![feature(let_chains)]
#![feature(result_flattening)]

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use aeonetica_engine::Id;
use aeonetica_engine::libloading::Library;
use renderer::context::Context;
use renderer::window::OpenGlContextProvider;
use crate::data_store::DataStore;
use crate::networking::messaging::ClientHandle;

pub mod networking;
pub mod client_runtime;
pub mod client;
pub mod renderer;
pub mod data_store;

pub trait ClientMod {
    #[allow(unused_variables)]
    fn init(&mut self, flags: &Vec<String>){}
    #[allow(unused_variables)]
    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {}
    #[allow(unused_variables)]
    fn start(&self, context: &mut Context, store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {}
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