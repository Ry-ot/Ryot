use crate::components::{Path, PathFindingQuery};
use crate::pathable::Pathable;
use crate::prelude::PathableApp;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;
use ryot_core::prelude::Navigable;
use ryot_utils::cache::Cache;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}

/// A component that represents an on-going pathfinding execution.
#[derive(Component, Copy, Clone)]
pub struct Pathing<P: Pathable>(pub P);

/// A marker component that represents the visual element of an obstacle.
#[derive(Component, Copy, Clone)]
pub struct Obstacle;

/// This is a builder for creating examples for pathfinding.
/// This makes it easier to create examples with different configurations.
#[derive(Copy, Clone)]
pub struct ExampleBuilder<
    P: Pathable + Component + Debug + Into<Vec2>,
    N: Navigable + Copy + Default,
> {
    pub grid_size: i32,
    pub n_entities: usize,
    pub n_obstacles: usize,
    pub max_distance: i32,
    pub sleep: u64,
    pub navigable: N,
    pub query_builder: fn(P) -> PathFindingQuery<P>,
    pub _phantom: std::marker::PhantomData<P>,
}

impl<P: Pathable + Default + Component + Debug + Into<Vec2>, N: Navigable + Copy + Default> Default
    for ExampleBuilder<P, N>
{
    fn default() -> Self {
        Self {
            grid_size: 10,
            n_entities: 1,
            n_obstacles: 0,
            max_distance: 10,
            sleep: 100,
            navigable: N::default(),
            // This is the default query builder, it will create a query that will try to find a
            // path to the same position. Check the `PathFindingQuery` documentation for more
            // information about the available options.
            query_builder: |pos| PathFindingQuery::new(pos).with_success_distance(0.),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[allow(dead_code)]
impl<P: Pathable + Default + Component + Debug + Into<Vec2>, N: Navigable + Copy + Default>
    ExampleBuilder<P, N>
{
    pub fn with_grid_size(mut self, grid_size: i32) -> Self {
        self.grid_size = grid_size;
        self
    }

    pub fn with_n_entities(mut self, n_entities: usize) -> Self {
        self.n_entities = n_entities;
        self
    }

    pub fn with_n_obstacles(mut self, n_obstacles: usize) -> Self {
        self.n_obstacles = n_obstacles;
        self
    }

    pub fn with_max_distance(mut self, max_distance: i32) -> Self {
        self.max_distance = max_distance;
        self
    }

    pub fn with_sleep(mut self, sleep: u64) -> Self {
        self.sleep = sleep;
        self
    }

    pub fn with_navigable(mut self, navigable: N) -> Self {
        self.navigable = navigable;
        self
    }

    pub fn with_query_builder(mut self, query_builder: fn(P) -> PathFindingQuery<P>) -> Self {
        self.query_builder = query_builder;
        self
    }

    /// This method creates a Bevy App with the necessary systems to run the pathfinding example
    /// with visual feedback.
    pub fn drawing_app(&self) -> App {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins)
            .add_systems(Startup, basic_setup)
            .add_systems(First, self.draw_grid())
            .add_systems(Last, draw_actors::<P>)
            .add_systems(Update, draw_target::<P>)
            .add_systems(Update, draw_obstacles::<P>);

        // This is the relevant part of the app, the rest is just visual feedback.
        app.add_pathable::<P, N>()
            .add_systems(Startup, (self.spawn_many(), self.spawn_obstacles(true)))
            .add_systems(Update, (self.start_path(), self.process_path()));

        app
    }

    /// This method creates a Bevy App with the necessary systems to run the pathfinding example
    /// without visual feedback.
    pub fn minimum_app(&self) -> App {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugins(LogPlugin::default());

        // This is the relevant part of the example, the rest is just boilerplate.
        app.add_pathable::<P, N>()
            .add_systems(Startup, (self.spawn_many(), self.spawn_obstacles(false)))
            .add_systems(Update, (self.start_path(), self.process_path()));

        app
    }

    /// This exemplifies how do you trigger a pathfinding execution.
    pub fn start_path(
        &self,
    ) -> impl FnMut(
        Commands,
        Res<Cache<P, N>>,
        Query<(Entity, &P), (Without<Pathing<P>>, Without<Obstacle>)>,
    ) {
        let builder = *self;

        move |mut commands: Commands,
              cache: Res<Cache<P, N>>,
              q_pos: Query<(Entity, &P), (Without<Pathing<P>>, Without<Obstacle>)>| {
            let cache_arc = Arc::clone(&cache);

            // Here we have a system that goes through all entities with a P component that
            // doesn't have a Pathing component (no pathing being executed) and is not an Obstacle.
            for (entity, current_pos) in q_pos.iter() {
                // Random position generator, can be ignored.
                let mut pos = builder.random_from_pos(current_pos);

                // This loop is just a way to avoid spawning an entity on top of an obstacle.
                // Can be ignored for the sake of understanding the pathfinding system.
                while !pos.can_be_navigated(cache_arc.clone()) {
                    pos = builder.random_from_pos(current_pos);
                }

                debug!("Starting path from {:?} to {:?}", current_pos, pos);

                // This is the relevant part of the system, where we insert a PathFindingQuery.
                // Adding this component will trigger the pathfinding system to initiate the
                // async task that will calculate the path.
                commands
                    .entity(entity)
                    .insert(((builder.query_builder)(pos), Pathing(pos)));
            }
        }
    }

    // Just a spawner for the test, can be ignored.
    pub fn spawn_many(&self) -> impl FnMut(Commands) {
        let builder = *self;

        move |mut commands: Commands| {
            for _ in 0..builder.n_entities {
                commands.spawn(builder.random_pos());
            }
        }
    }

    // Just a spawner for the test, can be ignored.
    pub fn spawn_obstacles(&self, draw: bool) -> impl FnMut(Commands, ResMut<Cache<P, N>>) {
        let builder = *self;

        move |mut commands: Commands, cache: ResMut<Cache<P, N>>| {
            let Ok(mut write_guard) = cache.write() else {
                return;
            };

            for _ in 0..builder.n_obstacles {
                let pos = builder.random_pos();
                write_guard.insert(pos, builder.navigable);

                if draw {
                    commands.spawn((pos, Obstacle));
                }
            }
        }
    }

    /// This exemplifies how do you process the result of a pathfinding execution.
    /// The result is stored in a Path<P> component.
    pub fn process_path(
        &self,
    ) -> impl FnMut(
        Commands,
        Query<(Entity, &mut P, &mut Path<P>, Option<&PathFindingQuery<P>>)>,
        Local<HashMap<Entity, Instant>>,
    ) {
        let builder = *self;

        move |mut commands: Commands,
              mut q_paths: Query<(Entity, &mut P, &mut Path<P>, Option<&PathFindingQuery<P>>)>,
              mut last_executed: Local<HashMap<Entity, Instant>>| {
            for (entity, mut pos, mut path, path_query) in q_paths.iter_mut() {
                // Just a simple delay on the consumption, to simulate a thinking (or walking) time.
                // We will consume one position every {sleep} milliseconds.
                if last_executed
                    .entry(entity)
                    .or_insert(Instant::now())
                    .elapsed()
                    < Duration::from_millis(builder.sleep)
                {
                    return;
                }

                // Here we update the last execution time, to keep track of the delay.
                last_executed.insert(entity, Instant::now());

                if path.is_empty() {
                    if path_query.is_none() {
                        // If the path is empty, and there is no new query being executed,
                        // we remove the Pathing component, so the entity stops.
                        commands.entity(entity).remove::<Pathing<P>>();
                    }
                    continue;
                }

                // If there is a next position, we remove it from the path and update the entity's
                // position to the next position.
                *pos = path.remove(0);
            }
        }
    }

    // Just a beautifier for the test, can be ignored.
    pub fn draw_grid(&self) -> impl FnMut(Gizmos) {
        let builder = *self;

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
            (x + rand::random::<i32>() % self.max_distance).clamp(-self.grid_size, self.grid_size),
            (y + rand::random::<i32>() % self.max_distance).clamp(-self.grid_size, self.grid_size),
            z,
        )
    }
}

// Just a spawner for the test, can be ignored.
pub fn basic_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Just a beautifier for the test, can be ignored.
pub fn draw_actors<P: Pathable + Component + Into<Vec2>>(
    mut gizmos: Gizmos,
    q_paths: Query<&P, Without<Obstacle>>,
) {
    for pos in &q_paths {
        gizmos.circle_2d((*pos).into(), (tile_size().x / 2) as f32, Color::BLUE);
    }
}

// Just a beautifier for the test, can be ignored.
pub fn draw_target<P: Pathable + Component + Into<Vec2>>(
    mut gizmos: Gizmos,
    q_targets: Query<&Pathing<P>>,
) {
    for Pathing(pos) in &q_targets {
        gizmos.circle_2d((*pos).into(), (tile_size().x / 2) as f32, Color::GREEN);
    }
}

// Just a beautifier for the test, can be ignored.
pub fn draw_obstacles<P: Pathable + Component + Into<Vec2>>(
    mut gizmos: Gizmos,
    q_obstacles: Query<&P, With<Obstacle>>,
) {
    for pos in &q_obstacles {
        gizmos.rect_2d((*pos).into(), 0., tile_size().as_vec2() / 2., Color::RED);
    }
}
