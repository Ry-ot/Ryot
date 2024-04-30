#![feature(test)]

extern crate test;

use bevy::utils::HashMap;
use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use ryot_core::prelude::Point;
use ryot_derive::Pathable;
use ryot_pathfinder::prelude::*;
use test::Bencher;

#[bench]
fn bench_2_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(2));
}

#[bench]
fn bench_3_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(3));
}

#[bench]
fn bench_5_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(5));
}

#[bench]
fn bench_10_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(10));
}

#[bench]
fn bench_15_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(15));
}

#[bench]
fn bench_20_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(20));
}

#[bench]
fn bench_30_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(30));
}

#[bench]
fn bench_50_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(50));
}

#[bench]
fn bench_75_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(75));
}

#[bench]
fn bench_100_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path::<Pos>(100));
}

#[bench]
fn bench_with_obstacles(b: &mut Bencher) {
    let mut obstacles = HashMap::new();
    for _ in 0..200 {
        obstacles.insert(
            Pos::generate(rand::random::<i32>() % 20, rand::random::<i32>() % 20, 0),
            true,
        );
    }

    b.iter(|| {
        find_random_path_with_validator(
            Pos::generate(rand::random::<i32>() % 20, rand::random::<i32>() % 20, 0),
            20,
            |pos| obstacles.get(pos).is_none(),
        )
    });
}

fn find_random_path<P: Pathable + Default>(max_distance: i32) {
    find_random_path_with_validator(
        P::generate(rand::random::<i32>(), rand::random::<i32>(), 0),
        max_distance,
        |_| true,
    );
}

fn find_random_path_with_validator<P: Pathable + Default>(
    from: P,
    max_distance: i32,
    validator: impl Fn(&P) -> bool,
) {
    let to = P::generate(
        from.x() + rand::random::<i32>() % max_distance,
        from.y() + rand::random::<i32>() % max_distance,
        from.z(),
    );

    if from
        .path_to(&PathFindingQuery::new(to), validator)
        .is_none()
    {
        panic!("Path finding failed");
    }
}

/// This is an example on how to use the Pathable trait to define a point for pathfinding.
/// Pos is a simple 3D point with x, y, and z coordinates.
#[derive(Eq, PartialEq, Component, Pathable, Default, Clone, Copy, Debug, Hash)]
pub struct Pos(i32, i32, i32);

impl From<Pos> for Vec2 {
    fn from(pos: Pos) -> Self {
        Vec2::new(pos.0 as f32, pos.1 as f32 - 1.) * 32.
    }
}

/// We are implementing the Pathable trait for Pos, where the coordinates are generated based on
/// the x, y, and z values, and the coordinates are returned as a tuple.
///
/// There is also a default implementation for path_to, which is focused on 2D pathfinding, which
/// for this example is sufficient. If you need 3D pathfinding or other scenarios, you can override
/// this implementation.
impl Point for Pos {
    fn generate(x: i32, y: i32, z: i32) -> Self {
        Pos(x, y, z)
    }

    fn coordinates(&self) -> (i32, i32, i32) {
        (self.0, self.1, self.2)
    }
}
