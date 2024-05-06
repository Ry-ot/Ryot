# Ryot Ray Casting

[![License](https://img.shields.io/badge/license-GNU%2FAGPL--3.0-blue.svg)](https://github.com/Ry-ot/Ryot?tab=AGPL-3.0-1-ov-file)
[![Crates.io](https://img.shields.io/crates/v/ryot_ray_casting.svg)](https://crates.io/crates/ryot_ray_casting)
[![Downloads](https://img.shields.io/crates/d/ryot_ray_casting.svg)](https://crates.io/crates/ryot_ray_casting)
[![Docs](https://docs.rs/ryot_ray_casting/badge.svg)](https://docs.rs/ryot_ray_casting/latest/ryot_ray_casting/)
[![Discord](https://img.shields.io/discord/528117503952551936.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.com/channels/528117503952551936)

## What is Ryot Ray Casting?

Ryot Ray Casting leverages the concept of [ray casting][ray_casting] to provide a robust ray casting system
for [Bevy][bevy]. It is designed to work seamlessly with Bevy's ECS, enabling developers to implement advanced
trajectory-based mechanics in their games and simulations that require precise straight-line trajectory calculations,
with support for obstacles, line of sight, and other complex interactions. Even though it's optimized for 2D grid-based
environments, it can be easily extended to fit specific game requirements and open-world scenarios. It's part of
[Ryot][ryot] framework, having [Ryot Core][ryot_core] and [Ryot Utils][ryot_utils] as dependencies.

## Ray Casting and Trajectories

Ray Casting is a fundamental concept in game development, enabling complex game mechanics such as projectile motion,
line of sight, fog of war, collision detection, and more. Ray casting is a technique used to simulate trajectories
by tracing rays through a 2D or 3D environment, detecting collisions and interactions along the way. It's widely
used in games to implement realistic physics, lighting effects, and AI behaviors, providing a versatile tool for
creating engaging gameplay experiences.

In the context of Ryot Ray Casting offers a comprehensive solution for implementing ray casting systems in Bevy
projects, providing a flexible and extensible framework for handling ray casting logic. By leveraging the ECS
architecture and seamless Bevy integration, developers can create dynamic ray casting systems that adapt to changing
game conditions and player interactions.

Ryot RayCasting uses [Bevy RayCast3d][bevy_ray_cast] as the underlying ray casting library, for both 2D and 3D
environments.

## Capabilities

- **Seamless Bevy Integration**: Built to work hand-in-hand with Bevy's ECS, offering smooth integration and ensuring
  compatibility with Bevy's event systems.
- **Ray Casting Support**: Utilizes ray casting to simulate trajectories, enabling precise collision detection and
  interaction with obstacles.
- **2D Optimization**: Specially tailored for 2D grid-based navigation, providing robust tools for tile-based and
  open-world game environments.
- **Extensible Architecture**: Designed to be flexible, allowing developers to extend and customize ray casting logic to
  fit specific game requirements.

## Basic Setup

Before setting up the ray casting framework, lets understand the core concepts: `Point`, `RayCastingPoint`, `Navigable`,
`RadialArea<P>` and `Perspective<P>`.

### Point

The Point trait represents a position in the world. It's a core concept of the Ryot ecosystem, that allows you to
integrate your own world representation with Ryot and its spatial algorithms.

### RayCastingPoint

An extension of the Point trait, the RayCastingPoint trait represents a position in the world that can be used to
evaluate ray casting. It's used to generate the bounding box of a given point in space, used to check the point
against the ray cast. This crate uses primarily the aabb3d (axis-aligned bounding box) to represent the bounding box of
a point in space and the ray cast aabb intersection to check if a point is inside a ray or not.

### Navigable

The Navigable trait belongs to `ryot_core` and is used to determine if a point is navigable or not. It's used
to determine if an actor can go through a particular point in the world, for instance if this point is walkable
or not.

Currently, Navigable has two flags: `is_walkable` and `is_flyable`. The first one is used to determine if an actor
can walk through a point, and the second one is used to determine if an actor can fly through a point.

### RadialArea<P>

The RadialArea struct is a descriptive representation of an area in the game world. It contains only primitive and
copyable types, implements Hash and its main purpose is to be used as a descriptive representation of Perspectives,
allowing to cache complex calculations of perspectives and reuse them in the future.

### Perspective<P>

The Perspective struct is a representation of a perspective from a given spectator point. It contains an array of
traversals, which are tuples of RayCast3d and the area traversed by the ray. It's used to represent all the rays that
can be cast from a spectator perspective in a determined scenario/condition.

### Bevy

To integrate `ryot_ray_casting` you need to add a ray casting to your Bevy app. This is done by calling
the `add_ray_casting<T, P, N>`, where `T` is a marker type that represents the ray casting context, `P` a
RayCastingPoint type and `N` is the Navigable type. This method is a builder method on your Bevy app builder.

Here is a basic example:

```rust
use bevy::prelude::*;
use ryot_core::prelude::*;
use ryot_ray_casting::prelude::*;

fn setup<P: RayCastingPoint + Component>(mut commands: Commands) {
    // here we use () as a marker, but in a real scenario you should use a marker type
    // that properly represents the context of the ray casting.
    commands.spawn(RayCasting::<(), P>::default());
}

fn build_app<P: RayCastingPoint + Component>(app: &mut App) -> &mut App {
    app
        .add_plugins(DefaultPlugins)
        .add_ray_casting::<(), P, Flags>()
}
```

## Components

This crate has two main ECS components:

### `RayCasting<T, P>`

This struct is a representation of a ray casting in the game world. It's a component that can be attached to
entities in the ECS, and it's used to represent the ray casting request of an entity given a context T.

This component is attached to entities that require a ray casting computation. It specifies the parameters for the
ray casting evaluation:

- **area**: the radial area that represents the plan over which the rays will be casted.
- **shared_with**: the entities that the propagation of the rays will be shared with.
- **conditions**: the conditions that the plane points P must satisfy, based on a navigable type, to avoid colliding
  with the rays cast.
- **params**: a set of parameters that can be used to customize the ray casting calculation.
    - **max_collisions**: the maximum number of collisions that a ray cast can have before stopping propagating.
    - **reversed**: if the ray should be analysed in reverse order (from the end to the start).
    - **execution_type**: the type of execution that the ray casting should have: once or time based.
- **last_executed_at**: the last time that the ray casting was executed, a flag to determine if it should be
  executed again or not.

It's part of the public API and should be used by the user to trigger ray casting computations.

This component is automatically removed when it no longer meets the execution requirements related to its ExecutionType.

### `RayPropagation<T, P>`

This component is attached to entities that have completed a ray casting computation. It holds the result of the
ray casting evaluation, how the ray propagated over the given plane, represented by:

- **collisions**: the collisions between the ray and the requested plane, meaning that the those points are not
  navigable.
    - **position**: the position where the collision happened.
    - **distance**: the distance from the ray's origin to the collision point.
    - **previous_position**: the previous position that the ray touched before the collision occurred.
    - **pierced**: if the collision was pierced or not, meaning that the ray propagation continued after the collision.
- **area_of_interest**: the area of interest of the ray casting, all points through which the rays cast propagated,
  meaning that the ray influences the world in these points.

This component is attached to entities that have completed a ray casting computation. It's part of the public API and
should be used by the user to check the ray propagation.

## Systems

The ray casting framework is composed of three main systems:

1. `update_intersection_cache<T, P>`: this system updates the cache of intersections represented by a radial area
   present in the RayCasting<T, P> component. This cache is used to speed up the ray casting computation, avoiding
   re-calculating already calculated ray cast aabb intersections.
2. `process_ray_casting<T, P, N>`: the main system of the ray casting framework, it executes the ray casting requests
   present in the ECS, calculating the ray propagation and attaching the results to the entities.
3. `share_results<T, P>`: this system shares the ray propagation results of an entity with the entities that the
   ray casting can be shared with.

There are also two systems that are part of the clean-up process:

1. `remove_stale_results<T, P>`: this system removes the RayPropagation<T, P> from entities that no longer have a
   RayCasting<T, P> component.
2. `remove_stale_requests<T, P>`: this system removes the RayCasting<T, P> requests that are no longer valid.

## Examples

Choose an example to run based on your needs, such as handling multiple entities or dealing with obstacles:

```bash
cargo run --example example_name --features stubs
```

Replace example_name with the name of the example you wish to run.

### Understanding the Examples

Each example included in the library showcases different aspects of the ray casting system:

- **Basic**: Demonstrates a basic complete ray casting use case, with obstacles and different radial areas.
- **Stress Test**: Evaluates the ray casting's performance under high load conditions.

### Building Your Own Scenarios

Leverage the `ExampleBuilder` to customize and create tailored ray casting example/test scenarios:

```rust
fn main() {
    // ExampleBuilder::<T /* Contextual Marker */, P /* RayCastingPoint */, N /* Navigable */>::new()
    // .with_ray_castings(/* array of (ray casting, count) tuples, containing the ray casting to be instantiated and how many */)
    //  .with_obstacles(/* number of obstacles to be instantiated */)
    //      .app() // basic app with visual capabilities
    //      /* add your custom systems, plugins and resources here */
    //      .run();
}
```

## Benchmarks

Performance benchmarks are included to provide insights into the crate's efficiency. The benchmark can be run to
evaluate performance under various conditions:

```bash
cargo bench --features stubs
```

### Results

There are three main benchmarks for the ray casting system: creating the perspectives, executing the ray casting, and
checking navigable points against the ray propagation. The benchmarks cover different scenarios, such as linear,
sectorial and circular areas, with different ranges values.

The following tables provide an overview of the benchmark results for the ray casting system:

#### Creation

| Test Name                          | Type      | Range (Distance) | Time (ns/iter) | Variability (± ns) | Iterations per Second (iters/s) |
|------------------------------------|-----------|------------------|----------------|--------------------|---------------------------------|
| create_linear_range_10             | linear    | 10               | 143            | 3                  | 6,993,007                       |
| create_linear_range_100            | linear    | 100              | 821            | 114                | 1,218,027                       |
| create_linear_range_255            | linear    | 255              | 1,337          | 197                | 747,951                         |
| create_45_degrees_sector_range_10  | radial_45 | 10               | 1,160          | 12                 | 862,069                         |
| create_45_degrees_sector_range_100 | radial_45 | 100              | 17,142         | 409                | 58,358                          |
| create_45_degrees_sector_range_255 | radial_45 | 255              | 29,580         | 830                | 33,822                          |
| create_90_degrees_sector_range_10  | radial_90 | 10               | 2,734          | 112                | 365,632                         |
| create_90_degrees_sector_range_100 | radial_90 | 100              | 34,297         | 883                | 29,159                          |
| create_90_degrees_sector_range_255 | radial_90 | 255              | 59,535         | 2,329              | 16,802                          |
| create_circular_range_3            | circular  | 3                | 1,871          | 28                 | 534,759                         |
| create_circular_range_5            | circular  | 5                | 4,724          | 74                 | 211,640                         |
| create_circular_range_10           | circular  | 10               | 9,819          | 292                | 101,844                         |
| create_circular_range_25           | circular  | 25               | 38,055         | 769                | 26,284                          |
| create_circular_range_50           | circular  | 50               | 81,998         | 2,237              | 12,195                          |
| create_circular_range_100          | circular  | 100              | 143,330        | 2,569              | 6,979                           |
| create_circular_range_255          | circular  | 255              | 277,505        | 40,670             | 3,605                           |

#### Execution

| Test Name                           | Type      | Range (Distance) | Time (ns/iter) | Variability (± ns) | Iterations per Second (iters/s) |
|-------------------------------------|-----------|------------------|----------------|--------------------|---------------------------------|
| execute_linear_range_10             | linear    | 10               | 95             | 1                  | 10,526,316                      |
| execute_linear_range_100            | linear    | 100              | 1,169          | 32                 | 855,048                         |
| execute_linear_range_255            | linear    | 255              | 2,783          | 349                | 359,323                         |
| execute_45_degrees_sector_range_10  | radial_45 | 10               | 602            | 6                  | 1,660,798                       |
| execute_45_degrees_sector_range_100 | radial_45 | 100              | 23,884         | 666                | 41,866                          |
| execute_45_degrees_sector_range_255 | radial_45 | 255              | 60,248         | 897                | 16,600                          |
| execute_90_degrees_sector_range_10  | radial_90 | 10               | 1,227          | 29                 | 815,073                         |
| execute_90_degrees_sector_range_100 | radial_90 | 100              | 47,821         | 972                | 20,914                          |
| execute_90_degrees_sector_range_255 | radial_90 | 255              | 121,197        | 25,467             | 8,250                           |
| execute_circular_range_3            | circular  | 3                | 920            | 77                 | 1,086,957                       |
| execute_circular_range_5            | circular  | 5                | 2,034          | 123                | 491,699                         |
| execute_circular_range_10           | circular  | 10               | 5,074          | 215                | 197,203                         |
| execute_circular_range_25           | circular  | 25               | 27,759         | 923                | 36,020                          |
| execute_circular_range_50           | circular  | 50               | 92,329         | 1,405              | 10,828                          |
| execute_circular_range_100          | circular  | 100              | 199,025        | 3,846              | 5,025                           |
| execute_circular_range_255          | circular  | 255              | 812,311        | 28,281             | 1,231                           |

#### Navigable Collision

| Test Name                                                    | Type      | Range (Distance) | Time (ns/iter) | Variability (± ns) | Iterations per Second (iters/s) |
|--------------------------------------------------------------|-----------|------------------|----------------|--------------------|---------------------------------|
| check_1million_obstacles_against_line_range_15               | linear    | 15               | 44             | 1                  | 22,727,273                      |
| check_1million_obstacles_against_line_range_50               | linear    | 50               | 267            | 8                  | 3,745,318                       |
| check_1million_obstacles_against_line_range_100              | linear    | 100              | 585            | 15                 | 1,709,402                       |
| check_1million_obstacles_against_line_range_255              | linear    | 255              | 1,699          | 38                 | 588,581                         |
| check_1million_obstacles_against_45_degrees_sector_range_15  | radial_45 | 15               | 612            | 21                 | 1,633,987                       |
| check_1million_obstacles_against_45_degrees_sector_range_50  | radial_45 | 50               | 6,660          | 1,812              | 150,150                         |
| check_1million_obstacles_against_45_degrees_sector_range_100 | radial_45 | 100              | 17,077         | 612                | 58,545                          |
| check_1million_obstacles_against_45_degrees_sector_range_255 | radial_45 | 255              | 53,496         | 8,487              | 18,692                          |
| check_1million_obstacles_against_90_degrees_sector_range_15  | radial_90 | 15               | 1,339          | 117                | 746,808                         |
| check_1million_obstacles_against_90_degrees_sector_range_50  | radial_90 | 50               | 13,385         | 405                | 74,706                          |
| check_1million_obstacles_against_90_degrees_sector_range_100 | radial_90 | 100              | 40,414         | 1,383              | 24,742                          |
| check_1million_obstacles_against_90_degrees_sector_range_255 | radial_90 | 255              | 108,612        | 4,525              | 9,209                           |
| check_1million_obstacles_against_circle_range_15             | circular  | 15               | 5,136          | 55                 | 194,748                         |
| check_1million_obstacles_against_circle_range_50             | circular  | 50               | 67,538         | 2,029              | 14,810                          |
| check_1million_obstacles_against_circle_range_100            | circular  | 100              | 161,176        | 3,945              | 6,206                           |
| check_1million_obstacles_against_circle_range_255            | circular  | 255              | 437,340        | 10,785             | 2,287                           |

This README format clearly sections out the features, example usage, and benchmarks, providing a comprehensive guide for
anyone looking to integrate the `ryot_ray_casting` crate into their projects.


[bevy]: https://bevyengine.org/

[ray_casting]: https://en.wikipedia.org/wiki/Ray_casting#:~:text=Ray%20casting%20is%20the%20most,scenes%20to%20two%2Ddimensional%20images.

[bevy_ray_cast]: https://docs.rs/bevy/latest/bevy/math/bounding/struct.RayCast3d.html

[ryot]: https://crates.io/crates/ryot

[ryot_core]: https://crates.io/crates/ryot_core

[ryot_utils]: https://crates.io/crates/ryot_utils