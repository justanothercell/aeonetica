use std::process::exit;
use aeonetica_engine::error::ErrorResult;
use aeonetica_engine::log;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use aeonetica_engine::util::nullable::Nullable::{Null, Value};
use crate::client_runtime::{ClientHandleBox, ClientRuntime};
use crate::data_store::DataStore;
use crate::networking::messaging::ClientMessenger;
use crate::renderer::context::RenderContext;

impl ClientRuntime {
    pub(crate) fn handle_queued(&mut self, store: &mut DataStore, context: &mut RenderContext) -> ErrorResult<()> {
        let packets = self.nc.borrow_mut().queued_packets();
        packets.into_iter().map(|packet| self.handle_packet(&packet, store, context))
        .reduce(|acc, r| {
            acc?;
            r?;
            Ok(())
        }).unwrap_or(Ok(()))
    }

    pub(crate) fn handle_packet(&mut self, packet: &ServerPacket, store: &mut DataStore, context: &mut RenderContext) -> ErrorResult<()>{
        if let Some(handler) = self.awaiting_replies.remove(&packet.conv_id) {
            handler(self, packet);
            return Ok(())
        }
        match &packet.message {
            ServerMessage::Ping(msg) => self.nc.borrow().send(&ClientPacket{
                client_id: self.client_id,
                conv_id: packet.conv_id,
                message: ClientMessage::Pong(msg.clone()),
            }, SendMode::Safe)?,
            ServerMessage::Unregister(reason) => {
                log!("server unregistered client: {reason}");
                exit(0)
            }
            ServerMessage::AddClientHandle(eid, handle_id) => {
                log!("added client handle: {handle_id}");
                self.registered_handles.get(handle_id).map(|creator| {
                    let mut handle = creator();
                    handle.init();
                    let mut messenger = ClientMessenger::new(self.nc.clone(), self.client_id, *eid);
                    if let Some(layer) = context.layer_stack.layer_map.get(&handle.owning_layer()) {
                        handle.start(&mut messenger, Value(&mut layer.borrow_mut().renderer), store);
                    } else {
                        handle.start(&mut messenger, Null, store);
                    }
                    self.handles.insert(*eid, ClientHandleBox {
                        handle,
                        messenger,
                    })
                });
            }
            ServerMessage::RemoveClientHandle(id) => {
                log!("remove client handle");
                if let Some(mut h) = self.handles.remove(id) {
                    if let Some(layer) = context.layer_stack.layer_map.get(&h.handle.owning_layer()) {
                        h.handle.remove(&mut h.messenger, Value(&mut layer.borrow_mut().renderer), store);
                    } else {
                        h.handle.remove(&mut h.messenger, Null, store);
                    }
                }
            }
            ServerMessage::ModMessage(eid, rid, data) => {
                if let Some(h) = self.handles.get_mut(eid) {
                    if let Some(f) = h.messenger.client_receivers.remove(rid) {
                        if let Some(layer) = context.layer_stack.layer_map.get(&h.handle.owning_layer()) {
                            f(&mut *h.handle, &mut h.messenger, Value(&mut layer.borrow_mut().renderer), store, data)
                        } else {
                            f(&mut *h.handle, &mut h.messenger, Null, store, data)
                        }
                        h.messenger.client_receivers.insert(*rid, f);
                    }
                }
            }
            _ => ()
        }
        Ok(())
    }
}