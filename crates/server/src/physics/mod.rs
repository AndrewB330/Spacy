use crate::physics::levitation::levitation_system;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, Real};
use bevy_rapier3d::rapier::prelude::Vector;

pub mod levitation;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(levitation_system);
        app.add_system_to_stage(CoreStage::PreUpdate, clean_external_impulses);
    }
}

fn clean_external_impulses(mut impulses: Query<&mut ExternalImpulse>) {
    for mut impulse in impulses.iter_mut() {
        if impulse.impulse != Vec3::ZERO {
            impulse.impulse = Vec3::ZERO;
        }
        if impulse.torque_impulse != Vec3::ZERO {
            impulse.torque_impulse = Vec3::ZERO;
        }
    }
}

pub fn get_bevy_vec(v: &Vector<Real>) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

pub fn get_rapier_vec(v: &Vec3) -> Vector<Real> {
    Vector::new(v.x, v.y, v.z)
}
