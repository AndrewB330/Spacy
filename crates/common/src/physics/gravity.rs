use crate::planet::Planet;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalForce, ReadMassProperties};

pub fn planet_gravity_system(
    planets: Query<&Planet>,
    mut objects: Query<(
        &ReadMassProperties,
        &Transform,
        &mut ExternalForce,
        Option<&Parent>,
    )>,
) {
    for (mass, transform, mut external_force, parent) in objects.iter_mut() {
        let global_transform = if let Some(parent) = parent {
            if let Ok(planet) = planets.get(parent.get()) {
                Transform::default()
                    .with_translation(planet.real_translation)
                    .with_rotation(planet.real_rotation)
                    .mul_transform(*transform)
            } else {
                *transform
            }
        } else {
            *transform
        };

        let mut total_force = Vec3::ZERO;

        for planet in planets.iter() {
            let distance = (global_transform.translation - planet.real_translation).length();

            let mut gravity = 1.0 * planet.mass * mass.0.mass / distance.powf(2.);

            if distance < planet.radius {
                gravity *= distance / planet.radius;
            }

            let force =
                (planet.real_translation - global_transform.translation).normalize() * gravity;

            total_force += force;
        }

        if ((external_force.force - total_force) / mass.0.mass).length() > 0.05 {
            external_force.force = total_force;
        }
    }
}
