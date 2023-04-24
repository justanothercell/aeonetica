
use aeonetica_engine::{EntityId, Id, TypeId};
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::type_to_id;
use crate::ecs::Engine;
use crate::ecs::module::{Module, ModuleDyn};

pub struct Entity {
    engine: *mut Engine, // Only use for add/removal events!!! DO NOT use for any other purpose!!!
    pub(crate) entity_id: EntityId,
    pub(crate) modules: IdMap<Box<dyn ModuleDyn>>
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
        if let std::collections::hash_map::Entry::Vacant(e) = self.modules.entry(type_to_id::<T>()) {
            e.insert(Box::new(module));
            if let Some(m) = self.modules.get(&type_to_id::<T>()) {
                m.start_dyn(&self.entity_id, unsafe {&mut *self.engine})
            }
            true
        } else {
            false
        }
    }

    pub fn modules(&self) -> Vec<TypeId> {
        self.modules.keys().copied().collect()
    }

    pub fn remove_module<T: Module + Sized + 'static>(&mut self) -> bool{
        if let Some(m) = self.modules.get(&type_to_id::<T>()) { m.remove_dyn(&self.entity_id, unsafe {&mut *self.engine}) }
        self.modules.remove(&type_to_id::<T>()).is_some()
    }

    pub fn get_module<T: Module + Sized + 'static>(&self) -> Option<&T> {
        self.modules.get(&type_to_id::<T>()).map(|m| unsafe { &*std::mem::transmute::<&Box<_>, &(*const T, usize)>(m).0 } )
    }

    pub fn mut_module<T: Module + Sized + 'static>(&mut self) -> Option<&mut T> {
        self.modules.get_mut(&type_to_id::<T>()).map(|m| unsafe { &mut*std::mem::transmute::<&Box<_>, &(*mut T, usize)>(m).0 })
    }

    pub fn get_or_create<T: Module + Sized + 'static, F: FnOnce() -> T>(&mut self, creator: F) -> &T {
        self.modules.get(&type_to_id::<T>()).map(|m| unsafe { &*std::mem::transmute::<&Box<_>, &(*const T, usize)>(m).0 } ).unwrap_or_else(||{
            self.add_module(creator());
            self.mut_module().unwrap()
        })
    }

    pub fn mut_or_create<T: Module + Sized + 'static, F: FnOnce() -> T>(&mut self, creator: F) -> &T {
        self.modules.get_mut(&type_to_id::<T>()).map(|m| unsafe { &*std::mem::transmute::<&Box<_>, &(*mut T, usize)>(m).0 } ).unwrap_or_else(||{
            self.add_module(creator());
            self.mut_module().unwrap()
        })
    }


    pub fn has_module<T: Module + Sized + 'static>(&self) -> bool {
        self.modules.contains_key(&type_to_id::<T>())
    }

    pub fn has_module_type(&self, ty: &TypeId) -> bool {
        self.modules.contains_key(ty)
    }

    pub fn id(&self) -> EntityId {
        self.entity_id
    }
}