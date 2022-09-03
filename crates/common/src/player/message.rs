use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub enum PlayerAction {
    Move([f32; 3]),
    Jump,
    UseTool,
    UseToolSpecial,
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum PlayerUpdate {
    Transform([f32; 3], [f32; 4]),
}
