use std::process::exit;
use aeonetica_engine::error::AError;
use aeonetica_engine::log;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use crate::client_runtime::{ClientHandleBox, ClientRuntime};
use crate::networking::messaging::ClientMessenger;

impl ClientRuntime {
    pub(crate) fn handle_queued(&mut self) -> Result<(), AError> {
        let packets = self.nc.borrow_mut().queued_packets();
        packets.into_iter().map(|packet| self.handle_packet(&packet))
        .reduce(|acc, r| {
            acc?;
            r?;
            Ok(())
        }).unwrap_or(Ok(()))
    }

    pub(crate) fn handle_packet(&mut self, packet: &ServerPacket) -> Result<(), AError>{
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
                log!("added client handle");
                self.registered_handles.get(handle_id).map(|creator| {
                    let mut handle = creator();
                    handle.init();
                    let mut messenger = ClientMessenger::new(self.nc.clone(), self.client_id, *eid);
                    handle.start(&mut messenger);
                    self.handles.insert(*eid, ClientHandleBox {
                        handle,
                        messenger,
                    })
                });
            }
            ServerMessage::RemoveClientHandle(id) => {
                log!("remove client handle");
                if let Some(mut h) = self.handles.remove(id) {
                    h.handle.remove(&mut h.messenger)
                }
            }
            ServerMessage::ModMessage(eid, rid, data) => {
                if let Some(h) = self.handles.get_mut(eid) {
                    if let Some(f) = h.messenger.client_receivers.get(rid) {
                        f(&mut *h.handle, data)
                    }
                }
            }
            _ => ()
        }
        Ok(())
    }
}