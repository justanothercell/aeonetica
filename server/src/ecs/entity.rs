use std::any::TypeId;
use std::collections::{hash_map, HashMap};
use std::collections::hash_map::Keys;
use std::marker::PhantomData;
use aeonetica_engine::Id;
use aeonetica_engine::uuid::Uuid;
use crate::ecs::module::Module;

pub struct Entity<'a> {
    entity_id: Id,
    modules: HashMap<TypeId, Box<dyn Module>>,
    phantom_data: PhantomData<&'a ()>
}

impl<'a> Default for Entity<'a> {
    fn default() -> Self {
        Entity::new()
    }
}

impl<'a> Entity<'a> {
    pub fn new() -> Self {
        Self {
            entity_id: Uuid::new_v4().into_bytes(),
            modules: Default::default(),
            phantom_data: Default::default(),
        }
    }

    pub fn add_module<T: Module + Sized + 'static>(&mut self, module: T) -> bool {
        if let hash_map::Entry::Vacant(e) = self.modules.entry(TypeId::of::<T>()) {
            e.insert(Box::new(module));
            true
        } else {
            false
        }
    }

    pub fn remove_module<T: Module + Sized + 'static>(&mut self) {
        self.modules.remove(&TypeId::of::<T>());
    }

    pub fn get_module<T: Module + Sized + 'static>(&self) -> Option<&'a T> {
        self.modules.get(&TypeId::of::<T>()).map(|m| unsafe { &*std::mem::transmute::<&Box<_>, &(*const T, usize)>(m).0 } )
    }

    pub fn mut_module<T: Module + Sized + 'static>(&'a mut self) -> Option<&'a mut T> {
        self.modules.get_mut(&TypeId::of::<T>()).map(|m| unsafe { &mut*std::mem::transmute::<&'a Box<_>, &(*mut T, usize)>(m).0 })
    }

    pub fn has_module<T: Module + Sized + 'static>(&self) -> bool {
        self.modules.contains_key(&TypeId::of::<T>())
    }

    pub fn modules(&'a self) -> Keys<TypeId, Box<dyn Module>> {
        self.modules.keys()
    }

    pub fn id(&self) -> Id {
        self.entity_id
    }
}