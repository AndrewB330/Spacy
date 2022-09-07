use crate::planet::systems::spawn_default_planet;
use bevy::prelude::*;

pub struct PlanetPlugin;

mod components;
mod systems;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_default_planet);
    }
}
