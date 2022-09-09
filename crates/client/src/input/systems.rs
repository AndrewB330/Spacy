use std::f32::consts::PI;
use std::time::Duration;

use bevy::input::mouse::MouseMotion;
use bevy::math::quat;
use bevy::prelude::*;
use common::message::player::PlayerAction;

use common::message::UserMessageData;
use common::planet::ParentPlanet;
use common::player::{Player, PlayerController, PlayerHead};

use crate::input::InputState;
use crate::player::ClientPlayer;
use crate::server_connection::UserMessages;

pub fn send_player_actions(
    query: Query<(
        &Transform,
        Option<&ParentPlanet>,
        &ClientPlayer,
        Option<&PlayerController>,
    )>,
    mut user_messages: UserMessages,
    mut previous_direction: Local<Vec3>,
    mut previous_direction_time: Local<Duration>,
    time: Res<Time>,
) {
    for (transform, maybe_parent_planet, client_player, maybe_player_controller) in query.iter() {
        if !client_player.is_me {
            continue;
        }

        if let Some(player_controller) = maybe_player_controller {
            let mut send_move = false;

            if player_controller.just_jumped {
                send_move = true;
                user_messages.send(UserMessageData::PlayerAction(PlayerAction::Jump));
            }

            let elapsed = time.time_since_startup() - *previous_direction_time;

            if elapsed > Duration::from_millis(200) {
                send_move = true;
            }

            if previous_direction.length() > 0.01
                && player_controller.move_direction.length() > 0.01
                && previous_direction
                    .normalize()
                    .dot(player_controller.move_direction.normalize())
                    < 0.98
            {
                send_move = true;
            }

            if (previous_direction.length() - player_controller.move_direction.length()).abs()
                > 0.05
            {
                send_move = true;
            }

            if send_move {
                user_messages.send(UserMessageData::PlayerAction(PlayerAction::Move(
                    maybe_parent_planet.map(|v| v.parent_planet_id),
                    transform.translation.to_array(),
                    player_controller.move_direction.to_array(),
                )));
                *previous_direction = player_controller.move_direction;
                *previous_direction_time = time.time_since_startup();
            }
        }
    }
}

pub fn process_player_input(
    mut players: Query<(
        &Player,
        &ClientPlayer,
        &Transform,
        Option<&mut PlayerController>,
    )>,
    mut player_heads: Query<(&PlayerHead, &mut Transform), Without<Player>>,
    keys: Res<Input<KeyCode>>,
    mouse_keys: Res<Input<MouseButton>>,
    mut mouse: EventReader<MouseMotion>,
    mut windows: ResMut<Windows>,
    mut input_state: ResMut<InputState>,
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

        let jump = keys.pressed(KeyCode::Space);

        for (player, client_player, transform, maybe_player_controller) in players.iter_mut() {
            if !client_player.is_me {
                continue;
            }

            if let Some(mut player_controller) = maybe_player_controller {
                if jump {
                    player_controller.prepare_to_jump();
                }
                player_controller.head_yaw = input_state.yaw;
                player_controller.head_pitch = input_state.pitch;
                player_controller.move_direction = transform.rotation
                    * Quat::from_axis_angle(Vec3::Y, input_state.yaw)
                    * direction.clamp_length(0.0, 1.0);
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
