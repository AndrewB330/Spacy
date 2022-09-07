use crate::sync::SyncSpawn;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use common::player::{spawn_player, PlayerId, SpawnPlayerType};
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
    );
    ec.insert(SyncSpawn::default());
    ec
}
