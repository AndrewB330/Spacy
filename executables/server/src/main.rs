use crate::udp_server::start_udp_server;
use server::start_server_app;
use crate::tcp_server::start_tcp_server;

mod udp_server;
mod tcp_server;

fn main() {
    let connection_receiver = start_tcp_server("8000");
    start_server_app(connection_receiver);
}
