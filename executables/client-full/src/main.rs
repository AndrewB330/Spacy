use std::sync::mpsc::channel;
use client::start_client_app;
use server::start_server_app;
use std::thread;
use network::server::ConnectionEvent;

fn main() {
    let (user_sender, user_receiver) = channel();
    let (server_sender, server_receiver) = channel();

    let (connection_sender, connection_receiver) = channel();

    let server_app_thread = thread::spawn(move || {
        start_server_app(connection_receiver);
    });

    connection_sender
        .blocking_send(ConnectionEvent::Connected(
            0,
            user_receiver,
            server_sender,
        ))
        .unwrap();

    start_client_app(user_sender, server_receiver);

    server_app_thread.join().unwrap();
}
