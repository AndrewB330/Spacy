use log::info;
use lru::LruCache;
use rand::random;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use common::message::{ServerMessage, UserId, UserMessage};
use common::udp_message::{UdpServerMessage, UdpUserMessage};
use server::user_connections::{UserConnection, UserConnectionEvent};

pub struct UdpConnection {
    pub running: bool,
    pub thread: Option<JoinHandle<()>>,
    pub sender: Sender<UserMessage>,
    pub receiver: Receiver<ServerMessage>,
    pub retry_queue: HashMap<u64, (u32, Instant, UdpServerMessage)>,
    pub ping_time: Instant,
}

fn connection_loop(connection: Arc<Mutex<UdpConnection>>, addr: SocketAddr, socket: UdpSocket) {
    connection.lock().unwrap().running = true;

    info!("Connected: {:?}", addr);

    let mut message_id = 0;

    loop {
        if Instant::now() - connection.lock().unwrap().ping_time > Duration::from_millis(5000) {
            break;
        }

        let recv = connection.lock().unwrap().receiver.try_recv();
        match recv {
            Ok(server_message) => {
                let udp_message = UdpServerMessage::Message(message_id, server_message);
                socket.send_to(&udp_message.to_bytes(), addr).unwrap();

                if udp_message.need_ack() {
                    connection.lock().unwrap().retry_queue.insert(
                        message_id,
                        (udp_message.retries_number(), Instant::now(), udp_message),
                    );
                }

                message_id += 1;
            }
            Err(TryRecvError::Empty) => thread::sleep(Duration::from_millis(1)),
            Err(TryRecvError::Disconnected) => {
                panic!("Unexpected end of the channel!")
            }
        }

        let mut to_delete = vec![];

        for (message_id, (retries, last_retry, udp_message)) in
            connection.lock().unwrap().retry_queue.iter_mut()
        {
            let now = Instant::now();
            if now - *last_retry < udp_message.retry_timeout() {
                break;
            }

            socket.send_to(&udp_message.to_bytes(), addr).unwrap();
            *last_retry = now;

            *retries = retries.saturating_sub(1);

            if *retries == 0 {
                to_delete.push(*message_id);
            }
        }

        for message_id in to_delete.iter() {
            connection.lock().unwrap().retry_queue.remove(&message_id);
        }

        std::hint::spin_loop();
    }

    println!("Disconnected: {:?}", socket);

    connection.lock().unwrap().running = false;
}

fn main_loop(socket: UdpSocket, connection_sender: Sender<UserConnectionEvent>) {
    let mut ping_time_map = HashMap::<SocketAddr, Arc<Mutex<UdpConnection>>>::default();

    let mut buffer = [0u8; 1 << 15];

    let mut received_messages = LruCache::<u64, ()>::new(1024);

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((_, src)) => {
                if random::<u32>() % 5 != 0 {
                    // todo: REMOVE randomly skip one packet!
                    //continue;
                }
                let connection = match ping_time_map.entry(src.clone()) {
                    Entry::Occupied(o) => {
                        // Old connection, refresh ping time
                        let mut lock = o.get().lock().unwrap();
                        lock.ping_time = Instant::now();
                        o.get().clone()
                    }
                    Entry::Vacant(v) => {
                        let user_to_server = channel();
                        let server_to_user = channel();

                        connection_sender
                            .send(UserConnectionEvent::Connected(UserConnection {
                                user_id: UserId::new(),
                                from_user: Mutex::new(user_to_server.1),
                                to_user: Mutex::new(server_to_user.0),
                            }))
                            .unwrap();

                        // New connection
                        v.insert(Arc::new(Mutex::new(UdpConnection {
                            ping_time: Instant::now(),
                            thread: None,
                            running: false,
                            sender: user_to_server.0,
                            receiver: server_to_user.1,
                            retry_queue: HashMap::default(),
                        })))
                        .clone()
                    }
                };

                let connection_clone = connection.clone();

                {
                    let mut lock = connection.lock().unwrap();
                    if !lock.running {
                        if let Some(thread) = lock.thread.take() {
                            thread.join().unwrap();
                        }

                        received_messages.clear();

                        let socket_clone = socket.try_clone().expect("Failed to clone socket");

                        let src_clone = src.clone();
                        lock.thread = Some(thread::spawn(move || {
                            connection_loop(connection_clone, src_clone, socket_clone);
                        }));
                    }

                    let udp_message = UdpUserMessage::from_bytes(&buffer);

                    match &udp_message {
                        UdpUserMessage::Message(message_id, message) => {
                            if udp_message.need_ack() {
                                if !received_messages.contains(message_id) {
                                    received_messages.push(*message_id, ());
                                    socket
                                        .send_to(
                                            &UdpServerMessage::Ack(*message_id).to_bytes(),
                                            src,
                                        )
                                        .unwrap();
                                    lock.sender.send(message.clone()).unwrap();
                                }
                            } else {
                                lock.sender.send(message.clone()).unwrap();
                            }
                        }
                        UdpUserMessage::Ack(message_id) => {
                            lock.retry_queue.remove(message_id);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("couldn't receive a datagram: {}", e);
            }
        }
    }
}

pub fn start_udp_server(port: &str) -> Receiver<UserConnectionEvent> {
    let (connection_sender, connection_receiver) = channel();

    let socket = UdpSocket::bind(format!("192.168.0.16:{}", port)).expect("Could not bind socket");

    thread::spawn(move || {
        main_loop(socket, connection_sender);
    });

    connection_receiver
}
