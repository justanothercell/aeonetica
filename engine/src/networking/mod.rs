use std::fmt::Debug;
use nanoserde::{DeBin, SerBin};

pub mod client_packets;
pub mod server_packets;
pub mod messaging;

pub const MAX_PACKET_SIZE: usize = 1500;
pub const MAX_RAW_DATA_SIZE: usize = 1500 - 26;

#[derive(Debug, SerBin, DeBin)]
pub enum NetResult<T: Debug + SerBin + DeBin, E: Debug + SerBin + DeBin>{
    Ok(T),
    Err(E)
}

impl<T: Debug + SerBin + DeBin, E: Debug + SerBin + DeBin> Into<Result<T, E>> for NetResult<T, E> {
    fn into(self) -> Result<T, E> {
        match self {
            NetResult::Ok(v) => Ok(v),
            NetResult::Err(e) => Err(e)
        }
    }
}

impl<T: Debug + SerBin + DeBin, E: Debug + SerBin + DeBin> From<Result<T, E>> for NetResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => NetResult::Ok(v),
            Err(e) => NetResult::Err(e)
        }
    }
}
