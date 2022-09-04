use crate::App;
use bevy::prelude::*;

mod components;
mod systems;

use crate::planet::Planet;
use crate::player::Player;
use crate::sync::systems::{spawn_default_with_transform, update_player_info, update_transform};
pub use components::*;

pub struct SynchronizationPlugin;

impl Plugin for SynchronizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, spawn_default_with_transform::<Player>);
        app.add_system_to_stage(CoreStage::PreUpdate, spawn_default_with_transform::<Planet>);

        app.add_system_to_stage(CoreStage::Update, update_transform::<Player>);
        app.add_system_to_stage(CoreStage::Update, update_transform::<Planet>);

        app.add_system(update_player_info);
    }
}
