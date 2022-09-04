use crate::planet::systems::{planet_gravity, setup_default_planet};
use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_default_planet);
        app.add_system(planet_gravity);
    }
}
