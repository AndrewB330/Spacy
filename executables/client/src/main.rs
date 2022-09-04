use client::start_client_app;
use crate::tcp_client::start_tcp_client;

use crate::udp_client::start_udp_client;

mod udp_client;
mod tcp_client;

fn main() {
    let (sender, receiver) =
        start_tcp_client("34.171.146.212", "8000");
    start_client_app(sender, receiver);
}
