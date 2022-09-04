use log::info;
use lru::LruCache;
use rand::random;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use common::message::{ServerMessage, UserId, UserMessage};
use common::udp_message::{UdpServerMessage, UdpUserMessage};
use server::user_connections::{UserConnection, UserConnectionEvent};

fn connection_loop(sender: Sender<UserMessage>, receiver: Receiver<ServerMessage>, addr: SocketAddr, mut socket: TcpStream) {
    info!("Connected: {:?}", addr);

    let mut message_id = 0;

    let mut socket_clone = socket.try_clone().unwrap();
    // read loop
    thread::spawn(move || {
        let mut bytes = [0u8; 64];
        loop {
            match socket_clone.read_exact(&mut bytes) {
                Ok(_) => {
                    let udp_message = UdpUserMessage::from_bytes(&bytes);

                    match &udp_message {
                        UdpUserMessage::Message(_, message) => {
                            sender.send(message.clone()).unwrap();
                        }
                        UdpUserMessage::Ack(_) => {
                        }
                    }
                }
                Err(e) => {
                    return;
                    println!("Server disconnected! {}", e);
                }
            }
        }
    });

    thread::spawn(move || {
        loop {
            match receiver.try_recv() {
                Ok(message) => {
                    let udp_message = UdpServerMessage::Message(0, message);
                    socket.write(&udp_message.to_bytes()).unwrap();
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    panic!("Unexpected end of the channel!")
                }
            }
        }
    });
}

fn main_loop(socket: TcpListener, connection_sender: Sender<UserConnectionEvent>) {
    loop {
        match socket.accept() {
            Ok((stream, addr)) => {
                let user_to_server = channel();
                let server_to_user = channel();

                connection_sender
                    .send(UserConnectionEvent::Connected(UserConnection {
                        user_id: UserId::new(),
                        from_user: Mutex::new(user_to_server.1),
                        to_user: Mutex::new(server_to_user.0),
                    }))
                    .unwrap();

                thread::spawn(move || {
                    connection_loop(user_to_server.0, server_to_user.1, addr, stream);
                });
            }
            Err(_) => {}
        }
    }
}

pub fn start_tcp_server(port: &str) -> Receiver<UserConnectionEvent> {
    let (connection_sender, connection_receiver) = channel();

    let socket = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Could not bind socket");

    thread::spawn(move || {
        main_loop(socket, connection_sender);
    });

    connection_receiver
}
