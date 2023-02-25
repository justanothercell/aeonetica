use std::process::exit;
use aeonetica_engine::error::AError;
use aeonetica_engine::log;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use crate::client_runtime::ClientRuntime;

impl ClientRuntime {
    pub(crate) fn handle_queued(&mut self) -> Result<(), AError> {
        self.nc.queued_packets().into_iter().map(|packet| self.handle_packet(&packet))
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
            ServerMessage::Ping(msg) => self.nc.send(&ClientPacket{
                client_id: self.client_id,
                conv_id: packet.conv_id,
                message: ClientMessage::Pong(msg.clone()),
            })?,
            ServerMessage::Unregister(reason) => {
                log!("server unregistered client: {reason}");
                exit(0)
            }
            ServerMessage::AddClientHandle(id, handle_id) => {
                log!("added client handle");
                self.registered_handles.get(handle_id).map(|creator| {
                    let mut handle = creator();
                    handle.init();
                    self.handles.insert(*id, handle)
                });
            }
            ServerMessage::RemoveClientHandle(id) => {
                log!("remove client handle");
                if let Some(mut h) = self.handles.remove(id) {
                    h.remove()
                }
            }
            ServerMessage::ModMessage(timestamp, id, message) => {
                log!("mod message");
                if timestamp > &self.last_server_msg {
                    self.last_server_msg = *timestamp;
                    if let Some(h) = self.handles.get_mut(id) {
                        h.receive_data(message)
                    }
                }
            }
            _ => ()
        }
        Ok(())
    }
}