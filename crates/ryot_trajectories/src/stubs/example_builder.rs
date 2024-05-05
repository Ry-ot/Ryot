use crate::prelude::*;
use crate::stubs::Pos;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::log::LogPlugin;
use bevy::prelude::{Camera2dBundle, Color, Gizmos, Time, Timer, TimerMode};
use bevy::{DefaultPlugins, MinimalPlugins};
use bevy_app::*;
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::prelude::{Commands, Component, Local, Query, With};
use glam::{UVec2, Vec2};
use ryot_core::prelude::*;
use ryot_utils::app::OptionalPlugin;
use ryot_utils::cache::Cache;
use std::marker::PhantomData;

pub fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}

/// A marker component that represents the visual element of an obstacle.
#[derive(Component, Copy, Clone)]
pub struct Obstacle;

#[derive(Clone)]
pub struct ExampleBuilder<
    T: Copy + Send + Sync + 'static,
    P: TrajectoryPoint + Component,
    N: Navigable + Copy + Default,
> {
    grid_size: i32,
    n_obstacles: usize,
    trajectories: Vec<(TrajectoryRequest<T, P>, usize)>,
    _marker: PhantomData<(T, N)>,
}

impl<
        T: Copy + Send + Sync + 'static,
        P: TrajectoryPoint + Component,
        N: Navigable + Copy + Default,
    > Default for ExampleBuilder<T, P, N>
{
    fn default() -> Self {
        Self {
            grid_size: 10,
            n_obstacles: 0,
            trajectories: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<
        T: Copy + Send + Sync + 'static,
        P: TrajectoryPoint + Component + Into<Vec2>,
        N: Navigable + Copy + Default,
    > ExampleBuilder<T, P, N>
{
    pub fn new(
        grid_size: i32,
        trajectories: Vec<(TrajectoryRequest<T, P>, usize)>,
        n_obstacles: usize,
    ) -> Self {
        Self {
            grid_size,
            trajectories,
            n_obstacles,
            _marker: PhantomData,
        }
    }

    pub fn with_trajectories(
        mut self,
        trajectories: Vec<(TrajectoryRequest<T, P>, usize)>,
    ) -> Self {
        self.trajectories = trajectories;
        self
    }

    pub fn with_obstacles(mut self, n_obstacles: usize) -> Self {
        self.n_obstacles = n_obstacles;
        self
    }

    pub fn minimal_app(&self) -> App {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_optional_plugin(LogPlugin::default());

        self.setup_app(app)
    }

    pub fn app(&self) -> App {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins)
            .add_systems(Startup, self.spawn_camera())
            .add_systems(Update, self.draw_grid());

        self.setup_app(app)
    }

    // Just a spawner for the test, can be ignored.
    pub fn random_pos(&self) -> P {
        P::generate(
            rand::random::<i32>() % self.grid_size,
            rand::random::<i32>() % self.grid_size,
            0,
        )
    }

    // Just a spawner for the test, can be ignored.
    pub fn random_from_pos(&self, pos: &P) -> P {
        let (x, y, z) = pos.coordinates();

        P::generate(
            (x + rand::random::<i32>()).clamp(-self.grid_size, self.grid_size),
            (y + rand::random::<i32>()).clamp(-self.grid_size, self.grid_size),
            z,
        )
    }

    pub fn move_obstacles(
        &self,
    ) -> impl FnMut(Local<Timer>, ResMut<Cache<Pos, Flags>>, Query<&mut Pos, With<Obstacle>>, Res<Time>)
    {
        move |mut timer: Local<Timer>,
              cache: ResMut<Cache<Pos, Flags>>,
              mut query: Query<&mut Pos, With<Obstacle>>,
              time: Res<Time>| {
            if timer.mode() == TimerMode::Once {
                *timer = Timer::from_seconds(5., TimerMode::Repeating);
            }

            if timer.just_finished() {
                timer.reset();

                let Ok(mut write_guard) = cache.write() else {
                    return;
                };

                write_guard.clear();

                for mut pos in query.iter_mut() {
                    *pos = Pos::generate(
                        rand::random::<i32>() % 4 + 1,
                        rand::random::<i32>() % 4 + 1,
                        0,
                    );

                    write_guard.insert(*pos, Flags::default().with_blocks_sight(true));
                }
            } else {
                timer.tick(time.delta());
            }
        }
    }

    fn setup_app(&self, mut app: App) -> App {
        app.add_trajectory::<T, P, N>()
            .add_systems(Startup, (self.spawn_trajectories(), self.spawn_obstacles()))
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
                SystemInformationDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
            ));

        app
    }

    fn spawn_camera(&self) -> impl FnMut(Commands) {
        move |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        }
    }

    fn spawn_trajectories(&self) -> impl FnMut(Commands) {
        let builder = self.clone();

        move |mut commands: Commands| {
            for (trajectory, n) in builder.trajectories.iter() {
                for _ in 0..*n {
                    commands.spawn((trajectory.clone(), trajectory.area.center_pos));
                }
            }
        }
    }

    fn spawn_obstacles(&self) -> impl FnMut(Commands, ResMut<Cache<Pos, Flags>>) {
        let builder = self.clone();

        move |mut commands: Commands, cache: ResMut<Cache<Pos, Flags>>| {
            let Ok(mut write_guard) = cache.write() else {
                return;
            };

            for _ in 0..builder.n_obstacles {
                let pos = Pos::generate(
                    rand::random::<i32>() % builder.grid_size + 1,
                    rand::random::<i32>() % builder.grid_size + 1,
                    0,
                );

                commands.spawn((pos, Obstacle));
                write_guard.insert(pos, Flags::default().with_blocks_sight(true));
            }
        }
    }

    fn draw_grid(&self) -> impl FnMut(Gizmos) {
        let builder = self.clone();

        move |mut gizmos: Gizmos| {
            for x in -builder.grid_size..=builder.grid_size {
                for y in -builder.grid_size..=builder.grid_size {
                    gizmos.rect_2d(
                        P::generate(x, y, 0).into(),
                        0.,
                        tile_size().as_vec2(),
                        Color::WHITE,
                    );
                }
            }
        }
    }
}

pub fn draw_obstacles(mut gizmos: Gizmos, query: Query<&Pos, With<Obstacle>>) {
    for pos in &query {
        gizmos.rect_2d((*pos).into(), 0., tile_size().as_vec2() / 2., Color::RED);
    }
}

pub fn draw_area_of_interest(mut gizmos: Gizmos, player_query: Query<&TrajectoryResult<(), Pos>>) {
    for results in &player_query {
        for pos in results.area_of_interest.iter() {
            gizmos.circle_2d(
                (*pos).into(),
                (tile_size().x / 2) as f32,
                if *pos == Pos::generate(0, 0, 0) {
                    Color::BLUE.as_rgba()
                } else {
                    Color::GREEN.as_rgba()
                },
            );
        }
    }
}

pub fn draw_collisions(mut gizmos: Gizmos, q_result: Query<&TrajectoryResult<(), Pos>>) {
    for result in &q_result {
        for intersection in result.collisions.iter() {
            gizmos.line_2d(
                Vec2::from(intersection.position) - (tile_size() / 2).as_vec2(),
                Vec2::from(intersection.position) + (tile_size() / 2).as_vec2(),
                Color::YELLOW.as_rgba(),
            );
        }
    }
}