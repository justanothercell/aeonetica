use std::fmt::{Debug, Formatter};
use uuid::Uuid;
use crate::Id;
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};


#[derive(SerBin, DeBin)]
pub struct ClientPacket {
    pub client_id: Id,
    pub conv_id: Id,
    pub message: ClientMessage
}

impl Debug for ClientPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cid = Uuid::from_bytes(self.client_id);
        let c = Uuid::from_bytes(self.conv_id);
        if !f.alternate(){
            write!(f, "ClientPacket {{ client_id: {cid}, conv_id: {c}, message: {:?} }}", self.message)
        } else {
            write!(f, "ClientPacket {{\n    client_id: {cid},\n    conv_id: {c},\n    message: {}\n}}", format!("{:#?}", self.message).replace("\n", "\n    "))
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
pub enum ClientMessage {
    Login,
    Logout,
    KeepAlive,
    Register(ClientInfo),
    DownloadMod(String, u64),
    Unregister,
    Acknowlege(Id),
    Ping(String),
    Pong(String),
    RawData(Vec<u8>)
}

#[derive(Debug, SerBin, DeBin)]
pub struct ClientInfo {
    pub client_id: Id,
    pub client_version: String
}