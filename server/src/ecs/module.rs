use aeonetica_engine::Id;
use crate::ecs::World;

pub trait Module {
    fn init(&mut self) where Self: Sized {}
    #[allow(unused_variables)]
    fn start<'a>(id: &Id, world: &'a mut World<'a>) where Self: Sized {}
    #[allow(unused_variables)]
    fn tick<'a>(id: &Id, world: &'a mut World<'a>) where Self: Sized {}
}

pub(crate) trait ModuleDyn {
    fn start_dyn<'a>(&self, id: &Id, world: &'a mut World<'a>);
    fn tick_dyn<'a>(&self, id: &Id, world: &'a mut World<'a>);
}

impl<T: Module + Sized> ModuleDyn for T {
    fn start_dyn<'a>(&self, id: &Id, world: &'a mut World<'a>) {
        T::start(id, world)
    }
    fn tick_dyn<'a>(&self, id: &Id, world: &'a mut World<'a>) {
        T::tick(id, world)
    }
}
