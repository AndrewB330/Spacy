#![allow(dead_code)]

mod camera;
mod input;
mod light;
mod planet;
mod player;
mod sync;

use bevy::log::LogPlugin;
#[cfg(debug_assertions)]
use bevy::log::{Level, LogSettings};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Mutex;

use bevy::prelude::*;
use common::message::{ServerMessageData, UserMessageData};
use common::physics::PhysicsPlugin;

use crate::camera::CameraPlugin;
use crate::input::InputPlugin;
use crate::light::LightPlugin;

use crate::server_connection::{ServerConnection, ServerConnectionPlugin, ServerMessages};
use crate::sync::SyncPlugin;

mod server_connection;

pub fn start_client_app(
    sender: SyncSender<UserMessageData>,
    receiver: Receiver<ServerMessageData>,
) {
    let mut app = App::new();
    app.add_plugins_with(DefaultPlugins, |p| p.disable::<LogPlugin>());
    app.add_plugin(ServerConnectionPlugin);
    app.insert_resource(ServerConnection {
        sender: Mutex::new(sender),
        receiver: Mutex::new(receiver),
    });

    app.add_plugin(PhysicsPlugin);
    app.add_plugin(CameraPlugin);
    app.add_plugin(InputPlugin);
    app.add_plugin(LightPlugin);
    app.add_plugin(SyncPlugin);

    app.run();
}
