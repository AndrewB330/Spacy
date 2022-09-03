#![allow(dead_code)]

mod player;

use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use crate::player::PlayerPlugin;
use bevy::core::CorePlugin;
use bevy::log::{Level, LogPlugin, LogSettings};
use bevy::prelude::*;

use crate::user_connections::{UserConnectionEvent, UserConnectionEvents, UserConnectionsPlugin};

pub mod user_connections;

pub fn start_server_app(connection_events: Receiver<UserConnectionEvent>) {
    let mut app = App::new();
    app.add_plugin(CorePlugin);
    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        level: Level::DEBUG,
        ..default()
    });
    app.add_plugin(LogPlugin);
    app.add_plugin(UserConnectionsPlugin);
    app.insert_resource(UserConnectionEvents {
        receiver: Mutex::new(connection_events),
    });

    app.add_plugin(PlayerPlugin);

    app.set_runner(|mut app| loop {
        app.update();
    });

    app.run();
}
