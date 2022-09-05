use bincode::config::{standard, Configuration};
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
    Transform(SyncTargetId, [f32; 3], [f32; 4]),
    PlayerInfo(PlayerId, PlayerInfo),
}

const BINCODE_CONFIG: Configuration = standard();

impl From<Vec<u8>> for UserMessageData {
    fn from(bytes: Vec<u8>) -> Self {
        // todo: assert read all
        bincode::decode_from_slice(&bytes, BINCODE_CONFIG).unwrap().0
    }
}

impl Into<Vec<u8>> for UserMessageData {
    fn into(self) -> Vec<u8> {
        bincode::encode_to_vec(self, BINCODE_CONFIG).unwrap()
    }
}

impl From<&[u8]> for ServerMessageData {
    fn from(bytes: &[u8]) -> Self {
        // todo: assert read all
        bincode::decode_from_slice(bytes, BINCODE_CONFIG).unwrap().0
    }
}

impl From<Vec<u8>> for ServerMessageData {
    fn from(bytes: Vec<u8>) -> Self {
        // todo: assert read all
        bincode::decode_from_slice(&bytes, BINCODE_CONFIG).unwrap().0
    }
}

impl Into<Vec<u8>> for ServerMessageData {
    fn into(self) -> Vec<u8> {
        bincode::encode_to_vec(self, BINCODE_CONFIG).unwrap()
    }
}
