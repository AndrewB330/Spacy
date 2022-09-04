use std::sync::atomic::{AtomicU32, Ordering};

use bevy::prelude::Component;
use bincode::{Decode, Encode};

pub use message::*;

mod components;
mod message;

pub use components::{PlayerHead, PlayerHeadBundle};

pub const PLAYER_CAPSULE_HEIGHT: f32 = 0.5;
pub const PLAYER_CAPSULE_RADIUS: f32 = 0.4;
pub const PLAYER_ABOVE_GROUND: f32 = 0.2;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Component, Decode, Encode)]
pub struct PlayerId(u32);

impl PlayerId {
    pub fn new() -> PlayerId {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        PlayerId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }

    pub fn get_inner(&self) -> u32 {
        self.0
    }
}
