use client::start_client_app;
use common::logging::init_logging;
use network::server::ConnectionEvent;
use server::start_server_app;
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    init_logging();

    let (user_sender, user_receiver) = sync_channel(512 * 32);
    let (server_sender, server_receiver) = sync_channel(512 * 32);

    let (connection_sender, connection_receiver) = sync_channel(512 * 32);

    let server_app_thread = thread::spawn(move || {
        start_server_app(connection_receiver);
    });

    connection_sender
        .send(ConnectionEvent::Connected(0, user_receiver, server_sender))
        .unwrap();

    start_client_app(user_sender, server_receiver);

    server_app_thread.join().unwrap();
}
