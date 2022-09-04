use std::collections::HashMap;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use lru::LruCache;

use common::message::{ServerMessage, UserMessage};
use common::udp_message::{UdpServerMessage, UdpUserMessage};

pub fn start_tcp_client(
    host: &str,
    server_port: &str,
) -> (Sender<UserMessage>, Receiver<ServerMessage>) {
    let (user_sender, user_receiver) = channel::<UserMessage>();
    let (server_sender, server_receiver) = channel::<ServerMessage>();

    let mut socket = TcpStream::connect(format!("{}:{}", host, server_port)).unwrap();
    let mut socket_clone = socket.try_clone().unwrap();

    thread::spawn(move || {
        let mut bytes = [0u8; 64];
        loop {
            match socket_clone.read_exact(&mut bytes) {
                Ok(_) => {
                    let udp_message = UdpServerMessage::from_bytes(&bytes);

                    match &udp_message {
                        UdpServerMessage::Message(_, message) => {
                            server_sender.send(message.clone()).unwrap();
                        }
                        UdpServerMessage::Ack(_) => {
                        }
                    }
                }
                Err(e) => {
                    println!("Server disconnected! {}", e);
                }
            }
        }
    });

    thread::spawn(move || {
        loop {
            match user_receiver.try_recv() {
                Ok(message) => {
                    let udp_message = UdpUserMessage::Message(0, message);
                    socket.write(&udp_message.to_bytes()).unwrap();
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
