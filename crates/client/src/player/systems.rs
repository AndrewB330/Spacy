use crate::player::components::{spawn_player, Player};
use crate::ServerMessages;
use bevy::prelude::*;
use common::message::ServerMessage;
use common::player::PlayerUpdate;

pub fn update_player_transforms(
    mut server_messages: ServerMessages,
    mut players: Query<(&Player, &mut Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in server_messages.iter() {
        if let ServerMessage::PlayerUpdate(player_id, is_me, update) = message {
            if let PlayerUpdate::Transform(translation, rotation) = update {
                let mut found = false;

                for (player, mut transform) in players.iter_mut() {
                    if player.player_id == *player_id {
                        transform.translation = Vec3::from_array(*translation);
                        transform.rotation = Quat::from_array(*rotation);
                        found = true;
                        break;
                    }
                }

                if !found {
                    spawn_player(
                        &mut commands,
                        *player_id,
                        Vec3::from_array(*translation),
                        Quat::from_array(*rotation),
                        &mut meshes,
                        &mut materials,
                    );
                }
            }
        }
    }
}
