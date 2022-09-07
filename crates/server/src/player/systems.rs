use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::{spawn_server_user_player, UserPlayer};
use common::message::player::{PlayerAction, SpawnPlayer};
use common::message::{ServerMessageData, UserMessageData};
use common::physics::get_bevy_vec;
use common::physics::levitation::Levitation;
use common::planet::ParentPlanet;
use common::player::{spawn_player, Player, PlayerController, PlayerId};
use common::user::UserId;

use crate::user_connections::{ServerMessages, UserConnections, UserMessages};

pub fn spawn_user_players(
    mut commands: Commands,
    connections: Res<UserConnections>,
    players: Query<&UserPlayer>,
) {
    for connection in connections.map.values() {
        let mut already_exists = false;
        for player in players.iter() {
            if connection.user_id == player.user_id {
                already_exists = true;
                break;
            }
        }

        if !already_exists {
            spawn_server_user_player(&mut commands, connection.user_id);
        }
    }
}

pub fn process_user_players_actions(
    mut players: Query<(
        &mut UserPlayer,
        &mut PlayerController,
        &Transform,
        Option<&ParentPlanet>,
    )>,
    mut user_messages: UserMessages,
) {
    for (user_id, message) in user_messages.iter() {
        if let UserMessageData::PlayerAction(action) = &message {
            for (mut user_player, mut player_controller, transform, maybe_parent_planet) in
                players.iter_mut()
            {
                if user_player.user_id != *user_id {
                    continue;
                }

                match action {
                    PlayerAction::Move(parent_planet, position, direction) => {
                        player_controller.move_direction = Vec3::from(*direction);
                        player_controller.error =
                            if maybe_parent_planet.map(|v| v.parent_planet_id) == *parent_planet {
                                Some(Vec3::from(*position) - transform.translation)
                            } else {
                                None
                            }
                    }
                    PlayerAction::JumpPressed => {
                        // todo: remember jump
                        player_controller.jump_pressed = true;
                    }
                    PlayerAction::JumpReleased => {
                        player_controller.jump_pressed = false;
                    }
                    PlayerAction::RotateCamera(pitch, yaw) => {
                        player_controller.head_pitch = *pitch;
                        player_controller.head_yaw = *yaw;
                    }
                    // Todo: other actions
                    PlayerAction::UseTool => {}
                    PlayerAction::UseToolSpecial => {}
                }
            }
        }
    }
}
