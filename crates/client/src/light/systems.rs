use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;

pub fn setup_light(mut commands: Commands, mut shadow_map: ResMut<DirectionalLightShadowMap>) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::ONE, 0.7)),
        ..default()
    });
    shadow_map.size = 4096;
}
