use crate::light::systems::setup_light;
use crate::App;
use bevy::prelude::*;

mod systems;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_light);
    }
}
