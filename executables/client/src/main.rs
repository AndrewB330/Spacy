use client::start_client_app;

use crate::udp_client::start_udp_client;

mod udp_client;

fn main() {
    let (sender, receiver) = start_udp_client("8000");
    start_client_app(sender, receiver);
}
