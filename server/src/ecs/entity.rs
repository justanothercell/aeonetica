use std::any::TypeId;
use std::collections::{hash_map, HashMap};
use std::collections::hash_map::Iter;
use std::iter::Map;
use std::marker::PhantomData;
use aeonetica_engine::Id;
use crate::ecs::module::{Module, ModuleDyn};

pub struct Entity<'a> {
    pub(crate) entity_id: Id,
    pub(crate) modules: HashMap<TypeId, Box<dyn ModuleDyn>>,
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
            entity_id: Id::new(),
            modules: Default::default(),
            phantom_data: Default::default(),
        }
    }

    pub fn add_module<T: Module + Sized + 'static>(&mut self, mut module: T) -> bool {
        module.init();
        if let hash_map::Entry::Vacant(e) = self.modules.entry(TypeId::of::<T>()) {
            e.insert(Box::new(module));
            true
        } else {
            false
        }
    }

    pub fn modules(&'a self) -> Vec<TypeId> {
        self.modules.keys().copied().collect()
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

    pub fn has_modul_type(&self, ty: &TypeId) -> bool {
        self.modules.contains_key(&ty)
    }

    pub fn id(&self) -> Id {
        self.entity_id
    }
}