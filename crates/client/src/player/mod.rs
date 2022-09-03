use crate::player::systems::update_player_transforms;
use bevy::prelude::*;

mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_player_transforms);
    }
}
