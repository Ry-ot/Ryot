mod properties;
pub use properties::{Elevation, Properties};

mod flags;
pub use flags::Flags;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Bottom,
    Containers,
    Corpses,
    Decor,
    Edges,
    Ground,
    #[default]
    Miscellaneous,
    Top,
    Wearable,
    Custom(i32),
}
