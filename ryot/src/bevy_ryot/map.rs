use bevy::prelude::{Deref, DerefMut, Entity, Reflect, Resource};
use bevy::utils::HashMap;

use crate::{layer::Layer, position::TilePosition};

/// A resource that holds the map tiles and the entities that are drawn on them.
/// An entity location is represented by the combination of a Layer and a Position.
/// The MapTiles are represented by a HashMap of TilePosition and a HashMap of Layer and Entity.
/// The MapTiles is used to keep track of the entities that are drawn on the map and their position.
#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct MapTiles(pub HashMap<TilePosition, HashMap<Layer, Entity>>);
