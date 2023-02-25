use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;
use crate::Id;
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};


#[derive(Debug, SerBin, DeBin)]
pub struct ClientPacket {
    pub client_id: Id,
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
    ModMessage(u128, HashMap<Id, Vec<u8>>)
}

#[derive(Debug, SerBin, DeBin)]
pub struct ClientInfo {
    pub client_id: Id,
    pub client_version: String
}