use bevy::prelude::*;

use common::planet::PlanetId;
use common::sync::{SyncTarget, SyncTargetId};

use crate::shape::UVSphere;

#[derive(Bundle)]
pub struct PlanetBundle {
    pub player: Planet,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Component)]
pub struct Planet {
    pub planet_id: PlanetId,
}

impl SyncTarget for Planet {
    fn get_id(&self) -> SyncTargetId {
        SyncTargetId::Planet(self.planet_id)
    }
}

pub fn spawn_planet(
    commands: &mut Commands,
    planet_id: PlanetId,
    position: Vec3,
    rotation: Quat,
    radius: f32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    info!("Planet spawned: {:?}", planet_id);

    commands
        .spawn_bundle(PlanetBundle {
            player: Planet { planet_id },
            mesh: meshes.add(
                UVSphere {
                    radius,
                    sectors: 30,
                    stacks: 30,
                }
                .into(),
            ),
            material: materials.add(Color::DARK_GRAY.into()),
            transform: Transform::from_translation(position).with_rotation(rotation),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        })
        .id()
}
