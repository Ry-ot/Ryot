#![feature(test)]

extern crate test;

use bevy_utils::hashbrown::HashMap;
use ryot_core::prelude::Flags;
use ryot_trajectories::prelude::*;
use ryot_trajectories::stubs::Pos;
use test::Bencher;

macro_rules! trajectories_bench {
    ($radial_area_builder:expr, $create_name:ident, $execute_name:ident) => {
        #[bench]
        fn $create_name(b: &mut Bencher) {
            b.iter(|| create_perspective($radial_area_builder));
        }

        #[bench]
        fn $execute_name(b: &mut Bencher) {
            let perspective = create_perspective($radial_area_builder);
            b.iter(|| execute_perspective(perspective.clone()));
        }
    };
}

macro_rules! trajectories_bench_with_obstacles {
    ($radial_area_builder:expr, $name:ident, $count:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut obstacles = HashMap::new();
            for _ in 0..$count {
                obstacles.insert(
                    Pos::generate(
                        rand::random::<i32>() % 16_000,
                        rand::random::<i32>() % 16_000,
                        0,
                    ),
                    Flags::default().with_blocks_sight(true),
                );
            }

            let trajectory = visible_trajectory::<(), Pos>($radial_area_builder);
            let intersections = execute_perspective(trajectory.area.clone().into());

            b.iter(|| {
                for intersections in intersections.iter() {
                    for pos in intersections {
                        let Some(flags) = obstacles.get(pos) else {
                            continue;
                        };

                        if !trajectory.meets_condition(flags, pos) {
                            continue;
                        }
                    }
                }
            });
        }
    };
}

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(3),
    create_circular_range_3,
    execute_circular_range_3
);

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(5),
    create_circular_range_5,
    execute_circular_range_5
);

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(10),
    create_circular_range_10,
    execute_circular_range_10
);

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(25),
    create_circular_range_25,
    execute_circular_range_25
);

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(50),
    create_circular_range_50,
    execute_circular_range_50
);

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(100),
    create_circular_range_100,
    execute_circular_range_100
);

trajectories_bench!(
    RadialArea::circle().with_range_and_auto_angle_step(255),
    create_circular_range_255,
    execute_circular_range_255
);

trajectories_bench!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(10),
    create_90_degrees_sector_range_10,
    execute_90_degrees_sector_range_10
);

trajectories_bench!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(100),
    create_90_degrees_sector_range_100,
    execute_90_degrees_sector_range_100
);

trajectories_bench!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(255),
    create_90_degrees_sector_range_255,
    execute_90_degrees_sector_range_255
);

trajectories_bench!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(10),
    create_45_degrees_sector_range_10,
    execute_45_degrees_sector_range_10
);

trajectories_bench!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(100),
    create_45_degrees_sector_range_100,
    execute_45_degrees_sector_range_100
);

trajectories_bench!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(255),
    create_45_degrees_sector_range_255,
    execute_45_degrees_sector_range_255
);

trajectories_bench!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(10),
    create_linear_range_10,
    execute_linear_range_10
);

trajectories_bench!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(100),
    create_linear_range_100,
    execute_linear_range_100
);

trajectories_bench!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(255),
    create_linear_range_255,
    execute_linear_range_255
);

trajectories_bench_with_obstacles!(
    RadialArea::circle().with_range_and_auto_angle_step(15),
    check_1million_obstacles_against_circle_range_15,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::circle().with_range_and_auto_angle_step(50),
    check_1million_obstacles_against_circle_range_50,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::circle().with_range_and_auto_angle_step(100),
    check_1million_obstacles_against_circle_range_100,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::circle().with_range_and_auto_angle_step(255),
    check_1million_obstacles_against_circle_range_255,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(15),
    check_1million_obstacles_against_90_degrees_sector_range_15,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(50),
    check_1million_obstacles_against_90_degrees_sector_range_50,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(100),
    check_1million_obstacles_against_90_degrees_sector_range_100,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 90).with_range_and_auto_angle_step(255),
    check_1million_obstacles_against_90_degrees_sector_range_255,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(15),
    check_1million_obstacles_against_45_degrees_sector_range_15,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(50),
    check_1million_obstacles_against_45_degrees_sector_range_50,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(100),
    check_1million_obstacles_against_45_degrees_sector_range_100,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 45).with_range_and_auto_angle_step(255),
    check_1million_obstacles_against_45_degrees_sector_range_255,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(15),
    check_1million_obstacles_against_line_range_15,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(50),
    check_1million_obstacles_against_line_range_50,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(100),
    check_1million_obstacles_against_line_range_100,
    1_000_000
);

trajectories_bench_with_obstacles!(
    RadialArea::sector(0, 1).with_range_and_auto_angle_step(255),
    check_1million_obstacles_against_line_range_255,
    1_000_000
);

fn create_perspective(radial_area: RadialArea<Pos>) -> Perspective<Pos> {
    radial_area.into()
}

fn execute_perspective(perspective: Perspective<Pos>) -> Vec<Vec<Pos>> {
    perspective.get_intersections()
}
