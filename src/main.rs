#![allow(dead_code)]

use crate::server::start_server;

mod server;
mod client;
mod common;

fn main() {
    start_server("8000");
}
