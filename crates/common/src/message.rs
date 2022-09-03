use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use bincode::{Decode, Encode};

use crate::player::{PlayerAction, PlayerId, PlayerUpdate};

#[derive(Debug, Clone, Decode, Encode)]
pub enum UserMessageData {
    Ping,
    PlayerAction(PlayerAction),
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct UserMessage {
    pub time: MessageTime,
    pub data: UserMessageData,
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum ServerMessageData {
    Pong,
    PlayerUpdate(PlayerId, bool, PlayerUpdate),
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct ServerMessage {
    pub time: MessageTime,
    pub data: ServerMessageData,
}

impl From<ServerMessageData> for ServerMessage {
    fn from(data: ServerMessageData) -> Self {
        ServerMessage {
            time: MessageTime::now(),
            data,
        }
    }
}

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageTime(u64);

impl MessageTime {
    pub fn now() -> MessageTime {
        MessageTime(chrono::Utc::now().timestamp() as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct UserId(u32);

impl UserId {
    pub fn new() -> UserId {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        UserId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
