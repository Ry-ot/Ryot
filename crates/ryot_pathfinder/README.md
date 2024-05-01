# Ryot PathFinder

[![License](https://img.shields.io/badge/license-GNU%2FAGPL--3.0-blue.svg)](https://github.com/Ry-ot/Ryot?tab=AGPL-3.0-1-ov-file)
[![Crates.io](https://img.shields.io/crates/v/ryot_pathfinder.svg)](https://crates.io/crates/ryot_pathfinder)
[![Downloads](https://img.shields.io/crates/d/ryot_pathfinder.svg)](https://crates.io/crates/ryot_pathfinder)
[![Docs](https://docs.rs/ryot_pathfinder/badge.svg)](https://docs.rs/ryot_pathfinder/latest/ryot_pathfinder/)
[![Discord](https://img.shields.io/discord/528117503952551936.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.com/channels/528117503952551936)

## What is Ryot PathFinder?

Ryot PathFinder is a high-performance, asynchronous implementation of [Pathfinding][path_finding] for [Bevy][bevy]. It
is designed to work seamlessly with Bevy's ECS, providing robust pathfinding capabilities for games and simulations that
demand dynamic navigation. Even though it's optimized for 2D grid-based environments, it can be easily extended to fit
specific game requirements and open-world scenarios. It's part of [Ryot][ryot] framework, having [Ryot Core][ryot_core]
and [Ryot Utils][ryot_utils] as dependencies.

## Pathfinding

Pathfinding is a fundamental concept in game development, enabling entities to navigate through complex environments
efficiently. It involves finding the shortest path between two points, considering obstacles, and optimizing the route
based on various criteria. Pathfinding algorithms are essential for creating engaging gameplay experiences, enabling
characters to move intelligently and interact with the game world effectively.

In this context, Ryot PathFinder offers a comprehensive solution for implementing pathfinding systems in Bevy projects,
providing a flexible and extensible framework for handling navigation logic. By leveraging asynchronous operations and
seamless Bevy integration, developers can create dynamic pathfinding systems that adapt to changing game conditions and
player interactions.

PathFinder uses [Pathfinding][path_finding] as the underlying pathfinding library. For the 2D default implementation,
it uses the [A*][astar] algorithm to calculate the shortest path between two points.

## Capabilities

- **Seamless Bevy Integration**: Built to work hand-in-hand with Bevy's ECS, offering smooth integration and ensuring
  compatibility with Bevy's event systems.
- **Asynchronous Operations**: Utilizes async tasks to maintain high performance and responsiveness, particularly
  suitable for scenarios with extensive pathfinding demands.
- **2D Optimization**: Specially tailored for 2D grid-based navigation, providing robust tools for tile-based and
  open-world game environments.
- **Extensible Architecture**: Designed to be flexible, allowing developers to extend and customize pathfinding logic to
  fit specific game requirements.

## Basic Setup

Before setting up the pathfinder, lets understand the core concepts of the pathfinder: `Point`, `Pathable<P>`
and `Navigable<N>`.

### Point

The Point trait represents a position in the world. It's a core concept of the Ryot ecosystem, that allows you to
integrate your own world representation with Ryot and its spatial algorithms.

### Pathable

The Pathable trait represents the position in the world. It's used to calculate the path between two points.
Pathable extends Point, provides an interface to calculate the path between two Points in space and to check
if the pathable can be navigated against a given Navigable.

### Navigable

The Navigable trait belongs to `ryot_core` and is used to determine if a point is navigable or not. It's used
to determine if an actor can go through a particular point in the world, for instance if this point is walkable
or not.

Currently, Navigable has two flags: `is_walkable` and `is_flyable`. The first one is used to determine if an actor
can walk through a point, and the second one is used to determine if an actor can fly through a point.

### Bevy

To integrate `ryot_pathfinder` you need to add a pathable to your Bevy app. This is done by calling the `add_pathable`
method on your Bevy app builder. A pathable is represented by a pair of Pathable and Navigable implementations <P, N>.

Here is a basic example:

```rust
use bevy::prelude::*;
use ryot_pathfinder::prelude::*;

fn setup<P: Pathable + Default>(mut commands: Commands) {
    commands.spawn(PathFindingQuery::<P>::default());
}

fn build_app<P: Pathable + Component>(app: &mut App) -> &mut App {
    app
        .add_plugins(DefaultPlugins)
        .add_pathable::<P, ()>()
}
```

## Components

The `PathFindingQuery` has three main components:

### `PathFindingQuery<P>`

This component is attached to entities that require a pathfinding computation. It specifies the parameters for the
pathfinding algorithm:

- **to**: target position.
- **cardinal_cost**: cost of moving in the cardinal directions.
- **diagonal_cost**: cost of moving in the diagonal directions.
- **success_distance**: distance from the target position that is considered a successful pathfinding computation.
- **timeout**: maximum time in seconds that the pathfinding algorithm can run before returning None.

It's part of the public API and should be used by the user to trigger pathfinding computations.

### `PathFindingTask<P>`

This component is attached to entities that are currently computing a pathfinding algorithm. It holds the future (or
task) that will return the path result. It's internal to ryot_pathfinder and cannot be used by the user.

### `Path<P>`

This component is attached to entities that have completed a pathfinding computation. It holds the result of the
path finding computation, the actual path that the entity can follow to reach the target.

It's part of the public API and should be used by the user to move the entity along the path.

## Workflow

The flow happens in four steps:

1. `PathFindingQuery<P>` is added to the entity, specifying the parameters for the pathfinding algorithm.
2. `PathFindingQuery<P>` is consumed by `trigger_path_finding_tasks` system, which creates a `PathFindingTask<P>` and
   attaches it to the entity.
3. `PathFindingTask<P>` is executed asynchronously, and once it completes, the `handle_path_finding_tasks` system
   creates
   a `Path<P>` component and attaches it to the entity.
4. The `Path<P>` can now be consumed by the user to move the entity along the path.

To better understand how `PathFindingQuery<P>`, `PathFindingTask<P>`, and `Path<P>` interact, check the diagram below:
![](workflow.png)

## Examples

Choose an example to run based on your needs, such as handling multiple entities or dealing with obstacles:

```bash
cargo run --example example_name
```

Replace example_name with the name of the example you wish to run.

### Understanding the Examples

Each example included in the library showcases different aspects of the pathfinding system:

- **Basic**: Demonstrates the simplest form of pathfinding.
- **Multiple**: Handles multiple actors navigating simultaneously.
- **Obstacles**: Integrates static obstacles within pathfinding calculations.

### Experimenting with Advanced Scenarios

As you grow more comfortable, explore more complex examples:

- **Simulation**: Small simulation of multiple actors navigating through multiple obstacles in a grid-based environment.
- **Stress Test**: Evaluates the pathfinder's performance under high load conditions.

### Building Your Own Scenarios

Leverage the `ExampleBuilder` to customize and create tailored pathfinding example/test scenarios:

```rust
fn main() {
    // ExampleBuilder::<P/* custom pathable type */, N/* custom navigable type */>::new()
    //     .with_grid_size(/* custom dimension of a squared grid, default 10 */)
    //     .with_n_entities(/* number of actors to spawn, default 10 */)
    //     .with_n_obstacles(/* number of obstacles to spawn, default 0 */)
    //     .with_max_distance(/* maximum distance to calculate pathfinding, default 10 */)
    //     .with_sleep(/* sleep time (ms) between consuming pathfinding results, default 100 */)
    //     .with_navigable(/* custom navigable (N) value for obstacles, default N::default() */)
    //     .with_query_builder(/* custom query builder, default PathFindingQuery::new(pos).with_success_distance(0.) */)
    //     .drawing_app()
    //     .run();
}
```

## Benchmarks

Performance benchmarks are included to provide insights into the crate's efficiency. The benchmark bench can be run to
evaluate performance under various conditions:

```bash
cargo bench
```

### Results

| Test Name                    | Size | Time (ns/iter) | Variability (± ns) | Iterations per Second (iters/s) |
|------------------------------|------|----------------|--------------------|---------------------------------|
| bench_2_sized_path_finding   | 2    | 137            | 2                  | 7,299,270                       |
| bench_3_sized_path_finding   | 3    | 166            | 2                  | 6,024,096                       |
| bench_5_sized_path_finding   | 5    | 285            | 6                  | 3,508,772                       |
| bench_10_sized_path_finding  | 10   | 1,131          | 73                 | 884,148                         |
| bench_15_sized_path_finding  | 15   | 3,272          | 188                | 305,709                         |
| bench_20_sized_path_finding  | 20   | 7,139          | 691                | 140,088                         |
| bench_with_obstacles         | 20   | 40,044         | 4,442              | 24,973                          |
| bench_30_sized_path_finding  | 30   | 21,406         | 831                | 46,726                          |
| bench_50_sized_path_finding  | 50   | 81,027         | 5,558              | 12,343                          |
| bench_75_sized_path_finding  | 75   | 225,945        | 86,525             | 4,424                           |
| bench_100_sized_path_finding | 100  | 445,268        | 241,426            | 2,246                           |

This README format clearly sections out the features, example usage, and benchmarks, providing a comprehensive guide for
anyone looking to integrate the `ryot_pathfinder` crate into their projects.


[astar]: https://docs.rs/pathfinding/latest/pathfinding/directed/astar/index.html

[bevy]: https://bevyengine.org/

[path_finding]: https://github.com/evenfurther/pathfinding

[ryot]: https://crates.io/crates/ryot

[ryot_core]: https://crates.io/crates/ryot_core

[ryot_utils]: https://crates.io/crates/ryot_utils