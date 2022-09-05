use client::start_client_app;
use network::client::resilient_tcp_client;
use std::thread;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let (server_sender, server_receiver) = channel(1024);
    let (user_sender, user_receiver) = channel(1024);

    let tcp_client_thread = thread::spawn(|| {
        resilient_tcp_client("127.0.0.1", "8000", server_sender, user_receiver);
    });

    start_client_app(user_sender, server_receiver);

    tcp_client_thread.join().unwrap();
}
