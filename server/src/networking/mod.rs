use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{Id, log_err};
use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::networking::MAX_PACKET_SIZE;
use aeonetica_engine::networking::client_packets::ClientPacket;
use aeonetica_engine::networking::server_packets::ServerPacket;
use crate::server_runtime::ServerRuntime;

mod protocol;

pub(crate) struct NetworkServer {
    pub(crate) socket: UdpSocket,
    pub(crate) received: Arc<Mutex<Vec<(SocketAddr, ClientPacket)>>>,
    pub(crate) clients: HashMap<Id, ClientHandle>
}

pub(crate) struct ClientHandle {
    client_addr: SocketAddr,
    socket: UdpSocket,
    awaiting_replies: HashMap<Id, fn(&mut ServerRuntime, &ClientPacket)>
}

impl NetworkServer {
    pub(crate) fn start(addr: &str) -> Result<Self, AError>{
        let socket = UdpSocket::bind(addr)?;
        let sock = socket.try_clone()?;
        let received = Arc::new(Mutex::new(vec![]));
        let recv = received.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; MAX_PACKET_SIZE];
            loop {
                match sock.recv_from(&mut buf) {
                    Ok((len, src)) => {
                        match DeBin::deserialize_bin(&buf[..len]) {
                            Ok(packet) => recv.lock().unwrap().push((src, packet)),
                            Err(e) => log_err!("invalid client packet from {src}: {e}")
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

    pub(crate) fn queued_packets(&mut self) -> Vec<(SocketAddr, ClientPacket)> {
        let mut packets = vec![];
        std::mem::swap(&mut self.received.lock().unwrap() as &mut Vec<(SocketAddr, ClientPacket)>, &mut packets);
        packets
    }

    pub(crate) fn send(&self, client_id: &Id, packet: &ServerPacket) -> Result<(), AError>{
        let data = SerBin::serialize_bin(packet);
        if data.len() > MAX_PACKET_SIZE {
            return Err(AError::new(AET::NetworkError(format!("Packet is too large: {} > {}", data.len(), MAX_PACKET_SIZE))))
        }
        let c = self.clients.get(client_id).ok_or(AError::new(AET::NetworkError("client does nto exist".to_string())))?;
        c.socket.send(&data[..])?;
        Ok(())
    }

    pub(crate) fn send_raw(&self, ip_addr: &str, packet: &ServerPacket) -> Result<(), AError>{
        let data = SerBin::serialize_bin(packet);
        let sock = self.socket.try_clone()?;
        sock.connect(ip_addr)?;
        sock.send(&data[..])?;
        Ok(())
    }
}