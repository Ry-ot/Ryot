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
