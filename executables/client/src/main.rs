use crate::tcp_client::start_tcp_client;
use client::start_client_app;

mod tcp_client;

fn main() {
    let (sender, receiver) = start_tcp_client("34.118.6.39", "8000");
    start_client_app(sender, receiver);
}
