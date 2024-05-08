use bevy::app::App;
use bevy::diagnostic::*;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use big_brain::pickers::FirstToScore;
use big_brain::prelude::Thinker;
use big_brain::{BigBrainPlugin, BigBrainSet};
use ryot_brain::prelude::{
    find_closest_target, find_path_scorer, follow_path, follow_path_scorer, moves_randomly,
    MovesRandomly, PathFindingThinker, PathFollowingThinker,
};
use ryot_core::game::Point;
use ryot_core::prelude::Flags;
use ryot_pathfinder::pathable::PathableApp;
use ryot_tiled::prelude::{OrdinalDirection, TilePosition};
use ryot_utils::prelude::*;

#[derive(Component, Copy, Debug, Clone)]
pub struct Target;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin {
            // Use `RUST_LOG=big_brain=trace,sequence=trace cargo run --example sequence --features=trace` to see extra tracing output.
            filter: "ryot_brain=debug".to_string(),
            ..default()
        })
        .add_pathable::<TilePosition, Flags>()
        .add_systems(Startup, (spawn_basic))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_systems(
            PreUpdate,
            (find_path_scorer::<Target>, follow_path_scorer).in_set(BigBrainSet::Scorers),
        )
        .add_systems(
            PreUpdate,
            (
                find_closest_target::<Target>,
                follow_path.pipe(initiate_walk),
                moves_randomly.pipe(initiate_walk),
            )
                .in_set(BigBrainSet::Actions)
                .after(CacheSystems::UpdateCache),
        )
        .add_systems(Update, shuffle_target_positions_every_x_seconds)
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .run();
}

fn initiate_walk(
    In(walks): In<Vec<(Entity, OrdinalDirection)>>,
    mut positions: Query<&mut TilePosition>,
) {
    for (entity, direction) in walks {
        if let Ok(mut position) = positions.get_mut(entity) {
            *position = TilePosition(position.0 + IVec2::from(direction).extend(0))
        }
    }
}

fn spawn_basic(mut commands: Commands) {
    commands.spawn((
        TilePosition::generate(
            rand::random::<i32>() % 100 + 1,
            rand::random::<i32>() % 100 + 1,
            0,
        ),
        Target,
    ));
    for i in 0..=400_000 {
        commands.spawn((
            TilePosition::generate(
                rand::random::<i32>() % 100 + 1,
                rand::random::<i32>() % 100 + 1,
                0,
            ),
            Thinker::build()
                .label("ChaserThinker")
                .picker(FirstToScore::new(0.6))
                .find_path::<Target>()
                .follows_path_with_fallback(MovesRandomly),
        ));
    }
}

fn shuffle_target_positions_every_x_seconds(
    time: Res<Time>,
    mut query: Query<&mut TilePosition, With<Target>>,
) {
    for mut position in &mut query.iter_mut() {
        if time.elapsed_seconds() % 5. < 0.0001 {
            *position = TilePosition::generate(
                rand::random::<i32>() % 100 + 1,
                rand::random::<i32>() % 100 + 1,
                0,
            );
        }
    }
}
