use crate::input::systems::process_player_input;
use bevy::prelude::*;

mod systems;

pub struct InputPlugin;

#[derive(Default)]
pub struct InputState {
    pitch: f32,
    yaw: f32,
    active: bool,
    direction: Vec3,
    direction_time: f32,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>();
        app.add_system(process_player_input);
    }
}
