use bevy::prelude::*;


#[derive(Component, Debug, Default)]
pub struct SyncTransform {
    pub unsync_translation: Vec3,
    pub unsync_rotation: Quat,
    pub translation: Vec3,
    pub rotation: Quat,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub time_delta: f32,
}
