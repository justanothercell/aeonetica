use std::cell::RefCell;
use std::rc::Rc;
use aeonetica_engine::{ClientId, EntityId, Id, TypeId};
use aeonetica_engine::nanoserde::{DeBin, SerBin};
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::messaging::ClientEntity;
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::nullable::Nullable;
use aeonetica_engine::util::type_to_id;
use aeonetica_server::ecs::Engine;
use crate::data_store::DataStore;
use crate::networking::NetworkClient;
use crate::renderer::Renderer;
use crate::renderer::window::events::Event;

#[allow(unused_variables)]
pub trait ClientHandle: ClientEntity {
    fn init(&mut self) {}
    fn owning_layer(&self) -> TypeId;
    fn start(&mut self, messenger: &mut ClientMessenger, renderer: Nullable<&mut Renderer>, store: &mut DataStore) {}
    fn remove(&mut self, messenger: &mut ClientMessenger, renderer: Nullable<&mut Renderer>, store: &mut DataStore) {}
    
    fn update(&mut self, messenger: &mut ClientMessenger, renderer: &mut Renderer, store: &mut DataStore, delta_time: f64) {}
    fn event(&mut self, event: &Event) -> bool { false }
}

pub struct ClientMessenger {
    nc: Rc<RefCell<NetworkClient>>,
    client_id: ClientId,
    entity_id: EntityId,
    pub(crate) client_receivers: IdMap<Box<dyn Fn(&mut dyn ClientHandle, &Vec<u8>)>>
}

impl ClientMessenger {
    pub(crate) fn new(nc: Rc<RefCell<NetworkClient>>, client_id: ClientId, entity_id: EntityId) -> Self {
        Self {
            nc,
            client_id,
            entity_id,
            client_receivers: Default::default()
        }
    }
}

impl ClientMessenger {
    pub fn register_receiver<F: Fn(&mut T, M) + 'static, T: ClientHandle, M: SerBin + DeBin>(&mut self, f: F) {
        let m = move |handle: &mut dyn ClientHandle, data: &Vec<u8>|
            f(unsafe { &mut *std::mem::transmute::<_, &(*mut T, usize)>(Box::new(handle)).0 }, M::deserialize_bin(data).unwrap());
        self.client_receivers.insert(type_to_id::<F>(), Box::new(m));
    }

    pub fn unregister_receiver<F: Fn(&mut T, M) + 'static, T: ClientHandle, M: SerBin + DeBin>(&mut self, _: F) {
        self.client_receivers.remove(&type_to_id::<F>());
    }

    pub fn call_server_fn<F: Fn(&EntityId, &mut Engine, &ClientId, M), M: SerBin + DeBin>(&mut self, _: F, message: M, mode: SendMode) {
        let id = type_to_id::<F>();
        let _ = self.nc.borrow().send(&ClientPacket {
            client_id: self.client_id,
            conv_id: Id::new(),
            message: ClientMessage::ModMessage(self.entity_id, id, message.serialize_bin()),
        }, mode);
    }
}