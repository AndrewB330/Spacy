use std::thread;
use tokio::sync::mpsc::channel;

use network::server::resilient_tcp_server;
use server::start_server_app;

#[tokio::main]
async fn main() {
    let (connection_sender, connection_receiver) = channel(1024);

    let tcp_server_thread = thread::spawn(|| {
        resilient_tcp_server("8000", connection_sender);
    });

    start_server_app(connection_receiver);

    tcp_server_thread.join().unwrap();
}
