use bevy_rapier3d::prelude::CollisionGroups;

pub const PLANET_COLLISION_BIT: u32 = 1 << 0;
pub const PLAYER_COLLISION_BIT: u32 = 1 << 1;
pub const FAKE_PLAYER_COLLISION_BIT: u32 = 1 << 2;

pub const PLANET_COLLISION_FILTER: u32 = !0;
pub const PLAYER_COLLISION_FILTER: u32 = !0;
pub const FAKE_PLAYER_COLLISION_FILTER: u32 = 0;

pub const PLANET_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(PLANET_COLLISION_BIT, PLANET_COLLISION_FILTER);
pub const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(PLAYER_COLLISION_BIT, PLAYER_COLLISION_FILTER);
pub const FAKE_PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(FAKE_PLAYER_COLLISION_BIT, FAKE_PLAYER_COLLISION_FILTER);
