use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use common::message::{ServerMessage, UserId, UserMessage};
use server::user_connections::{UserConnection, UserConnectionEvent};

pub struct UdpConnection {
    pub running: bool,
    pub thread: Option<JoinHandle<()>>,
    pub sender: Sender<UserMessage>,
    pub receiver: Receiver<ServerMessage>,
    pub ping_time: Instant,
}

fn connection_loop(connection: Arc<Mutex<UdpConnection>>, addr: SocketAddr, socket: UdpSocket) {
    connection.lock().unwrap().running = true;

    println!("New connection: {:?}", socket);

    let message_id = 0;

    loop {
        if Instant::now() - connection.lock().unwrap().ping_time > Duration::from_millis(5000) {
            break;
        }

        let recv = connection.lock().unwrap().receiver.try_recv();
        match recv {
            Ok(server_message) => {
                socket.send_to(&server_message.to_bytes(), addr).unwrap();
            }
            Err(TryRecvError::Empty) => {}
            _ => { panic!("Closed :(") }
        }
        std::hint::spin_loop();
    }

    println!("Disconnected: {:?}", socket);

    connection.lock().unwrap().running = false;
}

fn main_loop(socket: UdpSocket, connection_sender: Sender<UserConnectionEvent>) {
    let mut ping_time_map = HashMap::<SocketAddr, Arc<Mutex<UdpConnection>>>::default();

    let mut buffer = [0u8; 1 << 15];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((_, src)) => {
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

                        connection_sender.send(UserConnectionEvent::Connected(UserConnection {
                            user_id: UserId::new(),
                            from_user: Mutex::new(user_to_server.1),
                            to_user: Mutex::new(server_to_user.0),
                        })).unwrap();

                        // New connection
                        v.insert(Arc::new(Mutex::new(UdpConnection {
                            ping_time: Instant::now(),
                            thread: None,
                            running: false,
                            sender: user_to_server.0,
                            receiver: server_to_user.1,
                        }))).clone()
                    }
                };

                let connection_clone = connection.clone();

                {
                    let mut lock = connection.lock().unwrap();
                    if !lock.running {
                        if let Some(thread) = lock.thread.take() {
                            thread.join().unwrap();
                        }

                        let socket_clone = socket.try_clone().expect("Failed to clone socket");


                        lock.thread = Some(thread::spawn(move || {
                            connection_loop(connection_clone, src, socket_clone);
                        }));
                    }

                    lock.sender.send(UserMessage::from_bytes(&buffer)).unwrap();
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

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
        .expect("Could not bind socket");

    thread::spawn(move || {
        main_loop(socket, connection_sender);
    });

    connection_receiver
}
