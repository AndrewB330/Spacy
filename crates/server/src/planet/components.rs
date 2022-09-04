use crate::sync::{SyncHistory, TransformWrapper};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::planet::PlanetId;
use common::sync::{SyncTarget, SyncTargetId};

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub transform: Transform,
    pub planet_transform: PlanetTransform,
    pub global_transform: GlobalTransform,
    pub sync_transform: SyncHistory,
}

#[derive(Component)]
pub struct Planet {
    pub planet_id: PlanetId,

    pub radius: f32,
    pub mass: f32,
}

/// Deferred planet transform, it only appears on the client.
/// On the server all planets are fixed and do not move.
#[derive(Component, Default)]
pub struct PlanetTransform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl TransformWrapper for PlanetTransform {
    fn get_translation(&self) -> Vec3 {
        self.translation
    }

    fn get_rotation(&self) -> Quat {
        self.rotation
    }
}

impl SyncTarget for Planet {
    fn get_id(&self) -> SyncTargetId {
        SyncTargetId::Planet(self.planet_id)
    }
}

pub fn spawn_planet(commands: &mut Commands, radius: f32, mass: f32) -> Entity {
    commands
        .spawn_bundle(PlanetBundle {
            planet: Planet {
                planet_id: PlanetId::new(),
                radius,
                mass,
            },
            rigid_body: RigidBody::Fixed,
            collider: Collider::ball(radius),
            transform: Transform::default(),
            planet_transform: PlanetTransform::default(),
            global_transform: GlobalTransform::default(),
            sync_transform: SyncHistory::default(),
        })
        .id()
}
