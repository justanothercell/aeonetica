use std::collections::HashMap;
use aeonetica_client::ClientMod;
use aeonetica_engine::{Id, log};
use aeonetica_engine::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_engine::util::type_to_id;
use crate::common_client::MyClientHandle;

pub struct CoreModClient {

}

impl ClientMod for CoreModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!")
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
        handlers.insert(type_to_id::<MyClientHandle>(), || Box::new(MyClientHandle { has_greeted: false }));
    }
}

pub(crate) struct MyClientHandle {
    pub(crate) has_greeted: bool
}

impl ClientHandle for MyClientHandle {
    fn init(&mut self) {
        log!("my client handle initialized")
    }

    fn start(&mut self, messenger: &mut ClientMessenger) {
        messenger.register_server_receiver()
    }

    fn remove(&mut self, _messenger: &mut ClientMessenger) {
        log!("my client handle removed")
    }


}

impl MyClientHandle {
    pub(crate) fn receive_server_msg(&mut self, data: &Vec<u8>){

    }
}