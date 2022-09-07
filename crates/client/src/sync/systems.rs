use crate::planet::spawn_client_planet;
use crate::player::spawn_client_player;
use bevy::prelude::*;
use common::message::{ServerMessageData, SpawnInfo};
use common::sync::SyncTarget;

use crate::sync::SyncTransform;
use crate::ServerMessages;

pub fn spawn(
    mut commands: Commands,
    mut server_messages: ServerMessages,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in server_messages.iter() {
        if let ServerMessageData::Spawn(spawn_info) = message {
            match spawn_info {
                SpawnInfo::SpawnPlayer(spawn_player) => {
                    spawn_client_player(
                        &mut commands,
                        spawn_player.clone(),
                        &mut meshes,
                        &mut materials,
                    );
                }
                SpawnInfo::SpawnPlanet(spawn_planet) => {
                    spawn_client_planet(
                        &mut commands,
                        spawn_planet.clone(),
                        &mut meshes,
                        &mut materials,
                    );
                }
            }
        }
    }
}

pub fn update_transform<T: Component + SyncTarget>(
    mut query: Query<(&T, &mut Transform, &mut SyncTransform)>,
    mut server_messages: ServerMessages,
    time: Res<Time>,
) {
    for (_, mut transform, mut sync_transform) in query.iter_mut() {
        sync_transform.time_delta += time.delta_seconds();

        sync_transform.unsync_translation =
            sync_transform.unsync_translation * 0.9 + sync_transform.translation * 0.1;
        sync_transform.unsync_rotation =
            (sync_transform.unsync_rotation * 0.9 + sync_transform.rotation * 0.1).normalize();

        transform.translation = sync_transform.unsync_translation
            + sync_transform.linear_velocity * sync_transform.time_delta;
        transform.rotation =
            Quat::from_scaled_axis(sync_transform.angular_velocity * sync_transform.time_delta)
                * sync_transform.unsync_rotation;
    }

    for message in server_messages.iter() {
        if let ServerMessageData::Transform(transform_info) = &message {
            // todo: apply parent planet
            let translation = Vec3::from_array(transform_info.translation);
            let rotation = Quat::from_array(transform_info.rotation);
            let linear_velocity = Vec3::from_array(transform_info.linear_velocity);
            let angular_velocity = Vec3::from_array(transform_info.angular_velocity);

            for (obj, transform, mut sync_transform) in query.iter_mut() {
                if obj.get_id() == transform_info.target {
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
