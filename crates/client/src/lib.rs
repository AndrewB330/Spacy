#![allow(dead_code)]

mod camera;
mod input;
mod light;
mod planet;
mod player;
mod sync;

use bevy::log::{Level, LogSettings};
use std::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};

use bevy::prelude::*;
use common::message::{ServerMessageData, UserMessageData};

use crate::camera::CameraPlugin;
use crate::input::InputPlugin;
use crate::light::LightPlugin;
use crate::player::PlayerPlugin;

use crate::server_connection::{ServerConnection, ServerConnectionPlugin, ServerMessages};
use crate::sync::SynchronizationPlugin;

mod server_connection;

pub fn start_client_app(sender: Sender<UserMessageData>, receiver: Receiver<ServerMessageData>) {
    let mut app = App::new();
    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        level: Level::INFO,
        ..default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ServerConnectionPlugin);
    app.insert_resource(ServerConnection {
        sender: Mutex::new(sender),
        receiver: Mutex::new(receiver),
    });

    app.add_plugin(CameraPlugin);
    app.add_plugin(InputPlugin);
    app.add_plugin(LightPlugin);
    app.add_plugin(PlayerPlugin);
    app.add_plugin(SynchronizationPlugin);

    app.run();
}
