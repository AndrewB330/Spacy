use crate::input::systems::{process_player_input, send_player_actions};
use bevy::prelude::*;

mod systems;

pub struct InputPlugin;

#[derive(Default)]
pub struct InputState {
    pitch: f32,
    yaw: f32,
    active: bool,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>();
        app.add_system(process_player_input);
        app.add_system(send_player_actions);
    }
}
