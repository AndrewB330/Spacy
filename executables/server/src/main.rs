use std::sync::mpsc::sync_channel;
use std::thread;

use network::server::resilient_tcp_server;
use server::start_server_app;

fn main() {
    let (connection_sender, connection_receiver) = sync_channel(512 * 32);

    let tcp_server_thread = thread::spawn(|| {
        resilient_tcp_server("8000", connection_sender);
    });

    start_server_app(connection_receiver);

    tcp_server_thread.join().unwrap();
}
