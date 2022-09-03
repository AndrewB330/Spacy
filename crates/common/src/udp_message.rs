use bincode::{config::{Configuration, standard}, Decode, Encode};

use crate::message::{ServerMessage, UserMessage};
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
            UdpUserMessage::Message(_, message) => {
                match message {
                    UserMessage::Ping => 0,
                    UserMessage::PlayerAction(PlayerAction::Jump | PlayerAction::UseTool | PlayerAction::UseToolSpecial) => 10,
                    UserMessage::PlayerAction(PlayerAction::Move(_)) => 0,
                }
            }
            UdpUserMessage::Ack(_) => 0,
        }
    }
}

impl UdpServerMessage {
    pub fn retries_number(&self) -> u32 {
        match self {
            UdpServerMessage::Message(_, message) => {
                match message {
                    ServerMessage::Pong => 0,
                }
            }
            UdpServerMessage::Ack(_) => 0,
        }
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