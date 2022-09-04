use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use bincode::{Decode, Encode};

use crate::player::{PlayerAction, PlayerId, PlayerInfo};
use crate::sync::SyncTargetId;

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
    Transform(SyncTargetId, [f32; 3], [f32; 4]),
    PlayerInfo(PlayerId, PlayerInfo),
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

impl From<UserMessageData> for UserMessage {
    fn from(data: UserMessageData) -> Self {
        UserMessage {
            // TODO: sync time with server??? for now just return zero time
            time: MessageTime::default(),
            data,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageTime(u64);

impl MessageTime {
    pub fn now() -> MessageTime {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");
        MessageTime(now.as_millis() as u64)
    }

    pub fn before(&self, millis: u64) -> MessageTime {
        MessageTime(self.0 - millis)
    }

    pub fn after(&self, millis: u64) -> MessageTime {
        MessageTime(self.0 + millis)
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
