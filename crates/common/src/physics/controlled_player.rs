use crate::physics::get_bevy_vec;
use crate::physics::levitation::Levitation;
use crate::player::PlayerController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn update_controlled_players(
    mut players: Query<(
        &mut PlayerController,
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

    for (mut player_controller, handle, transform, levitation, mass, mut external_impulse) in
        players.iter_mut()
    {
        player_controller.tick(&time);

        if let Some(rigid_body) = context.bodies.get(handle.0) {
            let up = transform.rotation * Vec3::Y;
            let velocity = get_bevy_vec(rigid_body.linvel());

            let velocity_delta = player_controller.get_velocity_delta(up, velocity);

            if velocity_delta.length() > 1e-6 {
                external_impulse.impulse += velocity_delta
                    .clamp_length(0.0, player_controller.max_acceleration * dt)
                    * mass.0.mass;
            }

            if player_controller.can_jump() && !levitation.is_falling() {
                player_controller.jump();
                external_impulse.impulse +=
                    player_controller.get_jump_delta(up, velocity) * mass.0.mass;
            }
        }
    }
}
