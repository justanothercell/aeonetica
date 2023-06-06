
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, UdpSocket};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use aeonetica_engine::error::{Error, Fatality, ErrorResult};
use aeonetica_engine::error::builtin::NetworkError;
use aeonetica_engine::{Id, log};
use aeonetica_engine::nanoserde::{SerBin, DeBin};
use aeonetica_engine::networking::{MAX_PACKET_SIZE, SendMode};
use aeonetica_engine::networking::client_packets::ClientPacket;
use aeonetica_engine::networking::server_packets::ServerPacket;
use aeonetica_engine::util::id_map::IdMap;

mod protocol;

pub(crate) struct NetworkServer {
    pub(crate) udp: UdpSocket,
    pub(crate) received: Arc<Mutex<Vec<(SocketAddr, ClientPacket)>>>,
    pub(crate) clients: IdMap<ClientHandle>,
    pub(crate) tcp: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Vec<Vec<u8>>>>>>>
}

pub(crate) struct ClientHandle {
    pub(crate) last_seen: Instant,
    pub(crate) client_addr: SocketAddr,
}

impl NetworkServer {
    pub(crate) fn start(addr: &str) -> ErrorResult<Self>{
        let socket = UdpSocket::bind(addr)?;
        let sock = socket.try_clone()?;
        let received = Arc::new(Mutex::new(vec![]));
        let tcp_sockets = Arc::new(Mutex::new(HashMap::new()));
        let recv = received.clone();
        let recv_tcp = received.clone();
        let tcp = tcp_sockets.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; MAX_PACKET_SIZE];
            loop {
                match sock.recv_from(&mut buf) {
                    Ok((len, src)) => {
                        match DeBin::deserialize_bin(&buf[..len]) {
                            Ok(packet) => recv.lock().unwrap().push((src, packet)),
                            Err(e) => log!(ERROR, "invalid client packet from {src}: {e}")
                        }
                    },
                    Err(_e) => {}
                }
            }
        });
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(false).unwrap();
        std::thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                stream.set_nonblocking(false).unwrap();
                let addr = stream.peer_addr().unwrap();
                let queue = Arc::new(Mutex::new(vec![]));
                tcp.lock().unwrap().insert(addr, queue.clone());
                let recv_tcp_inner = recv_tcp.clone();
                let mut write_stream = stream.try_clone().unwrap();
                thread::spawn(move || {
                    loop {
                        let r = (||{
                            let mut size = [0u8; 4];
                            stream.read_exact(&mut size)?;
                            let size = u32::from_le_bytes(size);
                            let mut buffer: Vec<u8> = vec![0; size as usize];
                            stream.read_exact(&mut buffer[..])?;
                            match DeBin::deserialize_bin(&buffer[..]) {
                                Ok(packet) => recv_tcp_inner.lock().unwrap().push((addr, packet)),
                                Err(e) => log!(ERROR, "invalid client packet from {addr}: {e}")
                            }
                            Ok::<_, std::io::Error>(())
                        })();
                        if r.is_err() { break }
                    }
                    log!("terminated tcp connection with {}", addr)
                });
                thread::spawn(move || {
                    loop {
                        let mut queued: Vec<Vec<u8>> = vec![];
                        {
                            let mut q = queue.lock().unwrap();
                            std::mem::swap(&mut queued, &mut q);
                        }
                        if queued.len() > 0 {
                            for msg in queued {
                                write_stream.write(&(msg.len() as u32).to_le_bytes()).unwrap();
                                write_stream.write(&msg[..]).unwrap();
                            }
                        }
                    }
                });
            }
        });
        Ok(Self {
            udp: socket,
            received,
            clients: Default::default(),
            tcp: tcp_sockets
        })
    }

    pub(crate) fn queued_packets(&mut self) -> Vec<(SocketAddr, ClientPacket)> {
        std::mem::replace(&mut self.received.lock().unwrap() as &mut Vec<(SocketAddr, ClientPacket)>, vec![])
    }

    pub(crate) fn send(&self, client_id: &Id, packet: &ServerPacket, mode: SendMode) -> ErrorResult<()>{
        self.clients.get(client_id).map(|client| {
            self.send_raw(client.client_addr, packet, mode)
        }).unwrap_or(Err(Error::new(NetworkError(format!("client {client_id} does not exist")), Fatality::DEFAULT, true)))?;

        Ok(())
    }

    pub(crate) fn send_raw(&self, ip_addr: SocketAddr, packet: &ServerPacket, mode: SendMode) -> ErrorResult<()>{
        let data = SerBin::serialize_bin(packet);
        match mode {
            SendMode::Quick => {
                if data.len() > MAX_PACKET_SIZE {
                    return Err(Error::new(NetworkError(format!("Packet is too large: {} > {}", data.len(), MAX_PACKET_SIZE)), Fatality::WARN, false))
                }
                let sock = self.udp.try_clone()?;
                std::thread::spawn(move || sock.send_to(&data[..], ip_addr));
            }
            SendMode::Safe => {
                if let Some(tcp_queue) = self.tcp.lock().unwrap().get_mut(&ip_addr) {
                    let mut queue = tcp_queue.lock().unwrap();
                    queue.push(data);
                }
            }
        }
        Ok(())
    }
}