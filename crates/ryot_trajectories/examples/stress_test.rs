//! Shows how to do the bare minimum to use trajectories within ryot

use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use ryot_core::game::Point;
use ryot_core::prelude::Flags;
use ryot_trajectories::prelude::*;
use ryot_trajectories::stubs::Pos;
use ryot_utils::cache::Cache;
use ryot_utils::prelude::OptionalPlugin;
use std::time::Duration;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, (basic_setup, spawn_obstacle()))
        .add_systems(Update, process_intersections)
        .add_trajectory::<(), Pos, Flags>()
        .add_optional_plugin(LogPlugin::default())
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ));

    app.run();
}

pub fn basic_setup(mut commands: Commands) {
    for _ in 0..10_000 {
        commands.spawn((
            visible_trajectory::<(), Pos>(RadialArea::circle().with_range(15))
                .with_execution_type(ExecutionType::TimeBased(Duration::from_millis(500))),
            Pos::generate(0, 0, 0),
        ));
    }

    for _ in 0..100_000 {
        commands.spawn((
            visible_trajectory::<(), Pos>(
                RadialArea::default().with_range(1).with_angle_range((0, 1)),
            )
            .with_execution_type(ExecutionType::TimeBased(Duration::from_millis(500))),
            Pos::generate(0, 0, 0),
        ));
    }

    for _ in 0..50_000 {
        commands.spawn((
            visible_trajectory::<(), Pos>(RadialArea::circle().with_range(3))
                .with_execution_type(ExecutionType::TimeBased(Duration::from_millis(500))),
            Pos::generate(0, 0, 0),
        ));
    }

    for _ in 0..100_000 {
        commands.spawn((
            visible_trajectory::<(), Pos>(
                RadialArea::default()
                    .with_range(15)
                    .with_angle_range((0, 1)),
            )
            .with_execution_type(ExecutionType::TimeBased(Duration::from_millis(500))),
            Pos::generate(0, 0, 0),
        ));
    }
}

pub fn spawn_obstacle() -> impl FnMut(Commands, ResMut<Cache<Pos, Flags>>) {
    move |mut commands: Commands, cache: ResMut<Cache<Pos, Flags>>| {
        let Ok(mut write_guard) = cache.write() else {
            return;
        };

        write_guard.insert(
            Pos::generate(1, 1, 0),
            Flags::default().with_blocks_sight(true),
        );
    }
}

fn process_intersections(
    // mut gizmos: Gizmos,
    mut player_query: Query<&mut Intersections<(), Pos>>,
) {
    for mut intersections in player_query.iter_mut() {
        for intersection in intersections.area_of_interest.iter() {
            // gizmos.circle_2d(
            //     intersection.position.into(),
            //     (tile_size().x / 2) as f32,
            //     Color::MIDNIGHT_BLUE.as_rgba().with_a(1.9),
            // );
        }
    }
}
