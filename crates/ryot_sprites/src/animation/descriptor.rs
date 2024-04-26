use crate::prelude::*;
use bevy_ecs::component::Component;

#[derive(Debug, Clone)]
pub struct AnimationDescriptor {
    pub sprites: Vec<LoadedSprite>,
    pub layers: usize,
    pub skip: usize,
    pub synchronized: bool,
}

#[derive(Component, Clone, Debug)]
pub enum AnimationSprite {
    Independent {
        key: AnimationKey,
        descriptor: AnimationDescriptor,
        state: AnimationState,
    },
    Synchronized {
        key: AnimationKey,
        descriptor: AnimationDescriptor,
    },
}

impl AnimationSprite {
    pub fn from_key_and_descriptor(key: &AnimationKey, descriptor: &AnimationDescriptor) -> Self {
        if descriptor.synchronized {
            AnimationSprite::Synchronized {
                key: key.clone(),
                descriptor: descriptor.clone(),
            }
        } else {
            AnimationSprite::Independent {
                key: key.clone(),
                descriptor: descriptor.clone(),
                state: key.default_state(),
            }
        }
    }
}
