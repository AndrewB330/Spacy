use bevy::ecs::system::EntityCommands;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use common::message::player::SpawnPlayer;
use common::physics::collision_groups::FAKE_PLAYER_COLLISION_GROUPS;

use crate::shape::{Capsule, Cube};
use crate::sync::SyncTransform;
use common::player::{PlayerColliderSize, spawn_player, SpawnPlayerType};

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
        // Client-controlled player

        let mut ec = spawn_player(
            commands,
            info.player_id,
            info.translation.into(),
            Quat::from_array(info.rotation),
            SpawnPlayerType::Controlled,
            PlayerColliderSize::Mini,
        );

        ec.insert(FAKE_PLAYER_COLLISION_GROUPS);

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
        .insert_bundle(VisibilityBundle::default());
    }


    let mut ec = spawn_player(
        commands,
        info.player_id,
        info.translation.into(),
        Quat::from_array(info.rotation),
        if info.is_me {
            SpawnPlayerType::FakeKinematic
        } else {
            SpawnPlayerType::Kinematic
        },
        PlayerColliderSize::Full,
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
        .insert(materials.add(if !info.is_me {Color::WHITE} else {Color::rgba(1.0, 1.0, 1.0, 0.2)}.into()))
        .insert(ClientPlayer {
            is_me: false,
            is_user: info.is_user,
        })
        .insert_bundle(VisibilityBundle::default());

    if info.is_me {
        ec.insert(NotShadowCaster);
    } else {
        if !info.is_user {
            ec.insert(meshes.add(Cube::new(1.0).into()));
            ec.insert(Collider::cuboid(0.5, 0.5, 0.5));
        }
    }

    // todo
    // if !info.is_me
    {
        ec.insert(SyncTransform::default());
    }
    ec
}
