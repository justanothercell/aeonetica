use std::fmt::{Debug, Formatter};
use aeonetica_engine::nanoserde::{DeBin, DeBinErr, SerBin};
use aeonetica_client::messaging::ClientHandle;
use aeonetica_server::ecs::messaging::Message;
use aeonetica_engine::{log, nanoserde};

pub(crate) struct MyHandle {

}

impl ClientHandle for MyHandle {
    fn receive_data(&mut self, data: &Vec<u8>) {
        let broadcastings: Broadcastings = Broadcastings::deserialize_bin(data).unwrap()
        if broadcastings.0.len() > 0 {
            log!("Server says:");
            for msg in broadcastings.0 {
                log!("- {msg}")
            }
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
pub struct Broadcastings(pub Vec<String>);

impl Message for Broadcastings {}