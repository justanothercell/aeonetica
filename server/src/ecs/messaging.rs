use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::fmt::Debug;
use std::net::UdpSocket;
use std::rc::Rc;
use aeonetica_engine::Id;
use aeonetica_engine::nanoserde::{DeBin, SerBin};
use aeonetica_engine::networking::messaging::ClientHandle;
use crate::ecs::{Module, Engine};

pub(crate) struct MessagingSystem {
    pub(crate) messengers: HashSet<Id>
}

impl MessagingSystem {
    pub(crate) fn new() -> Self {
        Self {
            messengers: Default::default()
        }
    }
}

pub trait Message: SerBin + DeBin + Debug {}

pub struct Messenger {
    ms: Rc<RefCell<MessagingSystem>>,
    handle_type: TypeId,
    pub(crate) receivers: HashSet<Id>,
    pub(crate) on_send: fn(id: &Id, engine: &mut Engine, sending: &mut Vec<u8>),
    pub(crate) on_receive: fn(id: &Id, user: &Id, engine: &mut Engine),
}

impl Module for Messenger {
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        engine.ms.borrow_mut().messengers.insert(*id);
    }

    fn remove(id: &Id, engine: &mut Engine) where Self: Sized {
        engine.ms.borrow_mut().messengers.remove(id);
    }
}

impl Messenger {
    pub fn new<H: ClientHandle + Sized + 'static>(engine: &Engine, on_send: fn(id: &Id, engine: &mut Engine, sending: &mut Vec<u8>), on_receive: fn(id: &Id, user: &Id, engine: &mut Engine)) -> Self {
        Self {
            ms: engine.ms.clone(),
            receivers: Default::default(),
            handle_type: TypeId::of::<H>(),
            on_send,
            on_receive
        }
    }

    pub fn clients(&self) -> Iter<Id> {
        self.receivers.iter()
    }

    pub fn has_client(&self, id: &Id) -> bool {
        self.receivers.contains(id)
    }

    pub fn add_client(&mut self, id: Id) -> bool {
        if !self.receivers.contains(&id) {
            self.receivers.insert(id);
            true
        } else { false }
    }

    pub fn remove_client(&mut self, id: &Id) -> bool {
        if self.receivers.contains(id) {
            self.receivers.remove(id);
            true
        } else { false }
    }
}