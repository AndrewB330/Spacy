use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::message::player::SpawnPlayer;

use crate::shape::Capsule;
use crate::sync::SyncTransform;
use common::player::{spawn_player, SpawnPlayerType};

#[derive(Component)]
pub struct ClientPlayer {
    pub is_me: bool,
    pub is_user: bool,
}

pub fn spawn_client_player<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    info: SpawnPlayer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> EntityCommands<'w, 's, 'a> {
    const LOCAL_PLAYER_BIT: u32 = 1 << 0;
    const PLAYER_BIT: u32 = 1 << 1;

    if info.is_me {
        let mut ec = spawn_player(
            commands,
            info.player_id,
            info.translation.into(),
            Quat::from_array(info.rotation),
            SpawnPlayerType::Controlled,
        );

        ec.insert(
            meshes.add(
                Capsule {
                    radius: 0.4,
                    depth: 0.5,
                    ..default()
                }
                .into(),
            ),
        )
        .insert(materials.add(Color::GREEN.into()))
        .insert(ClientPlayer {
            is_me: true,
            is_user: info.is_user,
        })
        .insert_bundle(VisibilityBundle::default())
        .insert(CollisionGroups::new(
            LOCAL_PLAYER_BIT,
            !(LOCAL_PLAYER_BIT | PLAYER_BIT),
        ));
        ec
    } else {
        let mut ec = spawn_player(
            commands,
            info.player_id,
            info.translation.into(),
            Quat::from_array(info.rotation),
            if info.is_me {
                // todo
                SpawnPlayerType::Kinematic
            } else {
                SpawnPlayerType::Kinematic
            },
        );

        ec.insert(
            meshes.add(
                Capsule {
                    radius: 0.4,
                    depth: 0.5,
                    ..default()
                }
                    .into(),
            ),
        )
            .insert(materials.add(Color::WHITE.into()))
            .insert(ClientPlayer {
                is_me: false,
                is_user: info.is_user,
            })
            .insert_bundle(VisibilityBundle::default())
            .insert(CollisionGroups::new(
                PLAYER_BIT,
                !(LOCAL_PLAYER_BIT | PLAYER_BIT),
            ));

        // todo
        // if !info.is_me
        {
            ec.insert(SyncTransform::default());
        }
        ec
    }
}
