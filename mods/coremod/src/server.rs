use std::any::{type_name, TypeId};
use aeonetica_engine::{Id, log};
use aeonetica_engine::nanoserde::SerBin;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ServerMod;
use crate::common_client::{Broadcastings, MyClientHandle};

pub struct CoreModServer {

}

pub struct MyModule {
    broadcastings: Vec<String>
}

impl Module for MyModule {
    fn init(&mut self) where Self: Sized {
        log!("mymodule initialized");
    }
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        log!("mymodule started. entity id: {id:?}");
        log!("registering messenger");
        engine.mut_entity(id).unwrap().add_module(Messenger::new::<MyClientHandle>(|id, engine, sending| {
            let mut messages = vec![];
            std::mem::swap(&mut messages, &mut engine.mut_module_of::<Self>(id).unwrap().broadcastings);
            messages.push("foo!".to_string());
            *sending = Broadcastings(messages).serialize_bin();
        },
        |_id, _client, _engine| {
            // currently not receiving any data from client
        }));
        log!("registering client loginout listener");
        engine.mut_entity(id).unwrap().add_module(ConnectionListener::new(
            |id, user, engine| {
                log!("user joined: {user}");
                let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
                messenger.add_client(*user);
                engine.mut_module_of::<Self>(id).unwrap().broadcastings.push(format!("user joined: {user}"));
            },
            |id, user, engine| {
                log!("user left: {user}");
                let messenger: &mut Messenger = engine.mut_module_of(id).unwrap();
                messenger.remove_client(user);
                engine.mut_module_of::<Self>(id).unwrap().broadcastings.push(format!("user left: {user}"));
            }));
    }
    fn tick(id: &Id, engine: &mut Engine) where Self: Sized {

    }
}

impl ServerMod for CoreModServer {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from server core init!");
    }
    fn start(&mut self, engine: &mut Engine) {
        log!("server core start");
        let id = engine.new_entity();
        engine.mut_entity(&id).unwrap().add_module(MyModule {
            broadcastings: vec![],
        });
    }
}