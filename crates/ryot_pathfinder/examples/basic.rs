//! Shows how to do the bare minimum to execute a path finding using ryot.
use crate::shared::example_builder::*;
use crate::shared::pos::Pos;

mod shared {
    pub mod example_builder;
    pub mod pos;
}

fn main() {
    // Here we use a Pos as the representation of a point and () as the navigable type, meaning that
    // all positions are walkable.
    //
    // The params are:
    //  grid_size: 10,
    //  n_entities: 1,
    //  n_obstacles: 0,
    //  max_distance: 10,
    //  sleep: 100,
    //  query_builder: |pos| PathFindingQuery::new(pos).with_success_distance(0.),
    ExampleBuilder::<Pos, ()>::default().drawing_app().run();
}
