use crate::player::UserPlayer;
use bevy::prelude::*;
use common::message::planet::SpawnPlanet;
use common::message::player::SpawnPlayer;
use common::message::{ServerMessageData, SpawnInfo};
use common::planet::{ParentPlanet, Planet, PlanetId};
use common::player::Player;
use common::sync::{SyncTarget, SyncTargetId};
use common::user::UserId;

pub(super) trait TransformInfoProvider {
    fn get_target_id(&self) -> SyncTargetId;

    fn get_translation(&self) -> Vec3;

    fn get_rotation(&self) -> Quat;

    fn get_parent_planet(&self) -> Option<PlanetId>;
}

pub(super) trait SpawnInfoProvider {
    fn get_spawn_info(&self, user_id: UserId) -> SpawnInfo;
}

impl<T: SyncTarget> TransformInfoProvider for (&T, &Transform, Option<&ParentPlanet>) {
    fn get_target_id(&self) -> SyncTargetId {
        self.0.get_id()
    }

    fn get_translation(&self) -> Vec3 {
        self.1.translation
    }

    fn get_rotation(&self) -> Quat {
        self.1.rotation
    }

    fn get_parent_planet(&self) -> Option<PlanetId> {
        self.2.map(|v| v.parent_planet_id)
    }
}

impl TransformInfoProvider for &Planet {
    fn get_target_id(&self) -> SyncTargetId {
        self.get_id()
    }

    fn get_translation(&self) -> Vec3 {
        self.real_translation
    }

    fn get_rotation(&self) -> Quat {
        self.real_rotation
    }

    fn get_parent_planet(&self) -> Option<PlanetId> {
        None
    }
}

impl SpawnInfoProvider
    for (
        &Player,
        Option<&UserPlayer>,
        &Transform,
        Option<&ParentPlanet>,
    )
{
    fn get_spawn_info(&self, user_id: UserId) -> SpawnInfo {
        SpawnInfo::SpawnPlayer(SpawnPlayer {
            player_id: self.0.player_id,
            is_me: Some(user_id) == self.1.map(|v| v.user_id),
            is_user: self.1.is_some(),
            parent_planet_id: self.3.map(|v| v.parent_planet_id),
            translation: self.2.translation.to_array(),
            rotation: self.2.rotation.to_array(),
        })
    }
}

impl SpawnInfoProvider for &Planet {
    fn get_spawn_info(&self, _: UserId) -> SpawnInfo {
        SpawnInfo::SpawnPlanet(SpawnPlanet {
            planet_id: self.planet_id,
            mass: self.mass,
            radius: self.radius,
            translation: self.real_translation.to_array(),
            rotation: self.real_rotation.to_array(),
        })
    }
}
