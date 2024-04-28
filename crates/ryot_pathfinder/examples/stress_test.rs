mod shared {
    pub mod example_builder;
}

#[cfg(feature = "ryot_tiled")]
fn main() {
    use crate::shared::example_builder::ExampleBuilder;
    use bevy::diagnostic::*;
    use ryot_core::prelude::Flags;
    use ryot_tiled::prelude::TilePosition;

    // In this example we are stress testing the setup that uses ryot_tiled pre-build TilePosition and
    // Flags types to represent the position and navigable type of the pathfinder. We override the
    // default params to run 1_000_000 actors, add 1_000_000 obstacles and set them as non-walkable.
    // We also increased the max distance and the grid size to make it a bit more challenging.
    ExampleBuilder::<TilePosition, Flags>::default()
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

#[cfg(not(feature = "ryot_tiled"))]
fn main() {
    eprintln!(
        "\x1b[93m[WARNING]\x1b[0m Please run `cargo run --example stress_test --features ryot_tiled`"
    );
}
