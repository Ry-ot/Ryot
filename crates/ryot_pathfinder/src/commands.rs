use crate::components::PathFindingQuery;
use crate::pathable::Pathable;
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Copy)]
pub struct AmendPathCommand<P: Pathable> {
    pub entity: Entity,
    pub path_amend_index: usize,
    pub path_finding_query: PathFindingQuery<P>,
}
