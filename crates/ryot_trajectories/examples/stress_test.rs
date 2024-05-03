//! Shows how to do the bare minimum to use trajectories within ryot

use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use ryot_core::game::Point;
use ryot_core::prelude::Flags;
use ryot_trajectories::prelude::{
    visible_trajectory, InterestPositions, RadialArea, TrajectoryApp,
};
use ryot_trajectories::stubs::Pos;
use ryot_utils::cache::Cache;
use ryot_utils::prelude::OptionalPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, (basic_setup, spawn_obstacle()))
        .add_systems(Update, process_interest)
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
    commands.spawn(Camera2dBundle::default());

    for _ in 0..10_000 {
        commands.spawn(visible_trajectory::<(), Pos>(
            RadialArea::default()
                .with_range(15)
                .with_angle_range((0, 1)),
            // RadialArea::circle().with_range(5),
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

fn process_interest(player_query: Query<&InterestPositions<(), Pos>>) {
    for interest_positions in &player_query {
        for _ in interest_positions.positions.iter() {
            // gizmos.circle_2d((*pos).into(), (tile_size().x / 2) as f32, Color::BLUE);
        }
    }
}
