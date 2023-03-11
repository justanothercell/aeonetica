
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::fmt::Debug;
use std::rc::Rc;
use aeonetica_engine::{Id, log};
use aeonetica_engine::nanoserde::{DeBin, SerBin};
use aeonetica_engine::networking::messaging::ClientHandle;
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use aeonetica_engine::util::type_to_id;
use crate::ecs::{Module, Engine};
use crate::networking::NetworkServer;

pub trait Message: SerBin + DeBin + Debug {}

pub struct Messenger {
    ns: Option<Rc<RefCell<NetworkServer>>>,
    handle_type: Id,
    entity_id: Id,
    pub(crate) receivers: HashSet<Id>,
    pub(crate) receiver_functions: HashMap<Id, Box<dyn Fn(&Id, &mut Engine, &Vec<u8>)>>
}

impl Module for Messenger {
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        let ns = engine.runtime.ns.clone();
        let module = engine.mut_module_of::<Self>(id).unwrap();
        module.entity_id = *id;
        module.ns = Some(ns)
    }
}

impl Messenger {
    pub fn new<H: ClientHandle + Sized + 'static>() -> Self {
        Self {
            ns: None,
            receivers: Default::default(),
            handle_type: type_to_id::<H>(),
            entity_id: Id::new(),
            receiver_functions: Default::default()
        }
    }

    pub fn register_server_receiver<F: Fn(&Id, &mut Engine, M) + 'static, M: SerBin + DeBin>(&mut self, f: F) {
        //let m = move |&Id, &mut Engine, &mut dyn |
            //f(unsafe { &mut *std::mem::transmute::<_, &(*mut T, usize)>(Box::new(handle)).0 }, M::deserialize_bin(&data).unwrap());
        self.receiver_functions.insert(type_to_id::<F>(), Box::new(|_, _, _| ()));
    }

    pub fn call_client_fn<F: Fn(&mut T, M), T: ClientHandle, M: SerBin + DeBin>(&mut self, f: F, message: M) {
        let id = type_to_id::<F>();
    }

    pub fn clients(&self) -> Iter<Id> {
        self.receivers.iter()
    }

    pub fn has_client(&self, id: &Id) -> bool {
        self.receivers.contains(id)
    }

    pub fn add_client(&mut self, id: Id) -> bool {
        if !self.receivers.contains(&id) && self.ns.as_ref().unwrap().borrow().clients.contains_key(&id) {
            self.receivers.insert(id);
            let _ = self.ns.as_ref().unwrap().borrow().send(&id, &ServerPacket {
                conv_id: Id::new(),
                message: ServerMessage::AddClientHandle(self.entity_id, unsafe { self.handle_type }),
            });
            true
        } else { false }
    }

    pub fn remove_client(&mut self, id: &Id) -> bool {
        if self.receivers.contains(id) {
            self.receivers.remove(id);
            let _ = self.ns.as_ref().unwrap().borrow().send(id, &ServerPacket {
                conv_id: Id::new(),
                message: ServerMessage::RemoveClientHandle(self.entity_id),
            });
            true
        } else { false }
    }
}