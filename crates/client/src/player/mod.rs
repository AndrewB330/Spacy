use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}
