use aeonetica_engine::{Id, log};
use aeonetica_server::ecs::entity::Entity;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::ServerMessenger;
use aeonetica_server::ServerMod;
use crate::common::Broadcastings;

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
        let messenger = ServerMessenger::new(engine);
        engine.mut_entity(id).unwrap().add_module(messenger);
        log!("registering client loginout listener");
        engine.mut_entity(id).unwrap().add_module(ConnectionListener {
            on_join: |id, user, engine| {
                log!("user joined: {user}");
                let messenger: &mut ServerMessenger = engine.mut_module_of(id).unwrap();
                messenger.add_receiver(*user);
                engine.mut_module_of::<Self>(id).unwrap().broadcastings.push(format!("user joined: {user}"));
            },
            on_leave: |id, user, engine| {
                log!("user left: {user}");
                let messenger: &mut ServerMessenger = engine.mut_module_of(id).unwrap();
                messenger.remove_receiver(user);
                engine.mut_module_of::<Self>(id).unwrap().broadcastings.push(format!("user left: {user}"));
            },
        });
    }
    fn tick(id: &Id, engine: &mut Engine) where Self: Sized {
        if engine.get_module_of::<ServerMessenger>(id).unwrap().is_sending_tick() {
            let mut messages = vec![];
            std::mem::swap(&mut messages, &mut engine.mut_module_of::<Self>(id).unwrap().broadcastings);
            let messenger = engine.mut_module_of::<ServerMessenger>(id).unwrap();
            messenger.set_sending_data(&Broadcastings(messages));
        }
    }
}

impl ServerMod for CoreModServer {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from server core init!");
    }
    fn start(&mut self, engine: &mut Engine) {
        log!("server core start");
        let mut entity = Entity::new();
        entity.add_module(MyModule {
            broadcastings: vec![],
        });
        engine.add_entity(entity);
    }
}