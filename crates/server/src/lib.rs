#![allow(dead_code)]

use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use bevy::core::CorePlugin;
use bevy::prelude::*;

use crate::user_connections::{UserConnectionEvent, UserConnectionEvents, UserConnectionsPlugin};

pub mod user_connections;

pub fn start_server_app(connection_events: Receiver<UserConnectionEvent>) {
    let mut app = App::new();
    app.add_plugin(CorePlugin);
    app.add_plugin(UserConnectionsPlugin);
    app.insert_resource(UserConnectionEvents { receiver: Mutex::new(connection_events) });

    app.set_runner(|mut app| {
        loop {
            app.update();
        }
    });

    app.run();
}
