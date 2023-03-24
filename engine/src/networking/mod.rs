use std::fmt::Debug;
use nanoserde::{DeBin, SerBin};

pub mod client_packets;
pub mod server_packets;
pub mod messaging;

pub const MAX_PACKET_SIZE: usize = 25000;
pub const MAX_RAW_DATA_SIZE: usize = MAX_PACKET_SIZE - 26;
pub const MOD_DOWNLOAD_CHUNK_SIZE: usize = 65000;

#[derive(Debug, SerBin, DeBin)]
pub enum NetResult<T: Debug + SerBin + DeBin, E: Debug + SerBin + DeBin>{
    Ok(T),
    Err(E)
}

impl<T: Debug + SerBin + DeBin, E: Debug + SerBin + DeBin> From<NetResult<T, E>> for Result<T, E> {
    fn from(val: NetResult<T, E>) -> Self {
        match val {
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

/// The mode the network message will be send in
#[derive(Copy, Clone, Debug)]
pub enum SendMode {
    /// Quick and lossy. Use for discardable packets, such as continous updates.
    Quick,
    /// Safe transfer, but slow. Data is buffered. Use for things like downloading resources or events that only happen on state change.
    Safe
}