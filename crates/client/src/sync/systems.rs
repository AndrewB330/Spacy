use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::player::Player;
use common::message::ServerMessageData;
use common::sync::{SyncLabel, SyncTarget, SyncTargetId};

use crate::sync::{DefaultTarget, SyncHistory};
use crate::ServerMessages;

pub fn update_transform<T: Component + SyncTarget>(
    mut query: Query<(&T, &mut SyncHistory, &mut Transform)>,
    mut server_messages: ServerMessages,
) {
    for message in server_messages.iter() {
        if let ServerMessageData::Transform(target_id, translation, rotation) = &message.data {
            let translation = Vec3::from_array(*translation);
            let rotation = Quat::from_array(*rotation);

            for (obj, mut history, mut transform) in query.iter_mut() {
                let old_time = history
                    .message_time
                    .get(&SyncLabel::Transform)
                    .copied()
                    .unwrap_or_default();
                if obj.get_id() == *target_id && old_time < message.time {
                    transform.translation = translation;
                    transform.rotation = rotation;
                    history
                        .message_time
                        .insert(SyncLabel::Transform, message.time);
                }
            }
        }
    }
}

pub fn spawn_default_with_transform<T: Component + DefaultTarget + SyncTarget>(
    mut commands: Commands,
    mut query: Query<&T>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut server_messages: ServerMessages,
) {
    let mut targets = HashMap::<SyncTargetId, (Vec3, Quat)>::default();
    for message in server_messages.iter() {
        if let ServerMessageData::Transform(target_id, translation, rotation) = &message.data {
            let translation = Vec3::from_array(*translation);
            let rotation = Quat::from_array(*rotation);
            targets.insert(target_id.clone(), (translation, rotation));
        }
    }

    for target in query.iter() {
        targets.remove(&target.get_id());
    }

    for (target_id, (translation, rotation)) in targets.drain() {
        T::spawn_default_with_transform(
            &mut commands,
            target_id,
            translation,
            rotation,
            &mut meshes,
            &mut materials,
        );
    }
}

pub fn update_player_info(
    mut query: Query<(&mut Player, &Handle<StandardMaterial>, &mut SyncHistory)>,
    mut server_messages: ServerMessages,
    // todo: remove
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in server_messages.iter() {
        if let ServerMessageData::PlayerInfo(player_id, player_info) = &message.data {
            for (mut player, handle, mut history) in query.iter_mut() {
                let old_time = history
                    .message_time
                    .get(&SyncLabel::Info)
                    .copied()
                    .unwrap_or_default();
                if player.player_id == *player_id && old_time < message.time {
                    player.is_me = player_info.is_me;
                    player.is_user = player_info.is_user;
                    if player.is_user {
                        materials.get_mut(handle).unwrap().base_color = Color::RED;
                    }
                    if player.is_me {
                        materials.get_mut(handle).unwrap().base_color = Color::GREEN;
                    }
                    history
                        .message_time
                        .insert(SyncLabel::Transform, message.time);
                }
            }
        }
    }
}
