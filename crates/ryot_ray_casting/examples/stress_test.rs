use bevy::prelude::*;
use ryot_core::prelude::Flags;
use ryot_ray_casting::prelude::*;
use ryot_ray_casting::stubs::{ExampleBuilder, Pos};

fn main() {
    let builder: ExampleBuilder<(), Pos, Flags> = ExampleBuilder::default();

    builder
        .clone()
        .with_ray_castings(vec![
            (
                visible_ray_casting(
                    RadialArea::circle()
                        .with_range(15)
                        .with_center_pos(builder.random_pos()),
                )
                .with_execution_type(ExecutionType::every_in_ms(500)),
                10_000,
            ),
            (
                visible_ray_casting(
                    RadialArea::default()
                        .with_range(1)
                        .with_center_pos(builder.random_pos())
                        .with_angle_range((0, 1)),
                )
                .with_execution_type(ExecutionType::every_in_ms(500)),
                1_000_000,
            ),
            (
                visible_ray_casting(
                    RadialArea::circle()
                        .with_range(3)
                        .with_center_pos(builder.random_pos()),
                )
                .with_execution_type(ExecutionType::every_in_ms(500)),
                50_000,
            ),
            (
                visible_ray_casting(
                    RadialArea::default()
                        .with_range(15)
                        .with_center_pos(builder.random_pos())
                        .with_angle_range((0, 1)),
                )
                .with_execution_type(ExecutionType::every_in_ms(500)),
                100_000,
            ),
        ])
        .with_obstacles(10_000_000)
        .minimal_app()
        .add_systems(Update, process_intersections)
        .run();
}

fn process_intersections(player_query: Query<&RayPropagation<(), Pos>>) {
    for intersections in player_query.iter() {
        for _ in intersections.area_of_interest.iter() {}
    }
}
