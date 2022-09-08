use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use common::message::planet::SpawnPlanet;

use crate::shape::{Cube, UVSphere};
use crate::sync::SyncTransform;
use common::planet::spawn_planet;

pub fn spawn_client_planet<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    info: SpawnPlanet,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> EntityCommands<'w, 's, 'a> {
    let mut ec = spawn_planet(
        commands,
        info.planet_id,
        info.mass,
        info.radius,
        info.translation.into(),
        Quat::from_array(info.rotation),
    );
    ec.insert(
        meshes.add(
            /*UVSphere {
                radius: info.radius,
                sectors: 30,
                stacks: 30,
            }
            .into(),*/
            Cube::new(info.radius * 2.0).into()
        ),
    )
    .insert(materials.add(Color::DARK_GRAY.into()))
    .insert_bundle(VisibilityBundle::default())
    .insert(SyncTransform::default());
    ec
}
