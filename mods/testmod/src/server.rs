use aeonetica_engine::{Id, log};
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ServerMod;
use crate::client::MyClientHandle;
use crate::common_client::{Broadcastings, MyClientHandle};

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
        let mut messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
        messenger.register_server_receiver(MyModule::receive_client_msg);

        log!("registering client loginout listener");
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
    }
    fn tick(id: &Id, engine: &mut Engine) where Self: Sized {

    }
}

impl MyModule {
    fn receive_client_msg(id: &Id, engine: &mut Engine, data: &Vec<u8>){

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