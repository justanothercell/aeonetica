use std::any::TypeId;
use std::collections::HashMap;
use aeonetica_client::ClientMod;
use aeonetica_engine::log;
use aeonetica_engine::networking::messaging::ClientHandle;
use crate::common_client::MyClientHandle;

pub struct CoreModClient {

}

impl ClientMod for CoreModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client core!")
    }

    fn register_handlers(&self, handlers: &mut HashMap<TypeId, fn() -> Box<dyn ClientHandle>>) {
        handlers.insert(TypeId::of::<MyClientHandle>(), || Box::new(MyClientHandle {}));
    }
}