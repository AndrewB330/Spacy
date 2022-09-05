use bevy::prelude::*;

use common::message::ServerMessageData;
use common::sync::SyncTarget;

use crate::sync::components::TransformWrapper;
use crate::user_connections::{ServerMessages, UserConnections};
use crate::Component;

pub fn broadcast_transform<T: Component + SyncTarget, TW: Component + TransformWrapper>(
    mut transforms: Query<(&T, &mut TW)>,
    connection: Res<UserConnections>,
    mut server_messages: ServerMessages,
) {
    // Broadcast all changed transforms!
    for (target, transform) in transforms.iter_mut() {
        let message = ServerMessageData::Transform(
            target.get_id(),
            transform.get_translation().to_array(),
            transform.get_rotation().to_array(),
        );

        for connection in connection.map.values() {
            if transform.is_changed() {
                server_messages.send((connection.user_id, message.clone()));
            }
        }
    }
}
