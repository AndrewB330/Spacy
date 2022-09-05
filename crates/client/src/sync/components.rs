use crate::planet::{spawn_planet, Planet};
use crate::player::{spawn_player, Player};
use bevy::prelude::*;
use common::sync::SyncTargetId;

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

pub trait DefaultTarget {
    fn spawn_default_with_transform(
        commands: &mut Commands,
        target_id: SyncTargetId,
        translation: Vec3,
        rotation: Quat,
        // todo: get rid of these params here
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    );
}

impl DefaultTarget for Player {
    fn spawn_default_with_transform(
        commands: &mut Commands,
        target_id: SyncTargetId,
        translation: Vec3,
        rotation: Quat,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        if let SyncTargetId::Player(player_id) = target_id {
            spawn_player(
                commands,
                player_id,
                translation,
                rotation,
                meshes,
                materials,
            );
        }
    }
}

impl DefaultTarget for Planet {
    fn spawn_default_with_transform(
        commands: &mut Commands,
        target_id: SyncTargetId,
        translation: Vec3,
        rotation: Quat,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        if let SyncTargetId::Planet(planet_id) = target_id {
            spawn_planet(
                commands,
                planet_id,
                translation,
                rotation,
                10.0, // Default planet radius ??
                meshes,
                materials,
            );
        }
    }
}
