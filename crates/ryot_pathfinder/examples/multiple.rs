use crate::shared::example_builder::ExampleBuilder;
use crate::shared::pos::Pos;

mod shared {
    pub mod example_builder;
    pub mod pos;
}

fn main() {
    // Here we use a Pos as the representation of a point and () as the navigable type, meaning that
    // all positions are walkable. We override the default params to run 10 actors.
    ExampleBuilder::<Pos, ()>::default()
        .with_n_entities(10)
        .drawing_app()
        .run();
}
