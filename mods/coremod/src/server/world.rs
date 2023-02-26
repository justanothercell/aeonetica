use aeonetica_engine::{Id, log};
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::module::Module;

pub(crate) struct World {

}

impl Module for World {

}

pub(crate) struct WorldGen {

}

impl Module for WorldGen {
    fn tick(id: &Id, engine: &mut Engine) where Self: Sized {
        log!("generating world...");
        let world = WorldGen::generate(id, engine);
        let entity = engine.mut_entity(id).unwrap();
        entity.add_module(world);
        entity.remove_module::<Self>();
        log!("generated world");
    }
}

impl Default for WorldGen {
    fn default() -> Self {
        WorldGen::new()
    }
}

impl WorldGen {
    pub(crate) fn new() -> WorldGen {
        WorldGen {}
    }

    pub(crate) fn generate(id: &Id, engine: &mut Engine) -> World{
        let gen = engine.get_module_of::<Self>(id).unwrap();
        World {}
    }
}