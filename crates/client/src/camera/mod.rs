use crate::camera::systems::{attach_camera_to_me, setup_camera};
use bevy::prelude::*;

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_system_to_stage(CoreStage::PreUpdate, attach_camera_to_me);
    }
}
