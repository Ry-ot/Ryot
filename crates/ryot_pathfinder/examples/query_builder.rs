use ryot_pathfinder::components::PathFindingQuery;

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
    // This example is using a custom builder where diagonal movements are cheaper than cardinal
    // movements, to exemplify the different capabilities of the path finding query. This leads to
    // an actor that moves mostly diagonally.
    ExampleBuilder::<Pos, IsWalkable>::default()
        .with_n_obstacles(200)
        .with_query_builder(|pos| {
            PathFindingQuery::new(pos)
                .with_success_distance(0.)
                .with_diagonal_cost(1)
                .with_cardinal_cost(5)
        })
        .with_navigable(IsWalkable(false))
        .drawing_app()
        .run();
}
