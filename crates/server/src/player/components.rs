use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::physics::levitation::Levitation;
use common::player::{PlayerId, PLAYER_ABOVE_GROUND, PLAYER_CAPSULE_HEIGHT, PLAYER_CAPSULE_RADIUS};
use common::sync::{SyncTarget, SyncTargetId};
use common::user::UserId;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub read_mass: ReadMassProperties,
    pub scale: ColliderScale,
    pub levitation: Levitation,
}

#[derive(Component)]
pub struct Player {
    pub player_id: PlayerId,
    pub user_id: Option<UserId>,
    pub move_direction: Vec3,
    pub jump_pressed: bool,
    pub max_velocity: f32,
    pub max_acceleration: f32,
    pub head_yaw: f32,
    pub head_pith: f32,
}

impl SyncTarget for Player {
    fn get_id(&self) -> SyncTargetId {
        SyncTargetId::Player(self.player_id)
    }
}

pub fn spawn_user_player(
    commands: &mut Commands,
    player_id: PlayerId,
    user_id: Option<UserId>,
    position: Vec3,
    rotation: Quat,
) {
    info!("Player spawned: {:?}", player_id);
    let above_ground = PLAYER_ABOVE_GROUND + PLAYER_CAPSULE_RADIUS + PLAYER_CAPSULE_HEIGHT * 0.5;

    commands.spawn_bundle(PlayerBundle {
        player: Player {
            player_id,
            user_id: user_id,
            move_direction: Vec3::ZERO,
            jump_pressed: false,
            max_velocity: 3.5,
            max_acceleration: 100.0,
            head_yaw: 0.0,
            head_pith: 0.0,
        },
        collider: Collider::capsule(
            Vec3::new(0.0, -PLAYER_CAPSULE_HEIGHT * 0.5, 0.0),
            Vec3::new(0.0, PLAYER_CAPSULE_HEIGHT * 0.5, 0.0),
            PLAYER_CAPSULE_RADIUS,
        ),
        /*collider: Collider::cuboid(
            PLAYER_CAPSULE_RADIUS,
            PLAYER_CAPSULE_HEIGHT * 0.5 + PLAYER_CAPSULE_RADIUS,
            PLAYER_CAPSULE_RADIUS,
        ),*/
        rigid_body: RigidBody::Dynamic,
        transform: Transform::from_translation(position).with_rotation(rotation),
        global_transform: Default::default(),
        external_force: ExternalForce::default(),
        external_impulse: ExternalImpulse::default(),
        read_mass: ReadMassProperties::default(),
        scale: ColliderScale::Absolute(Vec3::ONE),
        levitation: Levitation::above_ground(above_ground),
    });
}
