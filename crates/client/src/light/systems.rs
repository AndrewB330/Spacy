use bevy::prelude::*;

pub fn setup_light(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
