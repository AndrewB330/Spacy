use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::player::Player;
use common::message::ServerMessageData;
use common::sync::{SyncTarget, SyncTargetId};

use crate::sync::{DefaultTarget, SyncTransform};
use crate::ServerMessages;

pub fn update_transform<T: Component + SyncTarget>(
    mut query: Query<(&T, &mut Transform, &mut SyncTransform)>,
    mut server_messages: ServerMessages,
    time: Res<Time>,
) {
    for (_, mut transform, mut sync_transform) in query.iter_mut() {
        sync_transform.time_delta += time.delta_seconds();

        sync_transform.unsync_translation = (sync_transform.unsync_translation * 0.9 + sync_transform.translation * 0.1);
        sync_transform.unsync_rotation = (sync_transform.unsync_rotation * 0.9 + sync_transform.rotation * 0.1).normalize();

        transform.translation = sync_transform.unsync_translation + sync_transform.linear_velocity * sync_transform.time_delta;
        transform.rotation = Quat::from_scaled_axis(sync_transform.angular_velocity * sync_transform.time_delta) * sync_transform.unsync_rotation;
    }

    for message in server_messages.iter() {
        if let ServerMessageData::Transform(
            target_id,
            translation,
            rotation,
            translation_velocity,
            rotation_velocity,
        ) = &message
        {
            let translation = Vec3::from_array(*translation);
            let rotation = Quat::from_array(*rotation);
            let linear_velocity = Vec3::from_array(*translation_velocity);
            let angular_velocity = Vec3::from_array(*rotation_velocity);

            for (obj, mut transform, mut sync_transform) in query.iter_mut() {
                if obj.get_id() == *target_id {
                    sync_transform.unsync_translation = transform.translation;
                    sync_transform.unsync_rotation = transform.rotation;
                    sync_transform.translation = translation;
                    sync_transform.rotation = rotation;
                    sync_transform.linear_velocity = linear_velocity;
                    sync_transform.angular_velocity = angular_velocity;
                    sync_transform.time_delta = 0.0;
                }
            }
        }
    }
}

pub fn spawn_default_with_transform<T: Component + DefaultTarget + SyncTarget>(
    mut commands: Commands,
    query: Query<&T>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut server_messages: ServerMessages,
) {
    let mut targets = HashMap::<SyncTargetId, (Vec3, Quat)>::default();
    for message in server_messages.iter() {
        if let ServerMessageData::Transform(target_id, translation, rotation, _, _) = &message {
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
    mut query: Query<(&mut Player, &Handle<StandardMaterial>)>,
    mut server_messages: ServerMessages,
    // todo: remove
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in server_messages.iter() {
        if let ServerMessageData::PlayerInfo(player_id, player_info) = &message {
            for (mut player, handle) in query.iter_mut() {
                if player.player_id == *player_id {
                    player.is_me = player_info.is_me;
                    player.is_user = player_info.is_user;
                    if player.is_user {
                        materials.get_mut(handle).unwrap().base_color = Color::RED;
                    }
                    if player.is_me {
                        materials.get_mut(handle).unwrap().base_color = Color::GREEN;
                    }
                }
            }
        }
    }
}
