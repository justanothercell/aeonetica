
use std::fmt::{Debug};

use crate::{ClientId, EntityId, Id, TypeId};
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};


#[derive(Debug, SerBin, DeBin)]
pub struct ClientPacket {
    pub client_id: ClientId,
    pub conv_id: Id,
    pub message: ClientMessage
}

#[derive(Debug, SerBin, DeBin)]
pub enum ClientMessage {
    Login,
    Logout,
    KeepAlive,
    Register(ClientInfo),
    DownloadMod(String, u64),
    Acknowlege(Id),
    Ping(String),
    Pong(String),
    RawData(Vec<u8>),
    ModMessage(EntityId, TypeId, Vec<u8>)
}

#[derive(Debug, SerBin, DeBin)]
pub struct ClientInfo {
    pub client_id: ClientId,
    pub client_version: String
}