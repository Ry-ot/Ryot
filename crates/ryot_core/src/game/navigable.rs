/// Defines behavior for tiles or entities in terms of navigation and visibility within the Ryot framework.
///
/// This trait abstracts the walkability and sight-blocking properties to ensure compatibility with
/// generic systems such as pathfinding and ray casting, facilitating their application across
/// different types of game environments and scenarios.
///
/// Implementing this trait allows for consistent behavior across various game elements, making
/// it integral to developing flexible and reusable game mechanics.
///
/// # Examples
///
/// Implementing `Navigable` for a custom tile type might look like this:
///
/// ```
/// use ryot_core::prelude::Navigable;
///
/// #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// struct CustomTile {
///     walkable: bool,
///     sight_blocking: bool,
/// }
///
/// impl Navigable for CustomTile {
///     fn is_walkable(&self) -> bool {
///         self.walkable
///     }
///
///     fn blocks_sight(&self) -> bool {
///         self.sight_blocking
///     }
///
///     fn set_walkable(&mut self, walkable: bool) {
///         self.walkable = walkable;
///     }
///
///     fn set_blocks_sight(&mut self, sight_blocking: bool) {
///         self.sight_blocking = sight_blocking;
///     }
/// }
/// ```
pub trait Navigable: Sync + Send + 'static {
    fn is_walkable(&self) -> bool {
        true
    }

    fn blocks_sight(&self) -> bool {
        false
    }

    fn set_walkable(&mut self, _: bool) {}
    fn set_blocks_sight(&mut self, _: bool) {}

    fn is_default(&self) -> bool {
        false
    }

    fn append_walkable(&mut self, walkable: bool) {
        self.set_walkable(self.is_walkable() && walkable);
    }

    fn append_blocks_sight(&mut self, blocks_sight: bool) {
        self.set_blocks_sight(self.blocks_sight() || blocks_sight);
    }
}

pub fn append_navigable<N1: Navigable, N2: Navigable>(mut a: N1, b: &N2) -> N1 {
    a.append_walkable(b.is_walkable());
    a.append_blocks_sight(b.blocks_sight());

    a
}

impl Navigable for () {}
