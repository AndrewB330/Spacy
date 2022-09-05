use std::collections::HashMap;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use common::message::{ServerMessageData, UserMessageData};
use std::thread;

pub fn start_tcp_client(
    host: &str,
    server_port: &str,
) -> (Sender<UserMessageData>, Receiver<ServerMessageData>) {
    let (user_sender, user_receiver) = channel::<UserMessageData>();
    let (server_sender, server_receiver) = channel::<ServerMessageData>();

    let mut socket = TcpStream::connect(format!("{}:{}", host, server_port)).unwrap();
    let mut socket_clone = socket.try_clone().unwrap();

    thread::spawn(move || {
        let mut bytes = [0u8; 64];
        loop {
            match socket_clone.read_exact(&mut bytes) {
                Ok(_) => {
                    server_sender
                        .send(ServerMessageData::from(&bytes[..]))
                        .unwrap();
                }
                Err(e) => {
                    println!("Server disconnected! {}", e);
                }
            }
        }
    });

    thread::spawn(move || loop {
        match user_receiver.try_recv() {
            Ok(message) => {
                let v: Vec<u8> = message.into();
                socket.write(&v).unwrap();
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                panic!("Unexpected end of the channel!")
            }
        }
    });

    (user_sender, server_receiver)
}
