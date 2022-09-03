#![allow(dead_code)]

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use bevy::prelude::*;

use common::message::{ServerMessage, UserMessage};

use crate::server_connection::{ServerConnection, ServerConnectionPlugin};

mod server_connection;

pub fn start_client_app(sender: Sender<UserMessage>, receiver: Receiver<ServerMessage>) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ServerConnectionPlugin);
    app.insert_resource(ServerConnection {
        sender: Mutex::new(sender),
        receiver: Mutex::new(receiver),
    });
    app.run();
}
