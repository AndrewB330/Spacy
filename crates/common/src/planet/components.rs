use crate::planet::PlanetId;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct Planet {
    pub planet_id: PlanetId,

    pub radius: f32,
    pub mass: f32,

    /// "Real" planet transform. These should be used to compute forces and for rendering.
    /// On client we can just sync this with Transform component.
    pub real_translation: Vec3,
    pub real_rotation: Quat,
}

#[derive(Component)]
pub struct ParentPlanet {
    pub parent_planet_id: PlanetId,
}

impl PlanetBundle {
    pub fn new(
        planet_id: PlanetId,
        mass: f32,
        radius: f32,
        translation: Vec3,
        rotation: Quat,
    ) -> Self {
        PlanetBundle {
            planet: Planet {
                planet_id,
                radius,
                mass,
                real_translation: translation,
                real_rotation: rotation,
            },
            rigid_body: RigidBody::Fixed,
            collider: Collider::ball(radius),
            transform: Transform::from_translation(translation).with_rotation(rotation),
            global_transform: GlobalTransform::default(),
        }
    }
}

pub fn spawn_planet<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    planet_id: PlanetId,
    mass: f32,
    radius: f32,
    translation: Vec3,
    rotation: Quat,
) -> EntityCommands<'w, 's, 'a> {
    info!("Spawned new planet: {:?}", planet_id);
    commands.spawn_bundle(PlanetBundle::new(
        planet_id,
        mass,
        radius,
        translation,
        rotation,
    ))
}
