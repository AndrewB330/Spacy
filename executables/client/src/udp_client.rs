use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use common::message::{ServerMessage, UserMessage};

pub fn start_udp_client(server_port: &str) -> (Sender<UserMessage>, Receiver<ServerMessage>) {
    let (user_sender, user_receiver) = channel::<UserMessage>();
    let (server_sender, server_receiver) = channel::<ServerMessage>();

    let addrs = vec![
        SocketAddr::from_str("127.0.0.1:8080").unwrap(),
        SocketAddr::from_str("127.0.0.1:8081").unwrap(),
        SocketAddr::from_str("127.0.0.1:8082").unwrap(),
        SocketAddr::from_str("127.0.0.1:8083").unwrap(),
        SocketAddr::from_str("127.0.0.1:8084").unwrap(),
    ];

    let socket = UdpSocket::bind(addrs.as_slice())
        .expect("Could not bind client socket");

    socket.connect(format!("127.0.0.1:{}", server_port))
        .expect("Could not connect to server");

    let socket_clone = socket.try_clone().unwrap();

    thread::spawn(move || {
        let mut bytes = [0u8; 1 << 15];
        loop {
            match socket_clone.recv(&mut bytes) {
                Ok(_) => {}
                Err(_) => {}
            }
            server_sender.send(ServerMessage::from_bytes(&bytes)).unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            match user_receiver.try_recv() {
                Ok(message) => {
                    socket.send(&message.to_bytes()).unwrap();
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    panic!("Unexpected end of the channel!")
                }
            }
        }
    });

    (user_sender, server_receiver)
}
