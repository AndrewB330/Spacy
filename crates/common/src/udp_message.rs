use std::time::Duration;

use bincode::{
    config::{Configuration, standard},
    Decode, Encode,
};

use crate::message::{ServerMessage, ServerMessageData, UserMessage, UserMessageData};
use crate::player::PlayerAction;

#[derive(Debug, Clone, Decode, Encode)]
pub enum UdpUserMessage {
    Message(u64, UserMessage),
    Ack(u64),
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum UdpServerMessage {
    Message(u64, ServerMessage),
    Ack(u64),
}

impl UdpUserMessage {
    pub fn retries_number(&self) -> u32 {
        match self {
            UdpUserMessage::Message(_, message) => match message.data {
                UserMessageData::Ping => 0,
                UserMessageData::PlayerAction(
                    PlayerAction::Jump | PlayerAction::UseTool | PlayerAction::UseToolSpecial,
                ) => 10,
                UserMessageData::PlayerAction(PlayerAction::Move(_)) => 0,
            },
            UdpUserMessage::Ack(_) => 0,
        }
    }

    pub fn retry_timeout(&self) -> Duration {
        Duration::from_millis(10)
    }

    pub fn need_ack(&self) -> bool {
        self.retries_number() > 0
    }
}

impl UdpServerMessage {
    pub fn retries_number(&self) -> u32 {
        match self {
            UdpServerMessage::Message(_, message) => match message.data {
                ServerMessageData::Pong => 0,
                ServerMessageData::PlayerUpdate(_, _, _) => 0,
            },
            UdpServerMessage::Ack(_) => 0,
        }
    }

    pub fn retry_timeout(&self) -> Duration {
        Duration::from_millis(100)
    }

    pub fn need_ack(&self) -> bool {
        self.retries_number() > 0
    }
}

const BINCODE_CONFIG: Configuration = standard();

impl UdpUserMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, BINCODE_CONFIG).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> UdpUserMessage {
        bincode::decode_from_slice(bytes, BINCODE_CONFIG).unwrap().0
    }
}

impl UdpServerMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, BINCODE_CONFIG).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> UdpServerMessage {
        bincode::decode_from_slice(bytes, BINCODE_CONFIG).unwrap().0
    }
}
