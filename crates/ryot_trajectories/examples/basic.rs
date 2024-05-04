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
fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (basic_setup, spawn_obstacle()))
        .add_systems(First, draw_grid::<Pos>)
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
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        visible_trajectory::<(), Pos>(
            // RadialArea::default()
            //     .with_range(15)
            //     .with_angle_range((0, 1)),
            RadialArea::circle().with_range(5),
        )
        .with_execution_type(ExecutionType::TimeBased(Duration::from_millis(200))),
        Pos::generate(0, 0, 0),
    ));
}

pub fn spawn_obstacle() -> impl FnMut(ResMut<Cache<Pos, Flags>>) {
    move |cache: ResMut<Cache<Pos, Flags>>| {
        let Ok(mut write_guard) = cache.write() else {
            return;
        };

        write_guard.insert(
            Pos::generate(1, 1, 0),
            Flags::default().with_blocks_sight(true),
        );

        write_guard.insert(
            Pos::generate(3, 3, 0),
            Flags::default().with_blocks_sight(true),
        );
    }
}

fn draw_grid<P: Point + Into<Vec2>>(mut gizmos: Gizmos) {
    for x in -10..=10 {
        for y in -10..=10 {
            gizmos.rect_2d(
                P::generate(x, y, 0).into(),
                0.,
                tile_size().as_vec2(),
                Color::WHITE,
            );
        }
    }
}

fn process_intersections(mut gizmos: Gizmos, player_query: Query<&Intersections<(), Pos>>) {
    for intersections in &player_query {
        for pos in intersections.area_of_interest.iter() {
            gizmos.circle_2d(
                (*pos).into(),
                (tile_size().x / 2) as f32,
                Color::MIDNIGHT_BLUE.as_rgba().with_a(1.9),
            );
        }
    }
}
