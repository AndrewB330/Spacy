use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub enum PlayerAction {
    Move([f32; 3]),
    RotateCamera(f32, f32),
    JumpPressed,
    JumpReleased,
    UseTool,
    UseToolSpecial,
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct PlayerInfo {
    pub is_me: bool,
}
