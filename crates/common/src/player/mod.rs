use std::sync::atomic::{AtomicU32, Ordering};

use bevy::prelude::Component;
use bincode::{Decode, Encode};

mod components;

pub use components::*;

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
