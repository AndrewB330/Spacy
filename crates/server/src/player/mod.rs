use crate::player::systems::{
    broadcast_player_info, move_players, process_player_actions, spawn_players,
};
use crate::App;
use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_players);
        app.add_system(broadcast_player_info);
        app.add_system(move_players);
        app.add_system(process_player_actions);
    }
}
