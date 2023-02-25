use std::collections::HashMap;
use aeonetica_client::ClientMod;
use aeonetica_engine::{Id, log};
use aeonetica_engine::networking::messaging::ClientHandle;
use aeonetica_engine::util::type_to_id;
use crate::common_client::MyClientHandle;

pub struct CoreModClient {

}

impl ClientMod for CoreModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client core!")
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
        handlers.insert(type_to_id::<MyClientHandle>(), || Box::new(MyClientHandle { has_greeted: false }));
    }
}