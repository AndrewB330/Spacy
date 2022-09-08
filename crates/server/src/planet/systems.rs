use crate::planet::components::spawn_server_planet;
use crate::player::spawn_server_empty_player;
use crate::{Commands, MaterialMeshBundle, Quat};
use bevy::math::Vec3;

use rand::random;

pub fn spawn_default_planet(mut commands: Commands) {
    spawn_server_planet(&mut commands, 2000.0, 10.0, Vec3::ZERO, Quat::default());

    for _ in 0..100 {
        spawn_server_empty_player(
            &mut commands,
            Vec3::new(
                random::<f32>() * 32.0 - 16.0,
                random::<f32>() * 32.0 - 16.0,
                random::<f32>() * 16.0 - 8.0 + 20.0,
            ),
        );
    }
}
