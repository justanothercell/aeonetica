use aeonetica_engine::Id;
use crate::ecs::Engine;

pub trait Module {
    fn init(&mut self) where Self: Sized {}
    #[allow(unused_variables)]
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {}
    #[allow(unused_variables)]
    fn tick(id: &Id, engine: &mut Engine) where Self: Sized {}
    #[allow(unused_variables)]
    fn remove(id: &Id, engine: &mut Engine) where Self: Sized {}
}

pub(crate) trait ModuleDyn {
    fn start_dyn(&self, id: &Id, engine: &mut Engine);
    fn tick_dyn(&self, id: &Id, engine: &mut Engine);
    fn remove_dyn(&self, id: &Id, engine: &mut Engine);
}

/// This trait is a helper trait to make the non-self methods of `Module` accessible via vtable
impl<T: Module + Sized> ModuleDyn for T {
    fn start_dyn(&self, id: &Id, engine: &mut Engine) {
        T::start(id, engine)
    }
    fn tick_dyn(&self, id: &Id, engine: &mut Engine) {
        T::tick(id, engine)
    }
    fn remove_dyn(&self, id: &Id, engine: &mut Engine) {
        T::tick(id, engine)
    }
}