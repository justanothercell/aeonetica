use std::fmt::{Debug, Formatter};
use crate::Id;
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};


#[derive(SerBin, DeBin)]
pub struct ServerPacket {
    message_id: Id,
    message: ServerMessage
}

impl Debug for ServerPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !f.alternate(){
            write!(f, "ClientPacket {{ message_id: {:?}, message: {:?} }}",
                   self.message_id, self.message)
        } else {
            write!(f, "ClientPacket {{\n    message_id: {:?},\n    message: {}\n}}",
                   self.message_id, format!("{:#?}", self.message).replace("\n", "\n    "))
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
pub enum ServerMessage {
    KeepAlive,
    Acknowlege(Id),
    Ping(String),
    Pong(String)
}
