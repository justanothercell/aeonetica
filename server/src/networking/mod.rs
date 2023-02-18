use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex, MutexGuard};
use aeonetica_engine::error::{AError};
use aeonetica_engine::{Id, log_err};
use aeonetica_engine::nanoserde::DeBin;
use aeonetica_engine::networking::client_packets::{ClientPacket};

pub(crate) struct NetworkServer {
    socket: UdpSocket,
    received: Arc<Mutex<Vec<ClientPacket>>>,
    pub(crate) clients: HashMap<Id, ClientHandle>
}

pub(crate) struct ClientHandle {
    client_addr: SocketAddr,
}

impl NetworkServer {
    pub(crate) fn start(addr: &str) -> Result<Self, AError>{
        let socket = UdpSocket::bind(addr)?;
        let sock = socket.try_clone()?;
        let received = Arc::new(Mutex::new(vec![]));
        let recv = received.clone();
        std::thread::spawn(move || {
            loop {
                let mut buf = [0u8; 65000]; // apparently the largest possible, lets hope thats true, idk what happens when larger
                match sock.recv_from(&mut buf) {
                    Ok((len, src)) => {
                        match DeBin::deserialize_bin(&buf[..len]) {
                            Ok(packet) => recv.lock().unwrap().push(packet),
                            Err(e) => eprintln!("invalid client packet from {src}: {e}")
                        }
                    },
                    Err(e) => {
                        log_err!("couldn't recieve a datagram: {}", e);
                    }
                }
            }
        });
        Ok(Self {
            socket,
            received,
            clients: Default::default(),
        })
    }

    pub(crate) fn queued_packets(&mut self) -> Vec<ClientPacket> {
        let mut packets = vec![];
        std::mem::swap(&mut self.received.lock().unwrap() as &mut Vec<ClientPacket>, &mut packets);
        packets
    }
}