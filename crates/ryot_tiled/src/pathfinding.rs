use crate::prelude::TilePosition;
use ryot_pathfinder::prelude::*;

pub type TiledPath = Path<TilePosition>;
pub type TiledPathFindingQuery = PathFindingQuery<TilePosition>;
