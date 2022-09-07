use crate::sync::SyncSpawn;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use common::planet::{spawn_planet, PlanetId};

pub fn spawn_server_planet<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    mass: f32,
    radius: f32,
    translation: Vec3,
    rotation: Quat,
) -> EntityCommands<'w, 's, 'a> {
    let mut ec = spawn_planet(
        commands,
        PlanetId::new(),
        mass,
        radius,
        translation,
        rotation,
    );

    ec.insert(SyncSpawn::default());

    ec
}
