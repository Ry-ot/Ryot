use bevy_ecs::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AnimationSystems {
    Initialize,
    Update,
}
