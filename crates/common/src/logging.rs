use bevy::app::App;
use bevy::log::{Level, LogPlugin, LogSettings};
use bevy::prelude::*;

pub fn init_logging() {
    let mut app = App::new();
    app.insert_resource(LogSettings {
        level: Level::INFO,
        ..default()
    });
    app.add_plugin(LogPlugin);
}
