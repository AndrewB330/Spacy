use crate::physics::{get_bevy_vec, get_rapier_vec};
use bevy::prelude::{Component, Entity, Quat, Query, Res, ResMut, Time, Transform, Vec3};
use bevy_rapier3d::prelude::{
    ExternalForce, InteractionGroups, QueryFilter, RapierContext, RapierRigidBodyHandle,
};
use crate::physics::collision_groups::{PLAYER_COLLISION_BIT, PLAYER_COLLISION_FILTER};

/// Needs RigidBody and GravityReceiver components to work.
#[derive(Component, Debug, Clone)]
pub struct Levitation {
    pub height_above_ground: f32,
    pub force: f32,
    pub damping: f32,
    pub preserve_orientation: bool,
    pub interaction_groups: InteractionGroups,
    falling: bool,
}

impl Levitation {
    pub fn is_falling(&self) -> bool {
        self.falling
    }

    pub fn above_ground(height: f32) -> Self {
        Self {
            height_above_ground: height,
            ..Self::default()
        }
    }

    pub fn with_interaction_groups(mut self, ig: InteractionGroups) -> Self {
        self.interaction_groups = ig;
        self
    }
}

impl Default for Levitation {
    fn default() -> Self {
        Self {
            height_above_ground: 1.0,
            force: 160.0,
            damping: 20.0,
            preserve_orientation: true,
            interaction_groups: InteractionGroups::default(),
            falling: false,
        }
    }
}

pub fn levitation_system(
    mut context: ResMut<RapierContext>,
    mut entities: Query<(
        Entity,
        &RapierRigidBodyHandle,
        &mut Transform,
        &mut Levitation,
        &ExternalForce,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds().min(0.03);

    for (entity, handle, mut transform, mut levitation, gravity_receiver) in entities.iter_mut() {
        let mut ray_direction = gravity_receiver.force;
        if ray_direction.length() < 1e-8 {
            ray_direction = transform.rotation * Vec3::NEG_Y;
        } else {
            ray_direction = ray_direction.normalize();
        }

        let mut impulse = Vec3::ZERO;
        let mut torque = Vec3::ZERO;

        levitation.falling = true;

        if let Some(rigid_body) = context.bodies.get(handle.0) {
            if let Some((_, hit_dist)) = context.cast_ray(
                transform.translation,
                ray_direction,
                levitation.height_above_ground * 1.5,
                true,
                QueryFilter::new()
                    .exclude_collider(entity)
                    .groups(levitation.interaction_groups),
            ) {
                let linear_velocity = get_bevy_vec(rigid_body.linvel()).dot(ray_direction);

                if hit_dist < levitation.height_above_ground {
                    let difference = hit_dist - levitation.height_above_ground;
                    impulse +=
                        ray_direction * difference * levitation.force * rigid_body.mass() * dt;
                    impulse -= ray_direction
                        * linear_velocity
                        * levitation.damping
                        * rigid_body.mass()
                        * dt;
                }

                if hit_dist < levitation.height_above_ground * 1.01 {
                    levitation.falling = false;
                }
            }

            if levitation.preserve_orientation {
                /*let delta_rotation =
                    Quat::from_rotation_arc(transform.rotation * Vec3::NEG_Y, ray_direction);
                let (axis, angle) = delta_rotation.to_axis_angle();
                let angular_velocity = get_bevy_vec(&rigid_body.angvel());

                // Todo: account for rotational mass?
                torque += axis * angle * levitation.force * 0.1 * dt;
                torque -= angular_velocity * levitation.damping * 0.1 * dt;*/
                transform.rotation = Quat::from_rotation_arc(transform.rotation * Vec3::NEG_Y, ray_direction) *  transform.rotation;
            }
        }

        if let Some(rigid_body) = context.bodies.get_mut(handle.0) {
            rigid_body.apply_impulse(get_rapier_vec(&impulse), true);
            rigid_body.apply_torque_impulse(get_rapier_vec(&torque), true);
        }
    }
}
