use bevy::prelude::*;
use bevy::utils::{Entry, HashMap};

use common::message::{ServerMessage, ServerMessageData};
use common::sync::{SyncLabel, SyncTarget};

use crate::sync::components::{SyncHistory, TransformWrapper};
use crate::user_connections::{ServerMessages, UserConnections};
use crate::Component;

pub fn broadcast_transform<T: Component + SyncTarget, TW: Component + TransformWrapper>(
    mut transforms: Query<(&T, &mut TW, &mut SyncHistory)>,
    connection: Res<UserConnections>,
    mut server_messages: ServerMessages,
) {
    // Broadcast all changed transforms!
    for (target, transform, mut history) in transforms.iter_mut() {
        let message: ServerMessage = ServerMessageData::Transform(
            target.get_id(),
            transform.get_translation().to_array(),
            transform.get_rotation().to_array(),
        )
        .into();

        for user_id in connection.map.keys() {
            let prev_time = history.get_time(*user_id, SyncLabel::Transform);
            if transform.is_changed() || prev_time < message.time.before(1000) {
                history.set_time(*user_id, SyncLabel::Transform, message.time);
                server_messages.send((*user_id, message.clone()));
            }
        }
    }
}
