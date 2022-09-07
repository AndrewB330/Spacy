use crate::message::planet::SpawnPlanet;
use crate::message::player::{PlayerAction, SpawnPlayer};
use crate::planet::PlanetId;

use crate::sync::SyncTargetId;
use bincode::{Decode, Encode};

pub mod planet;
pub mod player;

#[derive(Debug, Clone, Decode, Encode)]
pub enum UserMessageData {
    Ping,
    PlayerAction(PlayerAction),
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum ServerMessageData {
    Pong,
    Transform(TransformInfo),
    Spawn(SpawnInfo),
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct TransformInfo {
    pub target: SyncTargetId,
    pub parent_planet: Option<PlanetId>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub linear_velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum SpawnInfo {
    SpawnPlayer(SpawnPlayer),
    SpawnPlanet(SpawnPlanet),
}
