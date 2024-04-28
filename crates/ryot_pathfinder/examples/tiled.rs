mod shared {
    pub mod example_builder;
}

#[cfg(feature = "ryot_tiled")]
fn main() {
    use crate::shared::example_builder::ExampleBuilder;
    use ryot_core::prelude::Flags;
    use ryot_tiled::prelude::TilePosition;

    // In this example we are leveraging the ryot_tiled pre-build TilePosition and Flags types to
    // represent the position and navigable type of the pathfinder. We override the default params to
    // run 10 actors, add 200 obstacles and set them as non-walkable.
    ExampleBuilder::<TilePosition, Flags>::default()
        .with_n_entities(10)
        .with_n_obstacles(200)
        .with_navigable(Flags::default().with_walkable(false))
        .drawing_app()
        .run();
}

#[cfg(not(feature = "ryot_tiled"))]
fn main() {
    eprintln!(
        "\x1b[93m[WARNING]\x1b[0m Please run `cargo run --example tiled --features ryot_tiled`"
    );
}
