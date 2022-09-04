use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use common::message::{ServerMessage, ServerMessageData, UserMessageData};
use common::player::{PlayerAction, PlayerId, PlayerInfo};
use common::sync::SyncLabel;

use crate::physics::get_bevy_vec;
use crate::physics::levitation::Levitation;
use crate::player::components::{spawn_user_player, Player};
use crate::sync::SyncHistory;
use crate::user_connections::{ServerMessages, UserConnections, UserMessages};

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
                Some(*user_id),
                Vec3::Y * 30.0,
                Quat::IDENTITY,
            );
        }
    }
}

pub fn broadcast_player_info(
    mut players: Query<(&mut Player, &mut SyncHistory)>,
    connection: Res<UserConnections>,
    mut server_messages: ServerMessages,
) {
    for (player, mut history) in players.iter_mut() {
        for user_id in connection.map.keys() {
            let message: ServerMessage = ServerMessageData::PlayerInfo(
                player.player_id,
                PlayerInfo {
                    is_me: Some(*user_id) == player.user_id,
                    is_user: player.user_id.is_some(),
                },
            )
            .into();

            let prev_time = history.get_time(*user_id, SyncLabel::Info);

            if player.is_changed() || prev_time < message.time.before(5000) {
                history.set_time(*user_id, SyncLabel::Info, message.time);
                server_messages.send((*user_id, message.clone()));
            }
        }
    }
}

pub fn move_players(
    mut players: Query<(
        &Player,
        &RapierRigidBodyHandle,
        &Transform,
        &Levitation,
        &ReadMassProperties,
        &mut ExternalImpulse,
    )>,
    time: Res<Time>,
    context: Res<RapierContext>,
) {
    let dt = time.delta_seconds().min(0.03);

    for (player, handle, transform, levitation, mass, mut external_impulse) in players.iter_mut() {
        if player.user_id.is_none() {
            continue;
        }
        if let Some(rigid_body) = context.bodies.get(handle.0) {
            let up = transform.rotation * Vec3::Y;
            let velocity = get_bevy_vec(rigid_body.linvel());
            let velocity_horizontal = velocity - up.dot(velocity) * up;
            let mut velocity_target = transform.rotation
                * Quat::from_axis_angle(Vec3::Y, player.head_yaw)
                * player.move_direction
                * player.max_velocity;

            let velocity_delta = velocity_target - velocity_horizontal;

            if velocity_delta.length() > 1e-6 {
                let acceleration = if levitation.is_falling() {
                    player.max_acceleration * 0.05
                } else {
                    player.max_acceleration
                };

                external_impulse.impulse += velocity_delta.normalize()
                    * (velocity_delta.length().min(acceleration * dt))
                    * mass.0.mass;
            }

            if player.jump_pressed && !levitation.is_falling() {
                let jump_velocity_delta = 6.0 - up.dot(velocity);
                external_impulse.impulse += up * jump_velocity_delta * mass.0.mass;
            }
        }
    }
}

pub fn process_player_actions(mut players: Query<&mut Player>, mut user_messages: UserMessages) {
    for (user_id, message) in user_messages.iter() {
        if let UserMessageData::PlayerAction(action) = &message.data {
            for mut player in players.iter_mut() {
                if player.user_id != Some(*user_id) {
                    continue;
                }

                match action {
                    PlayerAction::Move(direction) => {
                        player.move_direction = Vec3::from_array(*direction);
                    }
                    PlayerAction::JumpPressed => {
                        // todo: remember jump
                        player.jump_pressed = true;
                    }
                    PlayerAction::JumpReleased => {
                        player.jump_pressed = false;
                    }
                    PlayerAction::RotateCamera(pitch, yaw) => {
                        player.head_pith = *pitch;
                        player.head_yaw = *yaw;
                    }
                    // Todo: other actions
                    PlayerAction::UseTool => {}
                    PlayerAction::UseToolSpecial => {}
                }
            }
        }
    }
}
