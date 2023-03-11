use std::collections::HashMap;
use nanoserde::{DeBin, SerBin};
use crate::Id;
use crate::util::type_to_id;

pub trait ClientHandle {
    fn init(&mut self) {}
    fn start(&mut self, messenger: &mut ClientMessenger) {}
    fn remove(&mut self, messenger: &mut ClientMessenger) {}
}

pub struct ClientMessenger {
    client_receivers: HashMap<Id, Box<dyn Fn(&mut dyn ClientHandle, Vec<u8>)>>
}

impl ClientMessenger {
    pub fn new() -> Self {
        Self {
            client_receivers: Default::default()
        }
    }
    pub fn register_client_receiver<F: Fn(&mut T, M) + 'static, T: ClientHandle, M: SerBin + DeBin>(&mut self, f: F) {
        let m = move |handle: &mut dyn ClientHandle, data: Vec<u8>|
            f(unsafe { &mut *std::mem::transmute::<_, &(*mut T, usize)>(Box::new(handle)).0 }, M::deserialize_bin(&data).unwrap());
        self.client_receivers.insert(type_to_id::<F>(), Box::new(m));
    }
}