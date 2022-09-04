use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use lru::LruCache;

use common::message::{ServerMessage, UserMessage};
use common::udp_message::{UdpServerMessage, UdpUserMessage};

pub fn start_udp_client(
    host: &str,
    server_port: &str,
) -> (Sender<UserMessage>, Receiver<ServerMessage>) {
    let (user_sender, user_receiver) = channel::<UserMessage>();
    let (server_sender, server_receiver) = channel::<ServerMessage>();

    let addrs = vec![
        SocketAddr::from_str("192.168.0.16:8080").unwrap(),
        SocketAddr::from_str("192.168.0.16:8081").unwrap(),
        SocketAddr::from_str("192.168.0.16:8082").unwrap(),
        SocketAddr::from_str("192.168.0.16:8083").unwrap(),
        SocketAddr::from_str("192.168.0.16:8084").unwrap(),
    ];

    let socket = UdpSocket::bind(addrs.as_slice()).expect("Could not bind client socket");

    socket
        .connect(format!("{}:{}", host, server_port))
        .expect("Could not connect to server");

    let socket_clone = socket.try_clone().unwrap();

    let retry_map = Arc::new(Mutex::new(
        HashMap::<u64, (u32, Instant, UdpUserMessage)>::default(),
    ));
    let retry_map_clone = retry_map.clone();

    let mut received_messages = LruCache::<u64, ()>::new(1024);

    thread::spawn(move || {
        let mut bytes = [0u8; 1 << 15];
        loop {
            match socket_clone.recv(&mut bytes) {
                Ok(_) => {
                    let udp_message = UdpServerMessage::from_bytes(&bytes);

                    match &udp_message {
                        UdpServerMessage::Message(message_id, message) => {
                            if udp_message.need_ack() {
                                if !received_messages.contains(message_id) {
                                    received_messages.push(*message_id, ());
                                }
                                socket_clone
                                    .send(&UdpUserMessage::Ack(*message_id).to_bytes())
                                    .unwrap();
                                server_sender.send(message.clone()).unwrap();
                            } else {
                                server_sender.send(message.clone()).unwrap();
                            }
                        }
                        UdpServerMessage::Ack(message_id) => {
                            retry_map_clone.lock().unwrap().remove(message_id);
                        }
                    }
                }
                Err(e) => {
                    received_messages.clear();
                    println!("Server disconnected! {}", e);
                }
            }
        }
    });

    thread::spawn(move || {
        let mut message_id = 0;
        loop {
            match user_receiver.try_recv() {
                Ok(message) => {
                    let udp_message = UdpUserMessage::Message(message_id, message);

                    socket.send(&udp_message.to_bytes()).unwrap();

                    if udp_message.need_ack() {
                        retry_map.lock().unwrap().insert(
                            message_id,
                            (udp_message.retries_number(), Instant::now(), udp_message),
                        );
                    }

                    message_id += 1;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    panic!("Unexpected end of the channel!")
                }
            }

            let mut to_delete = vec![];

            for (message_id, (retries, last_retry, udp_message)) in
                retry_map.lock().unwrap().iter_mut()
            {
                let now = Instant::now();
                if now - *last_retry < udp_message.retry_timeout() {
                    continue;
                }

                socket.send(&udp_message.to_bytes()).unwrap();
                *last_retry = now;

                *retries = retries.saturating_sub(1);

                if *retries == 0 {
                    to_delete.push(*message_id);
                }
            }

            for message_id in to_delete.iter() {
                retry_map.lock().unwrap().remove(&message_id);
            }
        }
    });

    (user_sender, server_receiver)
}
