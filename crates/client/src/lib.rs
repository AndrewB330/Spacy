#![allow(dead_code)]

mod camera;
mod player;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use bevy::log::{Level, LogSettings};

use bevy::prelude::*;

use crate::player::PlayerPlugin;
use common::message::{ServerMessage, UserMessage};

use crate::server_connection::{ServerConnection, ServerConnectionPlugin, ServerMessages};

mod server_connection;

pub fn start_client_app(sender: Sender<UserMessage>, receiver: Receiver<ServerMessage>) {
    let mut app = App::new();
    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        level: Level::DEBUG,
        ..default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ServerConnectionPlugin);
    app.insert_resource(ServerConnection {
        sender: Mutex::new(sender),
        receiver: Mutex::new(receiver),
    });
    app.add_plugin(PlayerPlugin);
    app.run();
}
