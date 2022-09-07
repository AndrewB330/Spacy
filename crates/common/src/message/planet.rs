use crate::planet::PlanetId;

use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub struct SpawnPlanet {
    pub planet_id: PlanetId,
    pub mass: f32,
    pub radius: f32,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}
