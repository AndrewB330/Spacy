use bevy::prelude::*;

use common::message::ServerMessage;
use common::player::{PlayerId, PlayerUpdate};

use crate::player::components::{spawn_user_player, Player};
use crate::user_connections::{ServerMessages, UserConnections};

pub fn spawn_players(
    mut commands: Commands,
    connections: Res<UserConnections>,
    players: Query<&Player>,
) {
    for user_id in connections.map.keys() {
        let mut already_exists = false;
        for player in players.iter() {
            if let Some(user_id_cur) = player.user_id {
                if *user_id == user_id_cur {
                    already_exists = true;
                    break;
                }
            }
        }

        if !already_exists {
            spawn_user_player(
                &mut commands,
                PlayerId::new(),
                *user_id,
                Vec3::ZERO,
                Quat::IDENTITY,
            );
        }
    }
}

pub fn broadcast_player_transforms(
    players: Query<(&Player, &Transform)>,
    connection: Res<UserConnections>,
    mut server_messages: ServerMessages,
) {
    for (player, transform) in players.iter() {
        let player_update = PlayerUpdate::Transform(
            transform.translation.to_array(),
            transform.rotation.to_array(),
        );

        for user_id in connection.map.keys() {
            let message = ServerMessage::PlayerUpdate(
                player.player_id,
                Some(*user_id) == player.user_id,
                player_update.clone(),
            );
            server_messages.send((*user_id, message));
        }
    }
}
