use crate::physics::levitation::Levitation;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::PlayerId;

const PLAYER_CAPSULE_HEIGHT: f32 = 0.5;
const PLAYER_CAPSULE_RADIUS: f32 = 0.4;
const PLAYER_ABOVE_GROUND: f32 = 0.2;
const PLAYER_HEAD_HEIGHT: f32 = 0.3;

const PLAYER_DEFAULT_MAX_VELOCITY: f32 = 3.5;
const PLAYER_TOTAL_MAX_VELOCITY: f32 = 20.0;
const PLAYER_MAX_ACCELERATION: f32 = 100.0;

#[derive(Bundle)]
pub struct PlayerHeadBundle {
    pub player_head: PlayerHead,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle)]
pub struct PhysicsBasedPlayerBehaviorBundle {
    pub rigid_body: RigidBody,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub read_mass: ReadMassProperties,
    pub scale: ColliderScale,
    pub levitation: Levitation,
}

#[derive(Component)]
pub struct PlayerHead {
    pub player_id: PlayerId,
    pub player_entity: Entity,
}

#[derive(Component)]
pub struct Player {
    pub player_id: PlayerId,
    pub head_entity: Entity,
}

#[derive(Component)]
pub struct PlayerController {
    pub move_direction: Vec3,
    /// Error of position on server relative to user, to sync client-server movements.
    pub error: Option<Vec3>,
    pub jump_pressed: bool,
    pub jump_pressed_elapsed_time: f32,
    pub head_yaw: f32,
    pub head_pitch: f32,
    pub max_velocity: f32,
    pub max_acceleration: f32,
}

impl PlayerHeadBundle {
    pub fn new(player_id: PlayerId, player_entity: Entity) -> Self {
        PlayerHeadBundle {
            player_head: PlayerHead {
                player_id,
                player_entity,
            },
            transform: Transform::from_translation(Vec3::new(0.0, PLAYER_HEAD_HEIGHT, 0.0)),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

impl PlayerBundle {
    pub fn new(player_id: PlayerId, head_entity: Entity) -> Self {
        PlayerBundle {
            player: Player {
                player_id,
                head_entity,
            },
            collider: Collider::capsule_y(PLAYER_CAPSULE_HEIGHT * 0.5, PLAYER_CAPSULE_RADIUS),
            transform: Transform::from_translation(Vec3::new(0.0, PLAYER_HEAD_HEIGHT, 0.0)),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform = self.transform.with_translation(translation);
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.transform = self.transform.with_rotation(rotation);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

impl PhysicsBasedPlayerBehaviorBundle {
    pub fn new() -> Self {
        let above_ground =
            PLAYER_ABOVE_GROUND + PLAYER_CAPSULE_RADIUS + PLAYER_CAPSULE_HEIGHT * 0.5;
        PhysicsBasedPlayerBehaviorBundle {
            rigid_body: RigidBody::Dynamic,
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            read_mass: ReadMassProperties::default(),
            scale: ColliderScale::Absolute(Vec3::ONE),
            levitation: Levitation::above_ground(above_ground),
        }
    }
}

pub enum SpawnPlayerType {
    Kinematic,
    Dynamic,
    Controlled,
}

pub enum SpawnPlayerColliderType {
    Standard,
    Mini
}

pub fn spawn_player<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    player_id: PlayerId,
    position: Vec3,
    rotation: Quat,
    spawn_player_type: SpawnPlayerType,
) -> EntityCommands<'w, 's, 'a> {
    info!("Spawned new player: {:?}", player_id);

    let player_entity = commands.spawn().id();
    let head_entity = commands.spawn().id();

    commands.entity(player_entity).insert_bundle(
        PlayerBundle::new(player_id, head_entity)
            .with_translation(position)
            .with_rotation(rotation),
    );
    commands
        .entity(head_entity)
        .insert_bundle(PlayerHeadBundle::new(player_id, head_entity));

    commands.entity(player_entity).add_child(head_entity);

    match spawn_player_type {
        SpawnPlayerType::Kinematic => {
            commands.entity(player_entity).insert_bundle((
                RigidBody::KinematicPositionBased,
                ColliderScale::Absolute(Vec3::ONE),
            ));
        }
        SpawnPlayerType::Dynamic => {
            commands
                .entity(player_entity)
                .insert_bundle(PhysicsBasedPlayerBehaviorBundle::new());
        }
        SpawnPlayerType::Controlled => {
            commands
                .entity(player_entity)
                .insert_bundle(PhysicsBasedPlayerBehaviorBundle::new())
                .insert(PlayerController {
                    move_direction: Vec3::ZERO,
                    error: None,
                    jump_pressed: false,
                    jump_pressed_elapsed_time: 0.0,
                    head_yaw: 0.0,
                    head_pitch: 0.0,
                    max_velocity: PLAYER_DEFAULT_MAX_VELOCITY,
                    max_acceleration: PLAYER_MAX_ACCELERATION,
                });
        }
    }

    commands.entity(player_entity)
}
