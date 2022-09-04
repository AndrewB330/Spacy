use crate::Transform;
use bevy::prelude::*;
use bevy::utils::hashbrown::hash_map::Entry;
use bevy::utils::HashMap;
use common::message::{MessageTime, UserId};
use common::sync::SyncLabel;

#[derive(Component, Default)]
pub struct SyncHistory {
    times: HashMap<UserId, HashMap<SyncLabel, MessageTime>>,
}

impl SyncHistory {
    pub fn get_time(&self, user_id: UserId, label: SyncLabel) -> MessageTime {
        self.times
            .get(&user_id)
            .and_then(|h| h.get(&label))
            .copied()
            .unwrap_or_default()
    }

    pub fn set_time(&mut self, user_id: UserId, label: SyncLabel, time: MessageTime) {
        match self.times.entry(user_id) {
            Entry::Occupied(mut o) => {
                o.get_mut().insert(label, time);
            }
            Entry::Vacant(v) => {
                let mut hm = HashMap::default();
                hm.insert(label, time);
                v.insert(hm);
            }
        }
    }
}

pub trait TransformWrapper {
    fn get_translation(&self) -> Vec3;

    fn get_rotation(&self) -> Quat;
}

impl TransformWrapper for Transform {
    fn get_translation(&self) -> Vec3 {
        self.translation
    }

    fn get_rotation(&self) -> Quat {
        self.rotation
    }
}
