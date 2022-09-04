use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use common::message::UserMessageData;
use common::player::{PlayerAction, PlayerHead};

use crate::input::InputState;
use crate::player::Player;
use crate::server_connection::UserMessages;

pub fn process_player_input(
    players: Query<&Player>,
    mut player_heads: Query<(&PlayerHead, &mut Transform)>,
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

        if keys.just_pressed(KeyCode::Space) {
            server_messages.send(UserMessageData::PlayerAction(PlayerAction::JumpPressed).into());
        } else if keys.just_released(KeyCode::Space) {
            server_messages.send(UserMessageData::PlayerAction(PlayerAction::JumpReleased).into());
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

        if (direction - input_state.direction).length() < 0.01 && direction.length() < 0.01 {
            input_state.direction_time += time.delta_seconds();
        } else {
            input_state.direction = direction;
            input_state.direction_time = 0.0;
        }

        if input_state.direction_time < 1.0 {
            server_messages.send(
                UserMessageData::PlayerAction(PlayerAction::Move(input_state.direction.to_array()))
                    .into(),
            );
        }

        // Set my camera rotation, all the server head rotation will be ignored for me.
        for player in players.iter() {
            if !player.is_me {
                continue;
            }

            for (player_head, mut transform) in player_heads.iter_mut() {
                if player_head.player_id != player.player_id {
                    continue;
                }

                transform.rotation = Quat::from_axis_angle(Vec3::Y, input_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, input_state.pitch);
            }
        }
    }
}
