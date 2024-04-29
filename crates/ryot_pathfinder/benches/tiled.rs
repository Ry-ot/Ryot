#![feature(test)]

extern crate test;

use bevy::utils::HashMap;
use ryot_pathfinder::prelude::*;
use ryot_tiled::prelude::*;
use test::Bencher;

#[bench]
fn bench_2_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(2));
}

#[bench]
fn bench_3_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(3));
}

#[bench]
fn bench_5_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(5));
}

#[bench]
fn bench_10_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(10));
}

#[bench]
fn bench_15_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(15));
}

#[bench]
fn bench_20_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(20));
}

#[bench]
fn bench_30_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(30));
}

#[bench]
fn bench_50_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(50));
}

#[bench]
fn bench_75_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(75));
}

#[bench]
fn bench_100_sized_path_finding(b: &mut Bencher) {
    b.iter(|| find_random_path(100));
}

#[bench]
fn bench_with_obstacles(b: &mut Bencher) {
    let mut obstacles = HashMap::new();
    for _ in 0..200 {
        obstacles.insert(
            TilePosition::new(rand::random::<i32>() % 20, rand::random::<i32>() % 20, 0),
            true,
        );
    }

    b.iter(|| {
        find_random_path_with_validator(
            TilePosition::new(rand::random::<i32>() % 20, rand::random::<i32>() % 20, 0),
            20,
            |pos| obstacles.get(pos).is_none(),
        )
    });
}

fn find_random_path(max_distance: i32) {
    find_random_path_with_validator(
        TilePosition::new(rand::random::<i32>(), rand::random::<i32>(), 0),
        max_distance,
        |_| true,
    );
}

fn find_random_path_with_validator(
    from: TilePosition,
    max_distance: i32,
    validator: impl Fn(&TilePosition) -> bool,
) {
    let to = TilePosition::new(
        from.x + rand::random::<i32>() % max_distance,
        from.y + rand::random::<i32>() % max_distance,
        0,
    );

    if from
        .path_to(&PathFindingQuery::new(to), validator)
        .is_none()
    {
        panic!("Path finding failed");
    }
}
