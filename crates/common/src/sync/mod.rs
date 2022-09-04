use crate::planet::PlanetId;
use crate::player::PlayerId;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub enum SyncTargetId {
    Player(PlayerId),
    Planet(PlanetId),
}

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub enum SyncLabel {
    Transform,
    Info,
}

pub trait SyncTarget {
    fn get_id(&self) -> SyncTargetId;
}
