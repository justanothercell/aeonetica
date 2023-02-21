use aeonetica_engine::Id;
use crate::ecs::World;

pub trait Module {
    fn init(&mut self) where Self: Sized {}
    #[allow(unused_variables)]
    fn start(id: &Id, world: &mut World) where Self: Sized {}
    #[allow(unused_variables)]
    fn tick(id: &Id, world: &mut World) where Self: Sized {}
}

pub(crate) trait ModuleDyn {
    fn start_dyn(&self, id: &Id, world: &mut World);
    fn tick_dyn(&self, id: &Id, world: &mut World);
}

impl<T: Module + Sized> ModuleDyn for T {
    fn start_dyn(&self, id: &Id, world: &mut World) {
        T::start(id, world)
    }
    fn tick_dyn(&self, id: &Id, world: &mut World) {
        T::tick(id, world)
    }
}
