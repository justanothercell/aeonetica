#![feature(let_chains)]
#![feature(result_flattening)]
#![feature(local_key_cell_methods)]
#![feature(generic_const_exprs)]

use std::ops::{Deref, DerefMut};
use aeonetica_engine::libloading::Library;
use aeonetica_engine::util::id_map::IdMap;
use crate::data_store::DataStore;
use crate::networking::messaging::ClientHandle;
use crate::renderer::context::RenderContext;
use crate::renderer::window::OpenGlRenderContextProvider;

pub mod networking;
pub mod client_runtime;
pub mod client;
pub mod renderer;
pub mod data_store;

pub trait ClientMod {
    #[allow(unused_variables)]
    fn init(&mut self, flags: &Vec<String>){}
    #[allow(unused_variables)]
    fn register_handlers(&self, handlers: &mut IdMap<fn() -> Box<dyn ClientHandle>>, store: &mut DataStore) {}
    #[allow(unused_variables)]
    fn start<'a>(&self, store: &mut DataStore, provider: OpenGlRenderContextProvider<'a>) -> &'a mut RenderContext { provider.make_context() }
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