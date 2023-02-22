use aeonetica_engine::{Id, log};
use aeonetica_server::ecs::entity::Entity;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ecs::Engine;
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
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        log!("mymodule started. entity id: {id:?}");
        log!("accessing data in start: {}", engine.get_module_of::<Self>(id).unwrap().data);
        let s = engine.mut_entity(id).unwrap();
        {
            let x= s.mut_module::<Self>().unwrap();
            //let y= s.mut_module::<Self>().unwrap();
            x.data = 1;
            //y.data = 1;
        }
        {
            let z = s.mut_module::<Self>().unwrap();
            z.data = -1;
        }
        log!("accessing data in start second time: {}", engine.get_module_of::<Self>(id).unwrap().data);
    }
    fn tick(_id: &Id, _engine: &mut Engine) where Self: Sized {

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
            data: 0
        });
        engine.add_entity(entity);
    }
}