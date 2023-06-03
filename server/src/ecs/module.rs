use aeonetica_engine::{EntityId, time::Time};
use crate::ecs::Engine;

pub trait Module {
    fn init(&mut self) where Self: Sized {}
    #[allow(unused_variables)]
    fn start(id: &EntityId, engine: &mut Engine) where Self: Sized {}
    #[allow(unused_variables)]
    fn tick(id: &EntityId, engine: &mut Engine, time: Time) where Self: Sized {}
    #[allow(unused_variables)]
    fn remove(id: &EntityId, engine: &mut Engine) where Self: Sized {}
}

pub(crate) trait ModuleDyn: Module {
    fn start_dyn(&self, id: &EntityId, engine: &mut Engine);
    fn tick_dyn(&self, id: &EntityId, engine: &mut Engine, time: Time);
    fn remove_dyn(&self, id: &EntityId, engine: &mut Engine);
}

/// This trait is a helper trait to make the non-self methods of `Module` accessible via vtable
impl<T: Module + Sized> ModuleDyn for T {
    fn start_dyn(&self, id: &EntityId, engine: &mut Engine) {
        T::start(id, engine)
    }
    fn tick_dyn(&self, id: &EntityId, engine: &mut Engine, time: Time) {
        T::tick(id, engine, time)
    }
    fn remove_dyn(&self, id: &EntityId, engine: &mut Engine) {
        T::remove(id, engine)
    }
}