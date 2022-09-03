use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::message::UserId;

use common::player::{PlayerId, PLAYER_CAPSULE_HEIGHT, PLAYER_CAPSULE_RADIUS};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct Player {
    pub player_id: PlayerId,
    pub user_id: Option<UserId>,
}

pub fn spawn_user_player(
    commands: &mut Commands,
    player_id: PlayerId,
    user_id: UserId,
    position: Vec3,
    rotation: Quat,
) {
    debug!("Player spawned: {:?}", player_id);
    commands.spawn_bundle(PlayerBundle {
        player: Player {
            player_id,
            user_id: Some(user_id),
        },
        collider: Collider::capsule(
            Vec3::new(0.0, -PLAYER_CAPSULE_HEIGHT * 0.5, 0.0),
            Vec3::new(0.0, PLAYER_CAPSULE_HEIGHT * 0.5, 0.0),
            PLAYER_CAPSULE_RADIUS,
        ),
        transform: Transform::from_translation(position).with_rotation(rotation),
        global_transform: Default::default(),
    });
}
