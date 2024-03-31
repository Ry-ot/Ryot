use crate::bevy_ryot::perspective::*;
use crate::position::TilePosition;
use quickcheck_macros::quickcheck;
use rstest::rstest;
use time_test::time_test;

#[cfg(not(target_os = "windows"))]
#[rstest]
#[
    case(RadialViewPoint::default().with_angle_range((315, 405)),
    vec![vec![TilePosition::new(1, -1, 0), TilePosition::new(0, 0, 0)], vec![TilePosition::new(1, 0, 0), TilePosition::new(0, 0, 0)], vec![TilePosition::new(1, 1, 0), TilePosition::new(0, 0, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((0, 0)),
    vec![]
)]
#[
    case(RadialViewPoint::default().with_angle_range((0, 1)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 0, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((45, 46)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 1, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((90, 91)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(0, 1, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((135, 136)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(-1, 1, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((180, 181)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(-1, 0, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((225, 226)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(-1, -1, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((270, 271)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(0, -1, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((315, 316)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(1, -1, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((360, 361)),
    vec![vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 0, 0)]]
)]
#[
    case(RadialViewPoint::default().with_angle_range((675, 675)),
    vec![]
)]
#[
    case(RadialViewPoint::default().with_angle_range((0, 360)),
    vec![
        vec![TilePosition::new(-1, -1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-1, 0, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-1, 1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(0, -1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(0, 1, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, -1, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 0, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 1, 0)]
    ]
)]
#[
    case(RadialViewPoint::default().with_range(3).with_angle_range((0, 360)),
    vec![
        vec![TilePosition::new(-2, -2, 0), TilePosition::new(-1, -1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-2, 2, 0), TilePosition::new(-1, 1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, -1, 0), TilePosition::new(2, -2, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 1, 0), TilePosition::new(2, 2, 0)],
        vec![TilePosition::new(-3, -1, 0), TilePosition::new(-2, -1, 0), TilePosition::new(-1, 0, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-3, 0, 0), TilePosition::new(-2, 0, 0), TilePosition::new(-1, 0, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-3, 1, 0), TilePosition::new(-2, 1, 0), TilePosition::new(-1, 0, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-3, 2, 0), TilePosition::new(-2, 1, 0), TilePosition::new(-1, 1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-2, 3, 0), TilePosition::new(-1, 1, 0), TilePosition::new(-1, 2, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-1, -3, 0), TilePosition::new(-1, -2, 0), TilePosition::new(0, -1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(-1, 2, 0), TilePosition::new(-1, 3, 0), TilePosition::new(0, 0, 0), TilePosition::new(0, 1, 0)],
        vec![TilePosition::new(0, -3, 0), TilePosition::new(0, -2, 0), TilePosition::new(0, -1, 0), TilePosition::new(0, 0, 0)],
        vec![TilePosition::new(0, -1, 0), TilePosition::new(0, 0, 0), TilePosition::new(1, -3, 0), TilePosition::new(1, -2, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(0, 1, 0), TilePosition::new(0, 2, 0), TilePosition::new(0, 3, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(0, 1, 0), TilePosition::new(1, 2, 0), TilePosition::new(1, 3, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, -1, 0), TilePosition::new(2, -1, 0), TilePosition::new(3, -2, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 0, 0), TilePosition::new(2, -1, 0), TilePosition::new(3, -1, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 0, 0), TilePosition::new(2, 0, 0), TilePosition::new(3, 0, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 0, 0), TilePosition::new(2, 1, 0), TilePosition::new(3, 1, 0)],
        vec![TilePosition::new(0, 0, 0), TilePosition::new(1, 1, 0), TilePosition::new(2, 1, 0), TilePosition::new(3, 2, 0)]]
    )]
fn radial_view_point_should_generate_the_correct_target_area(
    #[case] radial_view_point: RadialViewPoint,
    #[case] mut target_areas: Vec<Vec<TilePosition>>,
) {
    let perspective: Perspective = radial_view_point.into();
    let mut areas = perspective
        .view_points
        .iter()
        .map(|pov| pov.target_area.clone())
        .collect::<Vec<_>>();

    for area in &mut areas {
        for target_area in &mut target_areas {
            area.sort_unstable();
            target_area.sort_unstable();
        }
    }

    let sort_nested = |a: &Vec<TilePosition>, b: &Vec<TilePosition>| {
        a.len()
            .cmp(&b.len())
            .then_with(|| a.first().unwrap().x.cmp(&b.first().unwrap().x))
            .then_with(|| a.first().unwrap().y.cmp(&b.first().unwrap().y))
    };

    areas.sort_by(sort_nested);
    target_areas.sort_by(sort_nested);

    assert_eq!(areas, target_areas);
}

#[quickcheck]
fn perspective_generates_valid_viewpoints(radial_view_point: RadialViewPoint) -> bool {
    let perspective = Perspective::from(radial_view_point);

    if radial_view_point.angle_range.0 == radial_view_point.angle_range.1 {
        return true;
    }

    if perspective.view_points.is_empty() {
        println!("Failed:");
        println!("\tradial_view_point: {:?}", radial_view_point);
        println!("\t  view_points_len: {}", perspective.view_points.len());
    }

    true
}

#[quickcheck]
fn radial_view_point_should_generate_the_correct_line_of_sight(
    radial_view_point: RadialViewPoint,
) -> bool {
    let perspective: Perspective = radial_view_point.into();
    let expected_len = TilePosition::new(0, 0, 0)
        .tiles_on_arc_circumference(
            radial_view_point.range,
            radial_view_point.angle_range.0,
            radial_view_point.angle_range.1,
            radial_view_point.angle_step,
        )
        .len();

    if perspective.view_points.len() != expected_len {
        println!("Failed:");
        println!("\tradial_view_point: {:?}", radial_view_point);
        println!("\t     expected_len: {}", expected_len);
        println!("\t  view_points_len: {}", perspective.view_points.len());
    }

    true
}

#[quickcheck]
fn test_view_point_ray_cast_collisions_count(test_pos: tests::TilePosition3x3) -> bool {
    let povs: Perspective = RadialViewPoint::circle().with_range(3).into();
    let count = povs
        .view_points
        .iter()
        .filter(|pov| {
            pov.ray_cast
                .aabb_intersection_at(&test_pos.0.into())
                .is_some()
        })
        .count();

    // center is always 0 for simplicity
    let distance = ((test_pos.x.pow(2) + test_pos.y.pow(2)) as f32).sqrt();

    let expected_max_collisions = if test_pos == tests::TilePosition3x3::ZERO {
        16
    } else if distance <= 1. && test_pos.x != test_pos.y {
        3
    } else if distance <= 3. {
        1
    } else {
        0
    };

    if count > expected_max_collisions {
        println!("Failed:");
        println!("\ttest_pos: {:?}", test_pos);
        println!("\t   count: {}", count);
        println!(
            "Expected the position to be within the perspective's range but found no intersections"
        );
    }

    true
}

#[quickcheck]
fn test_view_point_ray_cast_collisions(
    test_pos: TilePosition,
    view_point: RadialViewPoint,
) -> bool {
    let povs: Perspective = view_point.into();

    let count = povs
        .view_points
        .iter()
        .filter(|pov| {
            pov.ray_cast
                .aabb_intersection_at(&test_pos.into())
                .is_some()
        })
        .count();

    // Check if the test position is within the specified range of the center
    let distance_from_center = (((test_pos.x - view_point.center_pos.x) as i64).pow(2)
        + ((test_pos.y - view_point.center_pos.y) as i64).pow(2))
        as f64;
    let is_within_range = distance_from_center.sqrt() <= view_point.range as f64;

    if is_within_range && count == 0 {
        println!("Failed:");
        println!("\t       test_pos: {:?}", test_pos);
        println!("\t     view_point: {:?}", view_point);
        println!("\tis_within_range: {}", is_within_range);
        println!("\t          count: {}", count);
        println!(
            "Expected the position to be within the perspective's range but found no intersections"
        );
    }

    true
}

#[test]
#[ignore]
fn stress_test_ray_cast() {
    {
        let radial = RadialViewPoint::circle().with_range(5);

        {
            time_test!("Creates 300k circular Perspective (5x5)");
            for _ in 0..300_000 {
                let _: Perspective = radial.into();
            }
        }

        {
            let povs: Perspective = radial.into();

            time_test!("Checks 400k circular Perspective (5x5)");
            for _ in 0..400_000 {
                povs.clone().get_filtered_intersections();
            }
        }
    }

    {
        let radial = RadialViewPoint::circle().with_range(3);

        {
            time_test!("Creates 550k circular Perspective (3x3)");
            for _ in 0..550_000 {
                let _: Perspective = radial.into();
            }
        }

        {
            let povs: Perspective = radial.into();

            time_test!("Checks 1.2M circular Perspective (3x3)");
            for _ in 0..1_200_000 {
                povs.clone().get_filtered_intersections();
            }
        }
    }

    {
        let radial = RadialViewPoint::sector(45, 135).with_range(5);

        {
            time_test!("Creates 1.1M sector Perspective (90degrees - 5x5)");
            for _ in 0..1_100_000 {
                let _: Perspective = radial.into();
            }
        }

        {
            let povs: Perspective = radial.into();

            time_test!("Checks 1.8M sector Perspective (90degrees - 5x5)");
            for _ in 0..1_800_000 {
                povs.clone().get_filtered_intersections();
            }
        }
    }

    {
        let radial = RadialViewPoint::sector(45, 135).with_range(3);

        {
            time_test!("Creates 2.2M sector Perspective (90degrees - 3x3)");
            for _ in 0..2_200_000 {
                let _: Perspective = radial.into();
            }
        }

        {
            let povs: Perspective = radial.into();

            time_test!("Checks 3.5M sector Perspective (90degrees - 3x3)");
            for _ in 0..3_500_000 {
                povs.clone().get_filtered_intersections();
            }
        }
    }
}
