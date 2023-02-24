use std::any::TypeId;
use std::collections::{hash_map, HashMap};
use aeonetica_engine::Id;
use crate::ecs::Engine;
use crate::ecs::module::{Module, ModuleDyn};

pub struct Entity {
    engine: *mut Engine, // Only use for add/removal events!!! DO NOT use for any other purpose!!!
    pub(crate) entity_id: Id,
    pub(crate) modules: HashMap<TypeId, Box<dyn ModuleDyn>>
}

impl Entity {
    pub(crate) fn new(engine: &Engine) -> Self {
        Self {
            engine: engine as *const Engine as *mut Engine,
            entity_id: Id::new(),
            modules: Default::default()
        }
    }

    pub fn add_module<T: Module + Sized + 'static>(&mut self, mut module: T) -> bool {
        module.init();
        if let hash_map::Entry::Vacant(e) = self.modules.entry(TypeId::of::<T>()) {
            e.insert(Box::new(module));
            self.modules.get(&TypeId::of::<T>()).map(|m| m.start_dyn(&self.entity_id, unsafe {&mut *self.engine}));
            true
        } else {
            false
        }
    }

    pub fn modules(&self) -> Vec<TypeId> {
        self.modules.keys().copied().collect()
    }

    pub fn remove_module<T: Module + Sized + 'static>(&mut self) -> bool{
        self.modules.get(&TypeId::of::<T>()).map(|m| m.remove_dyn(&self.entity_id, unsafe {&mut *self.engine}));
        self.modules.remove(&TypeId::of::<T>()).is_some()
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