use ryot_core::prelude::Flags;

#[path = "../shared_stubs/example_builder.rs"]
pub mod example_builder;
use example_builder::*;

#[path = "../shared_stubs/pos.rs"]
pub mod pos;
use pos::Pos;

fn main() {
    // In this example we are leveraging the Pos and Flags types to represent the position and
    // navigable type of the pathfinder. We override the default params to run 10 actors,
    // add 200 obstacles and set them as non-walkable.
    ExampleBuilder::<Pos, Flags>::default()
        .with_n_entities(10)
        .with_n_obstacles(200)
        .with_navigable(Flags::default().with_walkable(false))
        .drawing_app()
        .run();
}
