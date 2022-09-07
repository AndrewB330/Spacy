use bevy::ecs::query::{Fetch, WorldQuery, WorldQueryGats};
use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::RapierRigidBodyHandle;
use rand::random;

use common::message::{ServerMessageData, TransformInfo};
use common::physics::get_bevy_vec;

use crate::sync::traits::{SpawnInfoProvider, TransformInfoProvider};
use crate::sync::SyncSpawn;
use crate::user_connections::{ServerMessages, UserConnections};

pub(super) fn broadcast_spawn<Q: WorldQuery>(
    mut objects: Query<(Q, &mut SyncSpawn)>,
    connections: Res<UserConnections>,
    mut server_messages: ServerMessages,
) where
    for<'w> <<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item: SpawnInfoProvider,
{
    for (object, mut sync) in objects.iter_mut() {
        for (k, connection) in connections.map.iter() {
            if !sync.spawn_sent.contains(k) {
                server_messages.send((
                    connection.user_id,
                    ServerMessageData::Spawn(object.get_spawn_info(connection.user_id)),
                ));
                sync.spawn_sent.insert(*k);
            }
        }
    }
}

pub(super) fn broadcast_transform<Q: WorldQuery>(
    mut transforms: Query<(Q, Option<&RapierRigidBodyHandle>)>,
    connections: Res<UserConnections>,
    mut server_messages: ServerMessages,
    context: Res<RapierContext>,
) where
    for<'w> <<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item: TransformInfoProvider,
{
    // todo: send planet velocity somehow??

    // Broadcast all changed transforms!
    for (transform_info, maybe_handle) in transforms.iter_mut() {
        let mut linear_velocity = Vec3::ZERO;
        let mut angular_velocity = Vec3::ZERO;

        if let Some(rigid_body) = maybe_handle.and_then(|handle| context.bodies.get(handle.0)) {
            linear_velocity = get_bevy_vec(rigid_body.linvel());
            angular_velocity = get_bevy_vec(rigid_body.angvel());
        }

        let message = ServerMessageData::Transform(TransformInfo {
            target: transform_info.get_target_id(),
            parent_planet: transform_info.get_parent_planet(),
            translation: transform_info.get_translation().to_array(),
            rotation: transform_info.get_rotation().to_array(),
            linear_velocity: linear_velocity.to_array(),
            angular_velocity: angular_velocity.to_array(),
        });

        for connection in connections.map.values() {
            if random::<u32>() % 10 == 0 {
                // todo:
                server_messages.send((connection.user_id, message.clone()));
            }
        }
    }
}
