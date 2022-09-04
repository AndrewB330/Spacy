use bevy::prelude::*;

use common::player::{PlayerHeadBundle, PlayerId, PLAYER_CAPSULE_HEIGHT, PLAYER_CAPSULE_RADIUS};
use common::sync::{SyncTarget, SyncTargetId};

use crate::shape::Capsule;
use crate::sync::SyncHistory;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub transform_sync: SyncHistory,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Component)]
pub struct Player {
    pub player_id: PlayerId,
    pub is_me: bool,
    pub is_user: bool,
}

impl SyncTarget for Player {
    fn get_id(&self) -> SyncTargetId {
        SyncTargetId::Player(self.player_id)
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    player_id: PlayerId,
    position: Vec3,
    rotation: Quat,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    info!("Player spawned: {:?}", player_id);

    commands
        .spawn_bundle(PlayerBundle {
            player: Player {
                player_id,
                is_me: false,
                is_user: false,
            },
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
            transform_sync: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        })
        .with_children(|builder| {
            builder.spawn_bundle(PlayerHeadBundle::new(player_id));
        })
        .id()
}
