use crate::{App, Plugin};

mod components;
mod systems;

pub use components::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
