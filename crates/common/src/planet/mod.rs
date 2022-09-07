mod components;

use bevy::prelude::*;
use bincode::{Decode, Encode};
use std::sync::atomic::{AtomicU32, Ordering};

pub use components::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Component, Decode, Encode)]
pub struct PlanetId(u32);

impl PlanetId {
    pub fn new() -> PlanetId {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        PlanetId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
