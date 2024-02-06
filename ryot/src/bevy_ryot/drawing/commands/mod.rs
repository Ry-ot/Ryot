use crate::bevy_ryot::drawing::Layer;
use crate::position::TilePosition;
use bevy::prelude::{Commands, Deref, DerefMut, Entity, Resource};

mod update_tile_content;
pub use update_tile_content::*;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct DrawingCommandHistory {
    pub commands: Vec<ReversibleCommandRecord>,
}

pub trait ReversibleCommand: Send + Sync + 'static {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>);
}

pub struct ReversibleCommandRecord {
    pub layer: Layer,
    pub tile_pos: TilePosition,
    pub command: Box<dyn ReversibleCommand>,
}

impl ReversibleCommandRecord {
    pub fn new(layer: Layer, tile_pos: TilePosition, command: Box<dyn ReversibleCommand>) -> Self {
        Self {
            layer,
            tile_pos,
            command,
        }
    }
}
