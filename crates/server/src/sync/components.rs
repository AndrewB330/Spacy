use crate::Transform;
use bevy::prelude::*;

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
