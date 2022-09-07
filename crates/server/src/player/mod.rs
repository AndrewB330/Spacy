use crate::player::systems::{process_user_players_actions, spawn_user_players};
use crate::App;
use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_user_players);
        app.add_system(process_user_players_actions);
    }
}
