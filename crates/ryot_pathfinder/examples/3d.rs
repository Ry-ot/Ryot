//! Shows how to do the bare minimum to execute a path finding using ryot.
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_app::{App, Startup, Update};
use bevy_ecs::prelude::Commands;
use ryot_core::prelude::Point;
use ryot_pathfinder::pathable::PathableApp;

#[path = "../shared_stubs/example_builder.rs"]
pub mod example_builder;
use example_builder::*;

#[path = "../shared_stubs/pos.rs"]
pub mod pos;
use pos::Pos;

fn main() {
    let builder = ExampleBuilder::<Pos, ()>::default()
        .with_grid_size(3)
        .with_max_distance(3);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, basic_setup)
        .add_systems(Update, (draw_grid, draw_actors, draw_target))
        .add_pathable::<Pos, ()>()
        .add_systems(
            Startup,
            (builder.spawn_many(), builder.spawn_obstacles(false)),
        )
        .add_systems(Update, (builder.start_path(), builder.process_path()))
        .run();
}

pub fn basic_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3::splat(10.)),
        ..default()
    });
}

pub fn draw_grid(mut gizmos: Gizmos) {
    let size = 3;

    for x in -size..=size {
        for y in -size..=size {
            for z in -size..=size {
                gizmos.cuboid(
                    Transform::from_translation(Pos::generate(x, y, z).into()),
                    Color::WHITE,
                )
            }
        }
    }
}

pub fn draw_actors(mut gizmos: Gizmos, q_paths: Query<&Pos>) {
    for pos in &q_paths {
        gizmos.cuboid(Transform::from_translation((*pos).into()), Color::RED)
    }
}

pub fn draw_target(mut gizmos: Gizmos, q_targets: Query<&Pathing<Pos>>) {
    for Pathing(pos) in &q_targets {
        gizmos.cuboid(Transform::from_translation((*pos).into()), Color::GREEN)
    }
}
