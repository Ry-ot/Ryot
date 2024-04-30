use crate::shared::example_builder::ExampleBuilder;
use crate::shared::pos::Pos;
use bevy::diagnostic::*;
use ryot_core::prelude::Flags;

mod shared {
    pub mod example_builder;
    pub mod pos;
}

fn main() {
    // In this example we are stress testing the setup that uses Pos and Flags types to represent
    // the position and navigable type of the pathfinder. We override the default params to run
    // 1_000_000 actors, add 1_000_000 obstacles and set them as non-walkable. We also increased
    // the max distance and the grid size to make it a bit more challenging.
    ExampleBuilder::<Pos, Flags>::default()
        .with_grid_size(100_000)
        .with_n_entities(1_000_000)
        .with_n_obstacles(1_000_000)
        .with_max_distance(25)
        .with_navigable(Flags::default().with_walkable(false))
        .minimum_app()
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .run();
}
