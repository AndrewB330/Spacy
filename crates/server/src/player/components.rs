use crate::sync::SyncSpawn;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, LockedAxes};
use common::physics::levitation::Levitation;

use common::player::{spawn_player, PlayerId, SpawnPlayerType, PlayerColliderSize};
use common::user::UserId;

#[derive(Component)]
pub struct UserPlayer {
    pub user_id: UserId,
}

pub fn spawn_server_user_player<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    user_id: UserId,
) -> EntityCommands<'w, 's, 'a> {
    let mut ec = spawn_player(
        commands,
        PlayerId::new(),
        Vec3::Z * 30.0,
        Quat::default(),
        SpawnPlayerType::Controlled,
        PlayerColliderSize::Full,
    );
    ec.insert(UserPlayer { user_id })
        .insert(SyncSpawn::default());
    ec
}

pub fn spawn_server_empty_player<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    position: Vec3,
) -> EntityCommands<'w, 's, 'a> {
    let mut ec = spawn_player(
        commands,
        PlayerId::new(),
        position,
        Quat::default(),
        SpawnPlayerType::Dynamic,
        PlayerColliderSize::Full,
    );
    ec.insert(SyncSpawn::default());
    ec.remove::<Levitation>();
    ec.remove::<LockedAxes>();
    ec.insert(Collider::cuboid(0.5, 0.5, 0.5));
    ec
}
