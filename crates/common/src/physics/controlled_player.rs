use crate::physics::get_bevy_vec;
use crate::physics::levitation::Levitation;
use crate::player::PlayerController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn update_controlled_players(
    mut players: Query<(
        &PlayerController,
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

    for (player_controller, handle, transform, levitation, mass, mut external_impulse) in
        players.iter_mut()
    {
        if let Some(rigid_body) = context.bodies.get(handle.0) {
            let up = transform.rotation * Vec3::Y;
            let velocity = get_bevy_vec(rigid_body.linvel());
            let velocity_horizontal = velocity - up.dot(velocity) * up;

            let mut move_direction =
                player_controller.move_direction - up.dot(player_controller.move_direction) * up;
            if move_direction.length() > 1.0 {
                move_direction /= move_direction.length();
            }

            let mut velocity_target = move_direction * player_controller.max_velocity;

            if let Some(error) = player_controller.error {
                let error = error - up.dot(error) * up;

                let error_len = error.length();

                let reach_time = if error_len < 0.1 {
                    1.0
                } else if error_len < 0.2 {
                    1.0 - (error_len - 0.1) / 0.1 * 0.5
                } else {
                    error_len * 2.0 - 0.2 + 0.5
                };

                velocity_target += error / reach_time * 2.0;
            }

            let velocity_delta = velocity_target - velocity_horizontal;

            if velocity_delta.length() > 1e-6 {
                external_impulse.impulse += velocity_delta.normalize()
                    * (velocity_delta
                        .length()
                        .min(player_controller.max_acceleration * dt))
                    * mass.0.mass;
            }

            if player_controller.jump_pressed && !levitation.is_falling() {
                let jump_velocity_delta = 6.0 - up.dot(velocity);
                external_impulse.impulse += up * jump_velocity_delta * mass.0.mass;
            }
        }
    }
}
