

use std::fmt::{Debug};

use crate::Id;
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};
use crate::networking::NetResult;


#[derive(Debug, SerBin, DeBin)]
pub struct ServerPacket {
    pub conv_id: Id,
    pub message: ServerMessage
}

#[derive(Debug, SerBin, DeBin)]
pub enum ServerMessage {
    KeepAlive,
    Acknowlege(Id),
    Unregister(String),
    RegisterResponse(NetResult<ServerInfo, String>),
    Kick(String),
    Login(Id, String),
    Logout(Id, String),
    Ping(String),
    Pong(String),
    RawData(Vec<u8>),
    /// Entity Id, client handle TypeId
    AddClientHandle(Id, Id),
    /// Entity Id
    RemoveClientHandle(Id),
    /// timestamp, Entity Id, data
    ModMessage(u128, Id, Vec<u8>)
}

/// mods: Vec<(ModName, ModFlags, ZipHash, FileSize)>
#[derive(Debug, SerBin, DeBin)]
pub struct ServerInfo {
    pub server_version: String,
    pub mod_profile: String,
    pub mod_version: String,
    pub mods: Vec<(String, Vec<String>, String, u64)>
}
