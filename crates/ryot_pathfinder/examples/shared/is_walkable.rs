use bevy::prelude::Component;
use ryot_core::game::Navigable;

/// This is an example on how to use the Navigable trait to flag non-walkable pathfinding points.
/// Here we are defining a IsWalkable component that will be used to flag non-walkable points.
#[derive(Eq, PartialEq, Component, Clone, Copy, Debug, Hash)]
pub struct IsWalkable(pub bool);

impl Default for IsWalkable {
    fn default() -> Self {
        Self(true)
    }
}

/// We are implementing the Navigable trait for IsWalkable, where walkable points are flagged based
/// on the boolean value of the IsWalkable component, blocks-sight is irrelevant for this example.
///
/// Append is implemented to allow combining multiple IsWalkable components, where the result will
/// be non-walkable if any of the components is non-walkable.
///
/// is_default is implemented to allow skipping from the cache when the IsWalkable component is
/// the default value (in this case, true).
impl Navigable for IsWalkable {
    fn is_walkable(&self) -> bool {
        self.0
    }

    fn blocks_sight(&self) -> bool {
        true
    }

    fn append(mut self, navigable: &impl Navigable) -> Self {
        self.0 &= navigable.is_walkable();
        self
    }

    fn is_default(&self) -> bool {
        self.0
    }
}
