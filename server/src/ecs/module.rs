use aeonetica_engine::Id;
use crate::ecs::World;

pub trait Module {
    fn init(id: &Id, world: &mut World) where Self: Sized {}
    fn start(id: &Id, world: &mut World) where Self: Sized {}
    fn tick(id: &Id, world: &mut World) where Self: Sized {}
}

