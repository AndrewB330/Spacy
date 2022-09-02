mod app;

use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::utils::hashbrown::hash_map::Entry;
use bevy::utils::HashMap;
use crate::server::app::{start_server_app, UserConnection};

pub struct UdpConnection {
    pub running: bool,
    pub thread: Option<JoinHandle<()>>,
    pub ping_time: Instant,
}

fn connection_loop(connection: Arc<Mutex<UdpConnection>>, addr: SocketAddr) {
    connection.lock().unwrap().running = true;

    loop {
        if Instant::now() - connection.lock().unwrap().ping_time > Duration::from_millis(5000) {
            break;
        }
    }

    connection.lock().unwrap().running = false;
}

fn main_loop(socket: UdpSocket, connection_sender: Sender<UserConnection>) {
    let mut ping_time_map = HashMap::<SocketAddr, Arc<Mutex<UdpConnection>>>::default();

    loop {
        let mut buffer = [0u8; 1024];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buffer) {
            Ok((_, src)) => {
                let connection = match ping_time_map.entry(src.clone()) {
                    Entry::Occupied(o) => {
                        // Old connection, refresh ping time
                        o.get().lock().unwrap().ping_time = Instant::now();
                        o.get().clone()
                    }
                    Entry::Vacant(v) => {
                        // New connection
                        v.insert(Arc::new(Mutex::new(UdpConnection {
                            ping_time: Instant::now(),
                            thread: None,
                            running: false,
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


                        lock.thread = Some(thread::spawn(move || {
                            connection_loop(connection_clone, src);
                        }));
                    }
                }
            }
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}

pub fn start_server(listen_port: &str) {
    let (connection_sender, connection_receiver) = channel();

    let socket = UdpSocket::bind("0.0.0.0:8888")
        .expect("Could not bind socket");

    let t = thread::spawn(move || {
        main_loop(socket, connection_sender);
    });

    start_server_app(connection_receiver);

    t.join().unwrap();

}
