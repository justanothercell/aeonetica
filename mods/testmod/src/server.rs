use std::any::Any;
use std::ops::{Generator};
use aeonetica_engine::{Id, log};
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ecs::scheduling::WaitFor;
use aeonetica_server::{ServerMod, yield_task};
use crate::client::MyClientHandle;

pub struct CoreModServer {

}

pub struct MyModule {

}

impl Module for MyModule {
    fn init(&mut self) where Self: Sized {
        log!("mymodule initialized");
    }
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        log!("mymodule started. entity id: {id:?}");
        log!("registering messenger");
        engine.mut_entity(id).unwrap().add_module(Messenger::new::<MyClientHandle>());
        let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
        messenger.register_receiver(MyModule::receive_client_msg);
        log!("registering receive_client_msg");
        engine.mut_entity(id).unwrap().add_module(ConnectionListener::new(
            |id, engine, user| {
                log!("user joined: {user}");
                let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
                messenger.add_client(*user);
                messenger.call_client_fn(MyClientHandle::receive_server_msg, format!("user joined: {user}"));
            },
            |id, engine, user| {
                log!("user left: {user}");
                let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
                messenger.remove_client(user);
                messenger.call_client_fn(MyClientHandle::receive_server_msg, format!("user left: {user}"));
            }));
        log!("registered client loginout listener");
        engine.queue_task( |mut e: &mut Engine| {
            for i in 1..11 {
                yield_task!(e, WaitFor::Ticks(20));
                log!("waited {i} seconds...")
            }
        });
        log!("queued task");
    }
    fn tick(_id: &Id, _engine: &mut Engine) where Self: Sized {

    }
}

impl MyModule {
    pub(crate) fn receive_client_msg(_id: &Id, _engine: &mut Engine, msg: String){
        log!("received client msg: {msg}")
    }
}

impl ServerMod for CoreModServer {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from server testmod init!");
    }
    fn start(&mut self, engine: &mut Engine) {
        log!("server core start");
        let id = engine.new_entity();
        engine.mut_entity(&id).unwrap().add_module(MyModule {});
    }
}