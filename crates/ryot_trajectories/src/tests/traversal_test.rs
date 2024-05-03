use crate::prelude::*;
use crate::tests;
use crate::tests::*;
use quickcheck_macros::quickcheck;
use rstest::rstest;

#[cfg(not(target_os = "windows"))]
#[rstest]
#[
    case(RadialArea::default().with_angle_range((315, 405)),
    vec![vec![Pos::generate(1, -1, 0), Pos::generate(0, 0, 0)], vec![Pos::generate(1, 0, 0), Pos::generate(0, 0, 0)], vec![Pos::generate(1, 1, 0), Pos::generate(0, 0, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((0, 0)),
    vec![]
)]
#[
    case(RadialArea::default().with_angle_range((0, 1)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(1, 0, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((45, 46)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(1, 1, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((90, 91)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(0, 1, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((135, 136)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(-1, 1, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((180, 181)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(-1, 0, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((225, 226)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(-1, -1, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((270, 271)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(0, -1, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((315, 316)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(1, -1, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((360, 361)),
    vec![vec![Pos::generate(0, 0, 0), Pos::generate(1, 0, 0)]]
)]
#[
    case(RadialArea::default().with_angle_range((675, 675)),
    vec![]
)]
#[
    case(RadialArea::default().with_angle_range((0, 360)),
    vec![
        vec![Pos::generate(-1, -1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-1, 0, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-1, 1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(0, -1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(0, 1, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, -1, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 0, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 1, 0)]
    ]
)]
#[
    case(RadialArea::default().with_range(3).with_angle_range((0, 360)),
    vec![
        vec![Pos::generate(-2, -2, 0), Pos::generate(-1, -1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-2, 2, 0), Pos::generate(-1, 1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, -1, 0), Pos::generate(2, -2, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 1, 0), Pos::generate(2, 2, 0)],
        vec![Pos::generate(-3, -1, 0), Pos::generate(-2, -1, 0), Pos::generate(-1, 0, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-3, 0, 0), Pos::generate(-2, 0, 0), Pos::generate(-1, 0, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-3, 1, 0), Pos::generate(-2, 1, 0), Pos::generate(-1, 0, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-3, 2, 0), Pos::generate(-2, 1, 0), Pos::generate(-1, 1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-2, 3, 0), Pos::generate(-1, 1, 0), Pos::generate(-1, 2, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-1, -3, 0), Pos::generate(-1, -2, 0), Pos::generate(0, -1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(-1, 2, 0), Pos::generate(-1, 3, 0), Pos::generate(0, 0, 0), Pos::generate(0, 1, 0)],
        vec![Pos::generate(0, -3, 0), Pos::generate(0, -2, 0), Pos::generate(0, -1, 0), Pos::generate(0, 0, 0)],
        vec![Pos::generate(0, -1, 0), Pos::generate(0, 0, 0), Pos::generate(1, -3, 0), Pos::generate(1, -2, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(0, 1, 0), Pos::generate(0, 2, 0), Pos::generate(0, 3, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(0, 1, 0), Pos::generate(1, 2, 0), Pos::generate(1, 3, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, -1, 0), Pos::generate(2, -1, 0), Pos::generate(3, -2, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 0, 0), Pos::generate(2, -1, 0), Pos::generate(3, -1, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 0, 0), Pos::generate(2, 0, 0), Pos::generate(3, 0, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 0, 0), Pos::generate(2, 1, 0), Pos::generate(3, 1, 0)],
        vec![Pos::generate(0, 0, 0), Pos::generate(1, 1, 0), Pos::generate(2, 1, 0), Pos::generate(3, 2, 0)]]
    )]
fn radial_area_should_generate_the_correct_target_area(
    #[case] radial_area: RadialArea<Pos>,
    #[case] mut target_areas: Vec<Vec<Pos>>,
) {
    let perspective: Perspective<Pos> = radial_area.into();
    let mut areas = perspective
        .iter()
        .map(|(_, target_area)| target_area.clone())
        .collect::<Vec<_>>();

    for area in &mut areas {
        for target_area in &mut target_areas {
            area.sort_unstable();
            target_area.sort_unstable();
        }
    }

    let sort_nested = |a: &Vec<Pos>, b: &Vec<Pos>| {
        a.len()
            .cmp(&b.len())
            .then_with(|| a.first().unwrap().x().cmp(&b.first().unwrap().x()))
            .then_with(|| a.first().unwrap().y().cmp(&b.first().unwrap().y()))
    };

    areas.sort_by(sort_nested);
    target_areas.sort_by(sort_nested);

    assert_eq!(areas, target_areas);
}

#[quickcheck]
fn perspective_generates_valid_traversals(radial_area: RadialArea<Pos>) -> bool {
    let perspective = Perspective::<Pos>::from(radial_area);

    if radial_area.angle_range.0 == radial_area.angle_range.1 {
        return true;
    }

    if perspective.is_empty() {
        println!("Failed:");
        println!("\tradial_area: {:?}", radial_area);
        println!("\t  traversals_len: {}", perspective.len());
    }

    true
}

#[quickcheck]
fn radial_area_should_generate_the_correct_line_of_sight(radial_area: RadialArea<Pos>) -> bool {
    let perspective: Perspective<Pos> = radial_area.into();
    let expected_len = Pos::generate(0, 0, 0)
        .tiles_on_arc_circumference(
            radial_area.range,
            radial_area.angle_range.0,
            radial_area.angle_range.1,
            radial_area.angle_step,
        )
        .len();

    if perspective.len() != expected_len {
        println!("Failed:");
        println!("\tradial_area: {:?}", radial_area);
        println!("\t     expected_len: {}", expected_len);
        println!("\t  traversals_len: {}", perspective.len());
    }

    true
}

#[quickcheck]
fn test_radial_area_cast_collisions_count(test_pos: tests::Pos3x3) -> bool {
    let povs: Perspective<Pos> = RadialArea::circle().with_range(3).into();
    let count = povs
        .iter()
        .filter(|(ray_cast, _)| ray_cast.aabb_intersection_at(&test_pos.0.into()).is_some())
        .count();

    // center is always 0 for simplicity
    let distance = ((test_pos.x().pow(2) + test_pos.y().pow(2)) as f32).sqrt();

    let expected_max_collisions = if test_pos == tests::Pos3x3::generate(0, 0, 0) {
        16
    } else if distance <= 1. && test_pos.x() != test_pos.y() {
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
fn test_radial_area_ray_cast_collisions(test_pos: Pos, radial_area: RadialArea<Pos>) -> bool {
    let povs: Perspective<Pos> = radial_area.into();

    let count = povs
        .iter()
        .filter(|(ray_cast, _)| ray_cast.aabb_intersection_at(&test_pos.into()).is_some())
        .count();

    // Check if the test position is within the specified range of the center
    let distance_from_center = (((test_pos.x() - radial_area.center_pos.x()) as i64).pow(2)
        + ((test_pos.y() - radial_area.center_pos.y()) as i64).pow(2))
        as f64;
    let is_within_range = distance_from_center.sqrt() <= radial_area.range as f64;

    if is_within_range && count == 0 {
        println!("Failed:");
        println!("\t       test_pos: {:?}", test_pos);
        println!("\t    radial_area: {:?}", radial_area);
        println!("\tis_within_range: {}", is_within_range);
        println!("\t          count: {}", count);
        println!(
            "Expected the position to be within the perspective's range but found no intersections"
        );
    }

    true
}
