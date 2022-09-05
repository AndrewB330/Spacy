use bincode::{Decode, Encode};

use crate::player::{PlayerAction, PlayerId, PlayerInfo};
use crate::sync::SyncTargetId;

#[derive(Debug, Clone, Decode, Encode)]
pub enum UserMessageData {
    Ping,
    PlayerAction(PlayerAction),
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum ServerMessageData {
    Pong,
    Transform(SyncTargetId, [f32; 3], [f32; 4], [f32; 3], [f32; 3]),
    PlayerInfo(PlayerId, PlayerInfo),
}
