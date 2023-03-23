use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use aeonetica_engine::error::{AError, AET};
use aeonetica_engine::{log_err};
use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::networking::{MAX_PACKET_SIZE, SendMode};
use aeonetica_engine::networking::client_packets::{ClientPacket};
use aeonetica_engine::networking::server_packets::ServerPacket;

mod protocol;
pub mod messaging;

pub(crate) struct NetworkClient {
    pub(crate) udp: UdpSocket,
    pub(crate) tcp: TcpStream,
    received: Arc<Mutex<Vec<ServerPacket>>>
}

impl NetworkClient {
    pub(crate) fn start(addr: &str, server: &str) -> Result<Self, AError>{
        let tcp = TcpStream::connect(server)?;
        let udp = UdpSocket::bind(addr)?;
        udp.connect(server)?;
        let udp_sock = udp.try_clone()?;
        let mut tcp_sock = tcp.try_clone()?;
        let received = Arc::new(Mutex::new(vec![]));
        let recv_udp = received.clone();
        let recv_tcp = received.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; MAX_PACKET_SIZE];
            loop {
                match udp_sock.recv_from(&mut buf) {
                    Ok((len, src)) => match DeBin::deserialize_bin(&buf[..len]) {
                       Ok(packet) => recv_udp.lock().unwrap().push(packet),
                       Err(e) => log_err!("invalid server packet from {src}: {e}")
                    },
                    Err(e) => {
                        log_err!("couldn't recieve a datagram: {}", e);
                    }
                }
            }
        });
        std::thread::spawn(move || {
            loop {
                let mut size = [0u8;4];
                tcp_sock.read_exact(&mut size).unwrap();
                let size = u32::from_le_bytes(size);
                let mut buffer: Vec<u8> = vec![0;size as usize];
                tcp_sock.read_exact(&mut buffer[..]).unwrap();
                match DeBin::deserialize_bin(&buffer[..]) {
                    Ok(packet) => recv_tcp.lock().unwrap().push(packet),
                    Err(e) => log_err!("invalid server packet: {e}")
                }
            }
        });
        Ok(Self {
            udp,
            tcp,
            received
        })
    }

    pub(crate) fn queued_packets(&mut self) -> Vec<ServerPacket> {
        let mut packets = vec![];
        std::mem::swap(&mut self.received.lock().unwrap() as &mut Vec<ServerPacket>, &mut packets);
        packets
    }

    pub(crate) fn send(&self, packet: &ClientPacket, mode: SendMode) -> Result<(), AError>{
        let data = SerBin::serialize_bin(packet);
        if data.len() > MAX_PACKET_SIZE {
            return Err(AError::new(AET::NetworkError(format!("Packet is too large: {} > {}", data.len(), MAX_PACKET_SIZE))))
        }
        match mode {
            SendMode::Quick => {
                let sock = self.udp.try_clone()?;
                std::thread::spawn(move || sock.send(&data[..]).map_err(|e| {
                    let e: AError = e.into();
                    e.log();
                }));
            }
            SendMode::Safe => {
                let mut tcp = self.tcp.try_clone()?;
                std::thread::spawn(move || {
                    let _ = tcp.write_all(&(data.len() as u32).to_le_bytes());
                    let _ = tcp.write_all(&data[..]);
                });
            }
        }
        Ok(())
    }
}