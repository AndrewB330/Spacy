use std::sync::atomic::{AtomicU32, Ordering};

use bincode::{config::{Configuration, standard}, Decode, Encode};
use crate::player::PlayerAction;

#[derive(Debug, Clone, Decode, Encode)]
pub enum UserMessage {
    Ping,
    PlayerAction(PlayerAction),
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum ServerMessage {
    Pong,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct UserId(u32);

impl UserId {
    pub fn new() -> UserId {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        UserId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}