use crate::planet::components::{spawn_planet, Planet};
use crate::planet::PlanetTransform;
use crate::player::spawn_user_player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalForce, ReadMassProperties};
use common::player::PlayerId;
use rand::random;

pub fn planet_gravity(
    planets: Query<(&Planet, &PlanetTransform)>,
    mut objects: Query<(
        &ReadMassProperties,
        &Transform,
        &mut ExternalForce,
        Option<&Parent>,
    )>,
) {
    for (mass, transform, mut external_force, parent) in objects.iter_mut() {
        let global_transform = if let Some(parent) = parent {
            if let Ok((_, my_planet_transform)) = planets.get(parent.get()) {
                Transform::default()
                    .with_translation(my_planet_transform.translation)
                    .with_rotation(my_planet_transform.rotation)
                    .mul_transform(*transform)
            } else {
                *transform
            }
        } else {
            *transform
        };

        let mut total_force = Vec3::ZERO;

        for (planet, planet_transform) in planets.iter() {
            let distance = (global_transform.translation - planet_transform.translation).length();

            let mut gravity = 1.0 * planet.mass * mass.0.mass / distance.powf(2.);

            if distance < planet.radius {
                gravity *= distance / planet.radius;
            }

            let force =
                (planet_transform.translation - global_transform.translation).normalize() * gravity;

            total_force += force;
        }

        if ((external_force.force - total_force) / mass.0.mass).length() > 0.05 {
            external_force.force = total_force;
        }
    }
}

pub fn setup_default_planet(mut commands: Commands) {
    spawn_planet(&mut commands, 10.0, 1500.0);

    for _ in 0..10 {
        spawn_user_player(
            &mut commands,
            PlayerId::new(),
            None,
            Vec3::new(
                random::<f32>() * 32.0 - 16.0,
                random::<f32>() * 5.0 + 30.0,
                random::<f32>() * 32.0 - 16.0,
            ),
            Quat::default(),
        );
    }
}
