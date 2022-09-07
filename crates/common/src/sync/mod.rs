use crate::planet::{Planet, PlanetId};
use crate::player::{Player, PlayerId};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub enum SyncTargetId {
    Player(PlayerId),
    Planet(PlanetId),
}

pub trait SyncTarget {
    fn get_id(&self) -> SyncTargetId;
}

impl SyncTarget for Player {
    fn get_id(&self) -> SyncTargetId {
        SyncTargetId::Player(self.player_id)
    }
}

impl SyncTarget for Planet {
    fn get_id(&self) -> SyncTargetId {
        SyncTargetId::Planet(self.planet_id)
    }
}
