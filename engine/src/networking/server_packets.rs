use std::fmt::{Debug, Formatter};
use uuid::Uuid;
use crate::Id;
use crate::nanoserde;
use crate::nanoserde::{SerBin, DeBin};
use crate::networking::NetResult;


#[derive(SerBin, DeBin)]
pub struct ServerPacket {
    pub conv_id: Id,
    pub message: ServerMessage
}

impl Debug for ServerPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = Uuid::from_bytes(self.conv_id);
        if !f.alternate(){
            write!(f, "ClientPacket {{ conv_id: {c}, message: {:?} }}", self.message)
        } else {
            write!(f, "ClientPacket {{\n    conv_id: {c},\n    message: {}\n}}", format!("{:#?}", self.message).replace("\n", "\n    "))
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
pub enum ServerMessage {
    KeepAlive,
    Acknowlege(Id),
    RegisterResponse(NetResult<ServerInfo, String>),
    DownloadModPacket(usize, Vec<u8>),
    Kick(String),
    Login(Id, String),
    Logout(Id, String),
    Ping(String),
    Pong(String),
    RawData(Vec<u8>)
}

/// mods: Vec<(ModName, ModFlags, ZipHash, FileSize)>
#[derive(Debug, SerBin, DeBin)]
pub struct ServerInfo {
    pub server_version: String,
    pub mod_profile: String,
    pub mod_version: String,
    pub mods: Vec<(String, Vec<String>, String, u64)>
}
