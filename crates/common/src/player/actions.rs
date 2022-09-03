use bevy::prelude::Vec3;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub enum PlayerAction {
    Move([f32; 3]),
    Jump,
    UseTool,
    UseToolSpecial,
}
