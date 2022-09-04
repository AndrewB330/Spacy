use client::start_client_app;
use common::message::UserId;
use log::info;
use server::start_server_app;
use server::user_connections::{UserConnection, UserConnectionEvent};
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread;

fn main() {
    let (user_sender, user_receiver) = channel();
    let (server_sender, server_receiver) = channel();

    let (connection_sender, connection_receiver) = channel();

    let t = thread::spawn(move || {
        start_server_app(connection_receiver);
    });

    connection_sender
        .send(UserConnectionEvent::Connected(UserConnection {
            user_id: UserId::new(),
            from_user: Mutex::new(user_receiver),
            to_user: Mutex::new(server_sender),
        }))
        .unwrap();

    start_client_app(user_sender, server_receiver);

    info!("END!");
    t.join().unwrap();
}
