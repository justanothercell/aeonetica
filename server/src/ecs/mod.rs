use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut, Keys};
use std::iter::{FilterMap};
use crate::ecs::entity::Entity;
use aeonetica_engine::Id;
use crate::ecs::module::Module;

pub mod module;
pub mod entity;

pub struct World {
    entites: HashMap<Id, Entity>
}

impl World {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entites: Default::default(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let id = entity.entity_id;
        self.entites.insert(entity.id(), entity);
        let mut_self_ref= unsafe { &mut *(self as *mut World) };
        let modules = &self.get_entity(&id).unwrap().modules;
        let keys = modules.keys().clone();
        for ty in keys{
            if let Some(m) = self.get_entity(&id).unwrap().modules.get(ty) {
                m.start_dyn(&id, mut_self_ref)
            }
        };
    }

    pub fn remove_entity(&mut self, id: &Id) -> bool {
        self.entites.remove(id).is_some()
    }

    pub fn get_entity(&self, id: &Id) -> Option<&Entity> {
        self.entites.get(id)
    }

    pub fn mut_entity(&mut self, id: &Id) -> Option<&mut Entity> {
        self.entites.get_mut(id)
    }

    pub fn get_module_of<T: Module + Sized + 'static>(&self, id: &Id) -> Option<&T> {
        self.entites.get(id)?.get_module()
    }

    pub fn mut_module_of<T: Module + Sized + 'static>(&mut self, id: &Id) -> Option<&mut T> {
        self.entites.get_mut(id)?.mut_module()
    }

    pub fn ids(&self) -> Keys<Id, Entity>{
        self.entites.keys()
    }

    pub fn iter(&self) -> Iter<Id, Entity>{
        self.entites.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Id, Entity>{
        self.entites.iter_mut()
    }

    #[allow(clippy::type_complexity)]
    pub fn id_find_with<'a, T: Module + Sized + 'static>(&'a self) -> FilterMap<Iter<Id, Entity>, fn((&'a Id, &Entity)) -> Option<&'a Id>>{
        self.entites.iter().filter_map(|(id, e)| if e.has_module::<T>() { Some(id)} else { None })
    }

    #[allow(clippy::type_complexity)]
    pub fn find_with<'a, T: Module + Sized + 'static>(&'a self) -> FilterMap<Iter<Id, Entity>, fn((&'a Id, &'a Entity)) -> Option<(&'a Id, &'a dyn Module)>>{
        self.entites.iter().filter_map(|(id, e)| if e.has_module::<T>() { Some((id, e.get_module::<T>()?))} else { None })
    }


}
