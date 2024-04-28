use crate::shared::example_builder::ExampleBuilder;
use crate::shared::is_walkable::IsWalkable;
use crate::shared::pos::Pos;

mod shared {
    pub mod example_builder;
    pub mod is_walkable;
    pub mod pos;
}

fn main() {
    // Here we use a Pos as the representation of a point and IsWalkable as the navigable type, to
    // determine whether a position is walkable or not. We override the default params to add 200
    // obstacles and set them as non-walkable.
    ExampleBuilder::<Pos, IsWalkable>::default()
        .with_n_obstacles(200)
        .with_navigable(IsWalkable(false))
        .drawing_app()
        .run();
}
