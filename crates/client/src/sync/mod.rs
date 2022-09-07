use crate::App;
use bevy::prelude::*;

mod components;
mod systems;

use crate::sync::systems::{spawn, update_transform};
use common::planet::Planet;
use common::player::Player;
pub use components::*;

pub struct SyncPlugin;

impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, update_transform::<Player>);
        app.add_system_to_stage(CoreStage::Update, update_transform::<Planet>);

        app.add_system_to_stage(CoreStage::PreUpdate, spawn);
    }
}
