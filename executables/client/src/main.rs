use client::start_client_app;
use network::client::resilient_tcp_client;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (server_sender, server_receiver) = channel();
    let (user_sender, user_receiver) = channel();

    let tcp_client_thread = thread::spawn(|| {
        resilient_tcp_client("127.0.0.1", "8000", server_sender, user_receiver);
    });

    start_client_app(user_sender, server_receiver);

    tcp_client_thread.join().unwrap();
}
