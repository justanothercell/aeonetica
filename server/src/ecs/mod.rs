use std::cell::{RefCell};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::{Iter, IterMut, Keys};
use std::iter::{FilterMap};
use std::rc::Rc;
use std::task::ready;
use crate::ecs::entity::Entity;
use aeonetica_engine::Id;
use crate::ecs::messaging::MessagingSystem;
use crate::ecs::module::{Module, ModuleDyn};
use crate::server_runtime::ServerRuntime;

pub mod module;
pub mod entity;
pub mod events;
pub mod messaging;

pub struct Engine {
    entites: HashMap<Id, Entity>,
    tagged: HashMap<String, Id>,
    ms: Rc<RefCell<MessagingSystem>>,
    pub(crate) clients: HashSet<Id>,
    pub(crate) runtime: ServerRuntime
}

impl Engine {
    pub fn new(runtime: ServerRuntime) -> Self {
        Self {
            entites: Default::default(),
            tagged: Default::default(),
            ms: Rc::new(RefCell::new(MessagingSystem::new())),
            clients: Default::default(),
            runtime
        }
    }

    pub(crate) fn for_each_module<F: Fn(&mut Self, &Id, &mut Box<dyn ModuleDyn>)>(&mut self, runner: F) {
        for id in self.entites.keys().collect::<Vec<_>>() {
            self.entites.get(id).map(|e| {
                for mid in e.modules.keys().collect::<Vec<_>>() {
                    e.modules.get_mut(mid).map(|m| {
                        runner(self, id,  m)
                    });
                }
            });
        }
    }

    pub(crate) fn for_each_module_of_type<T: Module, F: Fn(&mut Self, &Id, &mut T)>(&mut self, runner: F) {
        for id in self.entites.keys().collect::<Vec<_>>() {
            self.entites.get(id).map(|e| {
                e.mut_module::<T>().map(|m| {
                    runner(self, id, m)
                });
            });
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let id = entity.entity_id;
        self.entites.insert(entity.id(), entity);
        let mut_self_ref= unsafe { &mut *(self as *mut Engine) };
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

    /// Returns `true` if tagging is successful.
    /// Tagging fails if `tag_exists(tag)` returns true or `entity_exists(id)` returns false.
    pub fn tag_entity(&mut self, tag: String, id: Id) -> bool {
        if !self.tag_exists(&tag) && self.entity_exists(&id) {
            self.tagged.insert(tag, id);
            true
        } else {
            false
        }
    }

    /// Returns `true` if tag exists and `entity_exists(id)` returns true.
    /// Tags whose entity got removed are treated as nonexistent and can be overridden
    pub fn tag_exists(&self, tag: &str) -> bool {
       self.tagged.get(tag).map(|id| self.entity_exists(id)).unwrap_or(false)
    }

    /// Returns `true` if tag existed.
    /// Removal fails if `tag_exists(tag)` returns false.
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if self.tag_exists(&tag) {
            self.tagged.remove(tag);
            true
        } else {
            false
        }
    }

    pub fn get_entity_by_tag(&self, tag: &str) -> Option<&Entity> {
        self.entites.get(self.tagged.get(tag)?)
    }

    pub fn mut_entity_by_tag(&mut self, tag: &str) -> Option<&mut Entity> {
        self.entites.get_mut(self.tagged.get(tag)?)
    }

    pub fn entity_exists(&self, id: &Id) -> bool {
        self.entites.contains_key(id)
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

    pub fn get_module_by_tag<T: Module + Sized + 'static>(&self, tag: &str) -> Option<&T> {
        self.get_entity_by_tag(tag)?.get_module()
    }

    pub fn mut_module_by_tag<T: Module + Sized + 'static>(&mut self, tag: &str) -> Option<&mut T> {
        self.mut_entity_by_tag(tag)?.mut_module()
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
