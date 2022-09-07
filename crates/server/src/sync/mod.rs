use crate::sync::systems::{broadcast_spawn, broadcast_transform};
use crate::{App, CoreStage, Transform};
use bevy::prelude::Plugin;
use common::planet::{ParentPlanet, Planet};
use common::player::Player;

mod components;
mod systems;
mod traits;

use crate::player::UserPlayer;
pub use components::*;

pub struct SyncPlugin;

impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        // Spawn
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            broadcast_spawn::<(
                &Player,
                Option<&UserPlayer>,
                &Transform,
                Option<&ParentPlanet>,
            )>,
        );
        app.add_system_to_stage(CoreStage::PostUpdate, broadcast_spawn::<(&Planet)>);

        // Transform
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            broadcast_transform::<(&Player, &Transform, Option<&ParentPlanet>)>,
        );
        app.add_system_to_stage(CoreStage::PostUpdate, broadcast_transform::<&Planet>);
    }
}
