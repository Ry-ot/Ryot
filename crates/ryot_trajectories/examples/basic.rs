//! Shows how to do the bare minimum to use trajectories within ryot
use bevy::prelude::*;
use ryot_core::game::Point;
use ryot_core::prelude::Flags;
use ryot_trajectories::prelude::*;
use ryot_trajectories::stubs::*;
use std::time::Duration;

fn main() {
    let builder: ExampleBuilder<(), Pos, Flags> = ExampleBuilder::default();

    builder
        .clone()
        .with_trajectories(vec![(
            visible_trajectory(RadialArea::default())
                .with_execution_type(ExecutionType::TimeBased(Duration::from_millis(10))),
            1,
        )])
        .with_obstacles(6)
        .app()
        .add_systems(
            Update,
            (
                draw_area_of_interest,
                draw_collisions,
                update_trajectory,
                change_area_type,
                builder.move_obstacles(),
                draw_obstacles,
            ),
        )
        .init_resource::<AreaType>()
        .run();
}

#[derive(Resource, Default)]
pub enum AreaType {
    Circular,
    Radial(u16),
    #[default]
    Linear,
}

impl AreaType {
    pub fn get_radial_area<P: Point>(&self, angle: u16) -> RadialArea<P> {
        match self {
            AreaType::Circular => RadialArea::circle(),
            AreaType::Radial(angle_range) => RadialArea::sector(angle, angle + angle_range),
            AreaType::Linear => RadialArea::sector(angle, angle + 1),
        }
        .with_range_and_auto_angle_step(10)
    }
}

fn change_area_type(mut area_type: ResMut<AreaType>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match *area_type {
            AreaType::Circular => *area_type = AreaType::Radial(180),
            AreaType::Radial(180) => *area_type = AreaType::Radial(90),
            AreaType::Radial(90) => *area_type = AreaType::Radial(45),
            AreaType::Radial(45) => *area_type = AreaType::Radial(30),
            AreaType::Radial(30) => *area_type = AreaType::Radial(15),
            AreaType::Radial(_) => *area_type = AreaType::Linear,
            AreaType::Linear => *area_type = AreaType::Circular,
        }
    }
}

fn update_trajectory(
    time: Res<Time>,
    area_type: Res<AreaType>,
    mut query: Query<&mut TrajectoryRequest<(), Pos>>,
    mut cache: Local<f32>,
) {
    *cache += 50f32 * time.delta_seconds();

    for mut trajectory in &mut query.iter_mut() {
        trajectory.area = area_type.get_radial_area((*cache % 360.) as u16);
    }
}
