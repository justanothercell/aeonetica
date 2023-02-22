use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use aeonetica_engine::Id;
use aeonetica_engine::nanoserde::{DeBin, SerBin};
use crate::ecs::{Module, Engine};

pub(crate) struct MessagingSystem {
    client_interfaces: HashSet<Id>,
    is_sending_tick: bool,
    is_receiving_tick: bool
}

impl MessagingSystem {
    pub(crate) fn new() -> Self {
        Self {
            client_interfaces: Default::default(),
            is_sending_tick: false,
            is_receiving_tick: false,
        }
    }
}

pub trait Message: SerBin + DeBin + Debug {}

pub struct ClientInterface {
    ms: Rc<RefCell<MessagingSystem>>,
    sending: Option<Vec<u8>>,
    received: Option<Vec<u8>>
}

impl Module for ClientInterface {
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        engine.ms.borrow_mut().client_interfaces.insert(*id);
    }

    fn remove(id: &Id, engine: &mut Engine) where Self: Sized {
        engine.ms.borrow_mut().client_interfaces.remove(id);
    }
}

impl ClientInterface {
    pub fn new(engine: &Engine) -> Self {
        Self {
            ms: engine.ms.clone(),
            sending: None,
            received: None,
        }
    }

    pub fn is_sending_tick(&self) -> bool {
        self.ms.borrow().is_sending_tick
    }

    pub fn is_receiving_tick(&self) -> bool {
        self.ms.borrow().is_receiving_tick
    }

    pub fn is_sending_data_set(&self) -> bool {
        self.sending.is_some()
    }

    pub fn set_sending_data<T: Message>(&mut self, data: &T) {
        self.sending = Some(SerBin::serialize_bin(data))
    }



    pub fn get_received_data<T: Message>(&self) -> Option<T> {
        self.received.as_ref().map(|v| DeBin::deserialize_bin(v).ok())?
    }

    pub fn has_received_data(&self) -> bool {
        self.received.is_some()
    }
}