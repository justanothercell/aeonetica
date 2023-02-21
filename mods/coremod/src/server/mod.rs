use aeonetica_engine::{Id, log};
use aeonetica_server::ecs::entity::Entity;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ecs::World;
use aeonetica_server::ServerMod;

pub struct CoreModServer {

}

pub struct MyModule {
    data: i32
}

impl Module for MyModule {
    fn init(&mut self) where Self: Sized {
        self.data = 43;
        log!("mymodule initialized");
    }
    fn start<'a>(id: &Id, world: &'a mut World<'a>) where Self: Sized {
        log!("mymodule started. entity id: {id:?}");
        log!("accessing data in start: {}", world.get_module_of::<Self>(id).unwrap().data)
    }
    fn tick<'a>(_id: &Id, _world: &'a mut World<'a>) where Self: Sized {

    }
}

impl ServerMod for CoreModServer {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from server core init!");
    }
    fn start<'a>(&mut self, world: &'a mut World<'a>) {
        log!("server core start");
        let mut entity = Entity::new();
        entity.add_module(MyModule {
            data: 0
        });
        world.add_entity(entity);
    }
}