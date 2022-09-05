#![allow(dead_code)]

mod physics;
mod planet;
mod player;
mod sync;

use bevy::asset::AssetPlugin;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use crate::physics::PhysicsPlugin;
use crate::planet::PlanetPlugin;
use crate::player::PlayerPlugin;
use crate::sync::SyncPlugin;
use bevy::core::CorePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::{Level, LogPlugin, LogSettings};
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy_rapier3d::prelude::*;
use common::message::{ServerMessageData, UserMessageData};
use network::server::ConnectionEvent;

use crate::user_connections::{UserConnectionEvents, UserConnectionsPlugin};

pub mod user_connections;

pub fn start_server_app(
    connection_events: Receiver<ConnectionEvent<UserMessageData, ServerMessageData>>,
) {
    let mut app = App::new();
    app.add_plugin(CorePlugin);
    app.add_plugin(TimePlugin);
    app.add_plugin(TransformPlugin);
    app.add_plugin(HierarchyPlugin);
    app.add_plugin(DiagnosticsPlugin);
    app.add_plugin(AssetPlugin);

    app.add_asset::<Mesh>();
    app.add_asset::<Scene>();

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.insert_resource(RapierConfiguration {
        gravity: Vec3::ZERO,
        timestep_mode: TimestepMode::Variable {
            max_dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps: 5,
        },
        ..default()
    });

    #[cfg(feature = "logging")]
    app.insert_resource(LogSettings {
        level: Level::INFO,
        ..default()
    });

    #[cfg(feature = "logging")]
    app.add_plugin(LogPlugin);

    app.add_plugin(UserConnectionsPlugin);
    app.insert_resource(UserConnectionEvents {
        receiver: Mutex::new(connection_events),
    });

    app.add_plugin(PlayerPlugin);
    app.add_plugin(PlanetPlugin);
    app.add_plugin(SyncPlugin);
    app.add_plugin(PhysicsPlugin);

    app.set_runner(|mut app| loop {
        app.update();
    });

    app.run();
}
