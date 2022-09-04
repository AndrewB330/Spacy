use crate::planet::{Planet, PlanetTransform};
use crate::player::Player;
use crate::sync::systems::broadcast_transform;
use crate::{App, CoreStage, Transform};
use bevy::prelude::Plugin;

mod components;
mod systems;

pub use components::*;

pub struct SyncPlugin;

impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            broadcast_transform::<Player, Transform>,
        );
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            broadcast_transform::<Planet, PlanetTransform>,
        );
    }
}
