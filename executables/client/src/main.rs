use client::start_client_app;
use network::client::resilient_tcp_client;
use std::thread;
use std::sync::mpsc::sync_channel;

fn main() {
    let (server_sender, server_receiver) = sync_channel(512);
    let (user_sender, user_receiver) = sync_channel(512);

    let tcp_client_thread = thread::spawn(|| {
        resilient_tcp_client("127.0.0.1", "8000", server_sender, user_receiver);
    });

    start_client_app(user_sender, server_receiver);

    tcp_client_thread.join().unwrap();
}
