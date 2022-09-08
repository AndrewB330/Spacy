use crate::physics::controlled_player::update_controlled_players;
use crate::physics::gravity::planet_gravity_system;
use crate::physics::levitation::levitation_system;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::Vector;

mod controlled_player;
mod gravity;
pub mod collision_groups;
pub mod levitation;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 5,
            },
            ..default()
        });

        app.add_system(levitation_system);
        app.add_system(planet_gravity_system);
        app.add_system(update_controlled_players);
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
