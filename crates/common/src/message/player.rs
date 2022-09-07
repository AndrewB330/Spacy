use crate::planet::PlanetId;
use crate::player::PlayerId;
use bevy::prelude::{Quat, Vec3};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub enum PlayerAction {
    Move(Option<PlanetId>, [f32; 3], [f32; 3]),
    RotateCamera(f32, f32),
    JumpPressed,
    JumpReleased,
    UseTool,
    UseToolSpecial,
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct SpawnPlayer {
    pub player_id: PlayerId,
    pub is_me: bool,
    pub is_user: bool,
    pub parent_planet_id: Option<PlanetId>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}
