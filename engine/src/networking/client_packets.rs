use std::fmt::{Debug, Formatter};
use crate::Id;
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};


#[derive(SerBin, DeBin)]
pub struct ClientPacket {
    pub client_id: Id,
    pub message_id: Id,
    pub message: ClientMessage
}

impl Debug for ClientPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !f.alternate(){
            write!(f, "ClientPacket {{ client_id: {:?}, message_id: {:?}, message: {:?} }}",
                   self.client_id, self.message_id, self.message)
        } else {
            write!(f, "ClientPacket {{\n    client_id: {:?},\n    message_id: {:?},\n    message: {}\n}}",
                   self.client_id, self.message_id, format!("{:#?}", self.message).replace("\n", "\n    "))
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
pub enum ClientMessage {
    Login,
    Logout,
    KeepAlive,
    Acknowlege(Id),
    Ping(String),
    Pong(String)
}
