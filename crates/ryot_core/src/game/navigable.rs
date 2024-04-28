/// Defines behavior for tiles or entities in terms of navigation and visibility within the Ryot framework.
///
/// This trait abstracts the walkability and sight-blocking properties to ensure compatibility with
/// generic systems such as pathfinding and ray-casting, facilitating their application across
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
///     fn append(self, navigable: &impl Navigable) -> Self{
///         self
///     }
///
///     fn is_default(&self) -> bool {
///         true
///     }
/// }
/// ```
pub trait Navigable: Sync + Send + 'static {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
    fn append(self, navigable: &impl Navigable) -> Self;
    fn is_default(&self) -> bool;
}

impl Navigable for () {
    fn is_walkable(&self) -> bool {
        true
    }

    fn blocks_sight(&self) -> bool {
        false
    }

    fn append(self, _: &impl Navigable) -> Self {
        self
    }

    fn is_default(&self) -> bool {
        false
    }
}
