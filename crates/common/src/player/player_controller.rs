use bevy::prelude::*;
use std::time::Duration;

const PLAYER_DEFAULT_MAX_VELOCITY: f32 = 3.5;
const PLAYER_JUMP_VELOCITY: f32 = 6.0;
const PLAYER_TOTAL_MAX_VELOCITY: f32 = 20.0;
const PLAYER_MAX_ACCELERATION: f32 = 40.0;

const IS_JUMPING_MAX_DURATION: Duration = Duration::from_millis(50);
const JUMP_DELAY: Duration = Duration::from_millis(400);
const ERROR_CORRECTION_MAX_DURATION: Duration = Duration::from_millis(4000);

#[derive(Component, Debug, Clone)]
pub struct PlayerController {
    /// Direction in which player will try to move.
    /// If length is less that 1.0, the max speed will be scaled accordingly.
    pub move_direction: Vec3,

    /// Error of position on server relative to user, to sync client-server movements.
    pub is_jumping: bool,
    pub just_jumped: bool,
    pub is_jumping_duration: Duration,
    pub since_last_jump_duration: Duration,

    pub error: Option<Vec3>,
    pub error_duration: Duration,

    pub head_yaw: f32,
    pub head_pitch: f32,

    pub jump_velocity: f32,
    pub max_velocity: f32,
    pub max_acceleration: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        PlayerController {
            move_direction: Vec3::ZERO,
            is_jumping: false,
            just_jumped: false,
            is_jumping_duration: Duration::default(),
            since_last_jump_duration: Duration::from_secs(100),
            error: None,
            error_duration: Duration::default(),
            head_yaw: 0.0,
            head_pitch: 0.0,
            jump_velocity: PLAYER_JUMP_VELOCITY,
            max_velocity: PLAYER_DEFAULT_MAX_VELOCITY,
            max_acceleration: PLAYER_MAX_ACCELERATION,
        }
    }
}

fn desired_error_correction_time(error: Vec3) -> f32 {
    const HI: f32 = 0.5;
    const LO: f32 = 0.25;
    let len = error.length();
    if len < 0.1 {
        HI
    } else if len < 0.2 {
        HI - (len - 0.1) / 0.1 * (HI - LO)
    } else {
        (len - 0.2) * 2.0 - LO
    }
}

impl PlayerController {
    pub fn prepare_to_jump(&mut self) {
        self.is_jumping = true;
        self.is_jumping_duration = Duration::default();
    }

    pub fn set_error(&mut self, delta: Vec3) {
        self.error = Some(delta);
        self.error_duration = Duration::default();
    }

    pub fn tick(&mut self, time: &Time) {
        self.is_jumping_duration += time.delta();
        self.error_duration += time.delta();
        self.since_last_jump_duration += time.delta();

        self.just_jumped = false;

        if self.is_jumping_duration > IS_JUMPING_MAX_DURATION {
            self.is_jumping_duration = Duration::default();
            self.is_jumping = false;
        }

        if self.error_duration > ERROR_CORRECTION_MAX_DURATION {
            self.error_duration = Duration::default();
            self.error = None;
        }
    }

    pub fn can_jump(&self) -> bool {
        self.is_jumping && self.since_last_jump_duration > JUMP_DELAY
    }

    pub fn jump(&mut self) {
        self.is_jumping = false;
        self.just_jumped = true;
        self.since_last_jump_duration = Duration::default();
    }

    pub fn get_velocity_delta(&self, up: Vec3, current_velocity: Vec3) -> Vec3 {
        let current_velocity_projected = current_velocity - up.dot(current_velocity) * up;

        let mut move_direction_projected = self.move_direction - up.dot(self.move_direction) * up;
        if move_direction_projected.length() > 1.0 {
            move_direction_projected /= move_direction_projected.length();
        }

        let mut velocity_target = move_direction_projected * self.max_velocity;

        if let Some(error) = self.error {
            let error_projected = error - up.dot(error) * up;
            velocity_target +=
                error_projected / desired_error_correction_time(error_projected) * 1.0;

            //velocity_target = velocity_target.clamp_length(0.0, self.max_velocity * 1.2);
        }

        velocity_target - current_velocity_projected
    }

    pub fn get_jump_delta(&self, up: Vec3, current_velocity: Vec3) -> Vec3 {
        (self.jump_velocity - up.dot(current_velocity)) * up
    }
}
