use std::collections::HashMap;
use aeonetica_client::ClientMod;
use aeonetica_engine::{Id, log};
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_client::networking::messaging::{ClientHandle, ClientMessenger};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::type_to_id;
use crate::server::MyModule;

pub struct TestModClient {

}

impl ClientMod for TestModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client testmod!")
    }

    fn register_handlers(&self, handlers: &mut HashMap<Id, fn() -> Box<dyn ClientHandle>>) {
        log!("handles registered");
        handlers.insert(type_to_id::<MyClientHandle>(), || Box::new(MyClientHandle { }));
    }
}

pub(crate) struct MyClientHandle {

}

impl ClientEntity for MyClientHandle {}

impl ClientHandle for MyClientHandle {
    fn init(&mut self) {
        log!("my client handle initialized")
    }

    fn start(&mut self, messenger: &mut ClientMessenger) {
        messenger.register_receiver(MyClientHandle::receive_server_msg);
        messenger.call_server_fn(MyModule::receive_client_msg, "Hello from client server call function".to_string(), SendMode::Safe);
        log!("receive_server_msg registered in start");
    }

    fn remove(&mut self, _messenger: &mut ClientMessenger) {
        log!("my client handle removed")
    }


}

impl MyClientHandle {
    pub(crate) fn receive_server_msg(&mut self, msg: String){
        log!("received server msg: {msg}")
    }
}