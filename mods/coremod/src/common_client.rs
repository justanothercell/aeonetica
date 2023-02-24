use std::fmt::{Debug,};
use aeonetica_engine::nanoserde::{DeBin, SerBin};
use aeonetica_engine::networking::messaging::ClientHandle;
use aeonetica_server::ecs::messaging::Message;
use aeonetica_engine::{log, nanoserde};

pub(crate) struct MyClientHandle {

}

impl ClientHandle for MyClientHandle {
    fn init(&mut self) {
        log!("my client handle initialized")
    }
    
    fn receive_data(&mut self, data: &Vec<u8>) {
        let broadcastings: Broadcastings = Broadcastings::deserialize_bin(data).unwrap();
        if broadcastings.0.len() > 0 {
            log!("Server says:");
            for msg in broadcastings.0 {
                log!("- {msg}")
            }
        }
    }

    fn remove(&mut self) {
        log!("my client handle removed")
    }
}

#[derive(Debug, SerBin, DeBin)]
pub struct Broadcastings(pub Vec<String>);

impl Message for Broadcastings {}