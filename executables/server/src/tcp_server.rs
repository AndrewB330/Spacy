use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;

use common::user::UserId;
use server::user_connections::{UserConnection, UserConnectionEvent};

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
