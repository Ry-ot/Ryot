#[path = "../shared_stubs/example_builder.rs"]
pub mod example_builder;
use example_builder::*;

#[path = "../shared_stubs/pos.rs"]
pub mod pos;
use pos::Pos;

fn main() {
    // Here we use a Pos as the representation of a point and () as the navigable type, meaning that
    // all positions are walkable. We override the default params to run 10 actors.
    ExampleBuilder::<Pos, ()>::default()
        .with_n_entities(10)
        .drawing_app()
        .run();
}
