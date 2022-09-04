use crate::player::Player;
use bevy::prelude::*;
use common::player::PlayerHead;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 7.0),
        ..default()
    });
}

pub fn attach_camera_to_me(
    mut commands: Commands,
    players: Query<(Entity, &Player)>,
    player_heads: Query<(Entity, &Parent), With<PlayerHead>>,
    cameras: Query<(Entity, Option<&Parent>), With<Camera3d>>,
) {
    let mut my_head = None;

    for (entity, parent) in player_heads.iter() {
        if let Ok((_, player)) = players.get(parent.get()) {
            if player.is_me {
                my_head = Some(entity);
            }
        }
    }

    if let Some(my_head) = my_head {
        for (entity, maybe_parent) in cameras.iter() {
            if let Some(parent) = maybe_parent {
                if parent.get() != my_head {
                    commands.entity(my_head).add_child(entity);
                }
            } else {
                commands.entity(my_head).add_child(entity);
            }
        }
    }
}
