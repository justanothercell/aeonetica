use std::net::UdpSocket;
use aeonetica_engine::nanoserde::SerBin;
use aeonetica_engine::networking::client_packets::{ClientMessage, ClientPacket};
use aeonetica_engine::uuid::Uuid;

fn main() {
    // nc -u 127.0.01 6090
    let socket = UdpSocket::bind("127.0.0.1:9000")
        .expect("Could not bind client socket");
    socket.connect("127.0.0.1:6090")
        .expect("Could not connect to server");
    let client_id = Uuid::new_v4().into_bytes();

    let data = SerBin::serialize_bin(&ClientPacket {
        client_id: client_id.clone(),
        message_id: Uuid::new_v4().into_bytes(),
        message: ClientMessage::Ping("hello, world!".to_string()),
    });
    socket.send(data.as_slice())
        .expect("Failed to write to server");
/*
    socket.recv_from(&mut buffer)
        .expect("Could not read into buffer");
    print!("{}", str::from_utf8(&buffer)
        .expect("Could not write buffer as string"));
        */
}