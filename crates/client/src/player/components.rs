use bevy::prelude::*;

use common::player::{PlayerHeadBundle, PlayerId, PLAYER_CAPSULE_HEIGHT, PLAYER_CAPSULE_RADIUS};

use crate::shape::Capsule;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Component)]
pub struct Player {
    pub player_id: PlayerId,
}

pub fn spawn_player(
    commands: &mut Commands,
    player_id: PlayerId,
    position: Vec3,
    rotation: Quat,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {

    debug!("Player spawned: {:?}", player_id);

    commands
        .spawn_bundle(PlayerBundle {
            player: Player { player_id },
            mesh: meshes.add(
                Capsule {
                    radius: PLAYER_CAPSULE_RADIUS,
                    depth: PLAYER_CAPSULE_HEIGHT,
                    ..default()
                }
                .into(),
            ),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(position).with_rotation(rotation),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        })
        .with_children(|builder| {
            builder.spawn_bundle(PlayerHeadBundle::new(player_id));
        })
        .id()
}
