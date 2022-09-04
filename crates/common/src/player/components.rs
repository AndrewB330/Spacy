use bevy::prelude::*;

use crate::player::PlayerId;

const PLAYER_HEAD_HEIGHT: f32 = 0.3;

#[derive(Bundle)]
pub struct PlayerHeadBundle {
    pub player_head: PlayerHead,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Component)]
pub struct PlayerHead {
    pub player_id: PlayerId,
}

impl PlayerHeadBundle {
    pub fn new(player_id: PlayerId) -> Self {
        PlayerHeadBundle {
            player_head: PlayerHead { player_id },
            transform: Transform::from_translation(Vec3::new(0.0, PLAYER_HEAD_HEIGHT, 0.0)),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
