use std::process::exit;
use aeonetica_engine::error::AError;
use aeonetica_engine::log;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::networking::server_packets::{ServerMessage, ServerPacket};
use crate::client_runtime::ClientRuntime;

impl ClientRuntime {
    pub(crate) fn handle_packet(&mut self, packet: &ServerPacket) -> Result<(), AError>{
        if let Some(handler) = self.awaiting_replies.remove(&packet.conv_id) {
            handler(self, packet);
            return Ok(())
        }
        match &packet.message {
            ServerMessage::Ping(msg) => self.nc.send(&ClientPacket{
                client_id: self.client_id,
                conv_id: packet.conv_id.clone(),
                message: ClientMessage::Pong(msg.clone()),
            })?,
            ServerMessage::Unregister(reason) => {
                log!("server unregistered client: {reason}");
                exit(0)
            }
            _ => ()
        }
        Ok(())
    }
}