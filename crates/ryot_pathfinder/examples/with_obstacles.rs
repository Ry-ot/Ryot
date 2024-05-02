#[path = "../shared_stubs/example_builder.rs"]
pub mod example_builder;
use example_builder::*;

#[path = "../shared_stubs/pos.rs"]
pub mod pos;
use pos::Pos;

#[path = "../shared_stubs/is_walkable.rs"]
pub mod is_walkable;
use is_walkable::IsWalkable;

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
