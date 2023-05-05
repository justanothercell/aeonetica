use std::{collections::HashMap, rc::Rc, cell::RefCell};

use aeonetica_client::{ClientMod, networking::messaging::{ClientHandle, ClientMessenger}, data_store::DataStore, renderer::{window::{OpenGlContextProvider, events::Event}, layer::Layer, context::Context, Renderer}, client_runtime::ClientHandleBox};
use aeonetica_engine::{log, Id, util::{id_map::IdMap, type_to_id}, networking::messaging::ClientEntity, log_warn};

use crate::common::Chunk;

pub struct WorldModClient {

}

impl ClientMod for WorldModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!");
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
        handlers.insert(type_to_id::<WorldHandle>(), || Box::new(WorldHandle::new()));
    }

    fn start(&self, context: &mut Context, _store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
        gl_context_provider.make_context();

        context.push(Rc::new(WorldLayer::instantiate()))
    }
}

pub(crate) struct WorldHandle {

}

impl WorldHandle {
    fn new() -> Self {
        Self {}
    }

    fn receive_chunk_data(&mut self, data: Chunk) {
        log_warn!("receive_chunk_data() called")
    }
}

impl ClientEntity for WorldHandle {

}

impl ClientHandle for WorldHandle {
    fn start(&mut self, messenger: &mut ClientMessenger, store: &mut DataStore) {
        messenger.register_receiver(Self::receive_chunk_data);
    }

    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &std::cell::RefMut<Renderer>, delta_time: f64) {
        
    }
}

pub struct WorldLayer {
    renderer: RefCell<Renderer>
}

impl Layer for WorldLayer {
    fn instantiate() -> Self where Self: Sized {
        Self {
            renderer: RefCell::new(Renderer::new())
        }
    }

    fn on_attach(&self) {
        log!("WorldLayer attached");
    }

    fn on_detach(&self) {
        log!("WorldLayer detached");
    }

    fn on_update(&self, handles: &mut IdMap<ClientHandleBox>, delta_time: f64) {
        let mut renderer = self.renderer.borrow_mut();

        handles.iter_mut().for_each(|(_, h)| h.update(&renderer, delta_time));

        renderer.draw_vertices();
    }

    fn on_event(&self, _handles: &mut IdMap<ClientHandleBox>, _event: &Event) -> bool {
        false
    }
}