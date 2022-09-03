use crate::player::systems::{broadcast_player_transforms, spawn_players};
use crate::{App, CoreStage, RunCriteriaDescriptorCoercion};
use bevy::prelude::Plugin;
use bevy::transform::TransformSystem;

mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_players);
        app.add_system_to_stage(CoreStage::PostUpdate, broadcast_player_transforms);
    }
}
