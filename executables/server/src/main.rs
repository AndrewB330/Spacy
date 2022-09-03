use crate::udp_server::start_udp_server;
use server::start_server_app;

mod udp_server;

fn main() {
    let connection_receiver = start_udp_server("8000");
    start_server_app(connection_receiver);
}
