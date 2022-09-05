use crate::tcp_server::start_tcp_server;
use server::start_server_app;

mod tcp_server;

fn main() {
    let connection_receiver = start_tcp_server("8000");
    start_server_app(connection_receiver);
}
