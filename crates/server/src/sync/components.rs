use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Component, Default)]
pub struct SyncSpawn {
    pub spawn_sent: HashSet<u32>,
}
