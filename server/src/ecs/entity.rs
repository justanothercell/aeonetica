use std::any::TypeId;
use std::collections::{hash_map, HashMap};
use aeonetica_engine::Id;
use crate::ecs::module::{Module, ModuleDyn};

pub struct Entity {
    pub(crate) entity_id: Id,
    pub(crate) modules: HashMap<TypeId, Box<dyn ModuleDyn>>
}

impl Default for Entity {
    fn default() -> Self {
        Entity::new()
    }
}

impl Entity {
    pub fn new() -> Self {
        Self {
            entity_id: Id::new(),
            modules: Default::default()
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

    pub fn modules(&self) -> Vec<TypeId> {
        self.modules.keys().copied().collect()
    }

    pub fn remove_module<T: Module + Sized + 'static>(&mut self) {
        self.modules.remove(&TypeId::of::<T>());
    }

    pub fn get_module<T: Module + Sized + 'static>(&self) -> Option<&T> {
        self.modules.get(&TypeId::of::<T>()).map(|m| unsafe { &*std::mem::transmute::<&Box<_>, &(*const T, usize)>(m).0 } )
    }

    pub fn mut_module<T: Module + Sized + 'static>(&mut self) -> Option<&mut T> {
        self.modules.get_mut(&TypeId::of::<T>()).map(|m| unsafe { &mut*std::mem::transmute::<&Box<_>, &(*mut T, usize)>(m).0 })
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