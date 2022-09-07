use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use common::message::player::PlayerAction;

use common::message::UserMessageData;
use common::planet::ParentPlanet;
use common::player::{Player, PlayerController, PlayerHead};

use crate::input::InputState;
use crate::player::ClientPlayer;
use crate::server_connection::UserMessages;

pub fn process_player_input(
    mut players: Query<(
        &Player,
        &Transform,
        Option<&ParentPlanet>,
        &ClientPlayer,
        Option<&mut PlayerController>,
    )>,
    mut player_heads: Query<(&PlayerHead, &mut Transform), Without<Player>>,
    keys: Res<Input<KeyCode>>,
    mouse_keys: Res<Input<MouseButton>>,
    mut mouse: EventReader<MouseMotion>,
    mut windows: ResMut<Windows>,
    mut input_state: ResMut<InputState>,
    mut server_messages: UserMessages,
    time: Res<Time>,
) {
    if let Some(window) = windows.get_primary_mut() {
        // Check that player input is active.
        if !input_state.active {
            if mouse_keys.pressed(MouseButton::Left) {
                input_state.active = true;
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);
                window.set_cursor_position(Vec2::new(window.width(), window.height()) / 2.0);
            }
            return;
        }

        if keys.pressed(KeyCode::E) {
            input_state.active = false;
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
            window.set_cursor_position(Vec2::new(window.width(), window.height()) / 2.0);
            return;
        }

        for e in mouse.iter() {
            input_state.pitch -= e.delta.y / 500.0;
            input_state.yaw -= e.delta.x / 500.0;
            input_state.yaw %= 2.0 * PI;
            input_state.pitch = input_state.pitch.clamp(-PI / 2.0, PI / 2.0);
        }

        server_messages.send(
            UserMessageData::PlayerAction(PlayerAction::RotateCamera(
                input_state.pitch,
                input_state.yaw,
            ))
            .into(),
        );

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction -= Vec3::Z;
        }
        if keys.pressed(KeyCode::S) {
            direction += Vec3::Z;
        }
        if keys.pressed(KeyCode::A) {
            direction -= Vec3::X;
        }
        if keys.pressed(KeyCode::D) {
            direction += Vec3::X;
        }

        for (player, transform, maybe_parent_planet, client_player, maybe_player_controller) in
            players.iter_mut()
        {
            if !client_player.is_me {
                continue;
            }

            if let Some(mut player_controller) = maybe_player_controller {
                let direction = transform.rotation
                    * Quat::from_axis_angle(Vec3::Y, input_state.yaw)
                    * direction;

                let mut send_move = false;

                if (direction - input_state.direction).length() < 0.01
                    && input_state.direction_time < 0.5
                    || input_state.direction_time < 0.05
                {
                    input_state.direction_time += time.delta_seconds();
                } else {
                    input_state.direction = direction;
                    input_state.direction_time = 0.0;
                    send_move = true;
                }

                if keys.just_pressed(KeyCode::Space) {
                    player_controller.jump_pressed = true;
                    server_messages
                        .send(UserMessageData::PlayerAction(PlayerAction::JumpPressed).into());
                    send_move = true;
                } else if keys.just_released(KeyCode::Space) {
                    player_controller.jump_pressed = false;
                    server_messages
                        .send(UserMessageData::PlayerAction(PlayerAction::JumpReleased).into());
                    send_move = true;
                }

                player_controller.head_yaw = input_state.yaw;
                player_controller.head_pitch = input_state.pitch;
                player_controller.move_direction = input_state.direction;

                if send_move {
                    server_messages.send(
                        UserMessageData::PlayerAction(PlayerAction::Move(
                            maybe_parent_planet.map(|v| v.parent_planet_id),
                            transform.translation.to_array(),
                            player_controller.move_direction.to_array(),
                        ))
                        .into(),
                    );
                }
            }

            // Set head rotation
            if let Ok((player_head, mut transform)) = player_heads.get_mut(player.head_entity) {
                if player_head.player_id != player.player_id {
                    continue;
                }

                transform.rotation = Quat::from_axis_angle(Vec3::Y, input_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, input_state.pitch);
            }
        }
    }
}
