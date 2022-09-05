use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::RapierRigidBodyHandle;
use rand::random;

use common::message::ServerMessageData;
use common::sync::SyncTarget;

use crate::physics::get_bevy_vec;
use crate::sync::components::TransformWrapper;
use crate::user_connections::{ServerMessages, UserConnections};
use crate::Component;

pub fn broadcast_transform<T: Component + SyncTarget, TW: Component + TransformWrapper>(
    mut transforms: Query<(&T, &mut TW, Option<&RapierRigidBodyHandle>)>,
    connection: Res<UserConnections>,
    mut server_messages: ServerMessages,
    context: Res<RapierContext>,
) {
    // todo: send planet velocity somehow??

    // Broadcast all changed transforms!
    for (target, transform, maybe_handle) in transforms.iter_mut() {
        let mut linear_velocity = Vec3::ZERO;
        let mut angular_velocity = Vec3::ZERO;

        if let Some(rigid_body) = maybe_handle.and_then(|handle| context.bodies.get(handle.0)) {
            linear_velocity = get_bevy_vec(rigid_body.linvel());
            angular_velocity = get_bevy_vec(rigid_body.angvel());
        }

        let message = ServerMessageData::Transform(
            target.get_id(),
            transform.get_translation().to_array(),
            transform.get_rotation().to_array(),
            linear_velocity.to_array(),
            angular_velocity.to_array(),
        );

        for connection in connection.map.values() {
            if random::<u32>() % 600 == 0 {
                // todo:
                server_messages.send((connection.user_id, message.clone()));
            }
        }
    }
}
