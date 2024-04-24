use crate::prelude::TilePosition;
use derive_more::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// Component to track the previous position of an entity.
/// Useful when needing to deal with both the current and previous position of an entity.
#[derive(Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect, Deref, DerefMut))]
pub struct PreviousPosition(pub TilePosition);

/// System to track changes in the position of entities. Needs to be run after the position
/// component has been changed, so it can track the previous position.
/// Better to run it during the [`Last`](Last) or [`PostUpdate`](PostUpdate) stages.
#[cfg(feature = "bevy")]
pub(crate) fn track_position_changes(
    mut commands: Commands,
    mut query: Query<(Entity, &TilePosition, Option<&mut PreviousPosition>), Changed<TilePosition>>,
) {
    for (entity, pos, prev_pos) in query.iter_mut() {
        if let Some(mut prev_pos) = prev_pos {
            prev_pos.0 = *pos;
        } else {
            commands.entity(entity).insert(PreviousPosition(*pos));
        }
    }
}
