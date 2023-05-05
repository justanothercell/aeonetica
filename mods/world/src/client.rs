use std::collections::HashMap;

use aeonetica_client::{ClientMod, networking::messaging::ClientHandle, data_store::DataStore, renderer::window::OpenGlContextProvider};
use aeonetica_engine::{log, util::type_to_id, Id};

pub struct WorldModClient {

}

impl ClientMod for WorldModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!");
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
    }

    fn start(&self, context: &mut aeonetica_client::renderer::context::Context, _store: &mut DataStore, gl_context_provider: &OpenGlContextProvider) {
    }
}
