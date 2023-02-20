use std::net::{UdpSocket};
use std::sync::{Arc, Mutex};
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{log_err};
use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::networking::MAX_PACKET_SIZE;
use aeonetica_engine::networking::client_packets::{ClientPacket};
use aeonetica_engine::networking::server_packets::ServerPacket;

mod protocol;

pub(crate) struct NetworkClient {
    pub(crate) socket: UdpSocket,
    received: Arc<Mutex<Vec<ServerPacket>>>
}

impl NetworkClient {
    pub(crate) fn start(addr: &str, server: &str) -> Result<Self, AError>{
        let socket = UdpSocket::bind(addr)?;
        socket.connect(server)?;
        let sock = socket.try_clone()?;
        let received = Arc::new(Mutex::new(vec![]));
        let recv = received.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; MAX_PACKET_SIZE];
            loop {
                match sock.recv_from(&mut buf) {
                    Ok((len, src)) => {
                        match DeBin::deserialize_bin(&buf[..len]) {
                            Ok(packet) => recv.lock().unwrap().push(packet),
                            Err(e) => log_err!("invalid server packet from {src}: {e}")
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
            received
        })
    }

    pub(crate) fn queued_packets(&mut self) -> Vec<ServerPacket> {
        let mut packets = vec![];
        std::mem::swap(&mut self.received.lock().unwrap() as &mut Vec<ServerPacket>, &mut packets);
        packets
    }

    pub(crate) fn send(&self, packet: &ClientPacket) -> Result<(), AError>{
        let data = SerBin::serialize_bin(packet);
        if data.len() > MAX_PACKET_SIZE {
            return Err(AError::new(AET::NetworkError(format!("Packet is too large: {} > {}", data.len(), MAX_PACKET_SIZE))))
        }
        self.socket.send(data.as_slice())?;
        Ok(())
    }
}