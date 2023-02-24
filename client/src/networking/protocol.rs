use std::process::exit;
use aeonetica_engine::error::AError;
use aeonetica_engine::log;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use aeonetica_engine::util::i64_to_typeid;
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
                self.registered_handles.get(unsafe { &i64_to_typeid(*handle_id) }).map(|creator| {
                    let mut handle = creator();
                    handle.init();
                    self.handles.insert(*id, handle)
                });
            }
            ServerMessage::RemoveClientHandle(id) => {
                self.handles.remove(id).map(|h| {
                    h.remove()
                });
            }
            _ => ()
        }
        Ok(())
    }
}