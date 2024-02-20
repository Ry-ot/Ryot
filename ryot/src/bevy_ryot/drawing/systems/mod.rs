//! This module contains all the systems for drawing the game.
//! The systems are used to draw the map and the entities that are on it.
//! The systems manipulate basic drawing entities that are added by the drawing commands.
//! Those entities are the trigger for the drawing systems within the ECS.
use bevy::prelude::SystemSet;

mod deletion;
pub use deletion::*;

mod update;
pub use update::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DrawingSystems {
    Apply,
    Persist,
}
