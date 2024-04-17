use crate::prelude::*;
use glam::Vec2;
use rstest::rstest;
use std::collections::HashSet;

#[rstest]
#[case((0, 0), (0., -32.))]
#[case((1, 0), (32., -32.))]
#[case((0, 1), (0., 0.))]
#[case((1, 1), (32., 0.))]
#[case((-3, 0), (-96., -32.))]
#[case((0, -3), (0., -128.))]
#[case((-3, -3), (-96., -128.))]
#[case((12300, -5433), (393600., -173888.))]
fn test_position_to_screen_vec2(#[case] input: (i32, i32), #[case] expected: (f32, f32)) {
    let position = TilePosition::new(input.0, input.1, 0);
    let screen_vec2 = Vec2::from(position);
    assert_eq!(screen_vec2, Vec2::new(expected.0, expected.1));
}

#[rstest]
#[case((0., 0.), (0, 1))]
#[case((1., 0.), (1, 1))]
#[case((0., 1.), (0, 1))]
#[case((1., 1.), (1, 1))]
#[case((-26., 0.), (0, 1))]
#[case((0., -26.), (0, 0))]
#[case((-26., -26.), (0, 0))]
#[case((26., 0.), (1, 1))]
#[case((-32., -32.), (-1, 0))]
#[case((-31.999, -31.999), (0, 0))]
#[case((32., 0.), (1, 1))]
#[case((100000., 200000.), (3125, 6251))]
fn test_position_from_screen_vec2(#[case] input: (f32, f32), #[case] expected: (i32, i32)) {
    let position = TilePosition::from(Vec2::new(input.0, input.1));
    assert_eq!(position, TilePosition::new(expected.0, expected.1, 0));
}

#[rstest]
#[case((0, 0), true)]
#[case((i32::MIN, i32::MIN), false)]
#[case((i32::MAX, i32::MAX), false)]
#[case((i32::MIN, 0), false)]
#[case((0, i32::MIN), false)]
#[case((i16::MAX as i32, 0), true)]
#[case((0, i16::MAX as i32), true)]
fn test_validate(#[case] input: (i32, i32), #[case] expected: bool) {
    let position = TilePosition::new(input.0, input.1, 0);
    assert_eq!(position.is_valid(), expected);
}

#[rstest]
#[case((0, 0))]
#[case((1, 0))]
#[case((0, 1))]
#[case((1, 1))]
#[case((-3, 0))]
#[case((0, -3))]
#[case((-3, -3))]
#[case((12300, -5433))]
fn test_reversible(#[case] input: (i32, i32)) {
    let position = TilePosition::new(input.0, input.1, 0);
    let screen = Vec2::from(position);
    let position2 = TilePosition::from(screen);
    assert_eq!(
        position, position2,
        "position: {:?}, screen: {:?}",
        position, screen
    );
}

#[rstest]
#[case(
    TilePosition::new(0, 0, 0),
    TilePosition::new(0, 1, 0),
    &HashSet::from([TilePosition::new(0, 0, 0), TilePosition::new(0, 1, 0), TilePosition::new(0, 2, 0)]),
    true
)]
#[case(
    TilePosition::new(0, 0, 0),
    TilePosition::new(0, 1, 0),
    &HashSet::from([TilePosition::new(0, 1, 0), TilePosition::new(0, 2, 0)]),
    false
)]
#[case(
    TilePosition::new(0, 0, 0),
    TilePosition::new(0, 1, 0),
    &HashSet::from([TilePosition::new(0, 0, 0), TilePosition::new(0, 2, 0)]),
    false
)]
fn test_is_directly_connected(
    #[case] from: TilePosition,
    #[case] to: TilePosition,
    #[case] positions: &HashSet<TilePosition>,
    #[case] expected: bool,
) {
    assert_eq!(from.is_directly_connected(to, positions), expected);
}

#[test]
#[ignore]
#[cfg(feature = "pathfinding")]
fn stress_test_path_finding() {
    use time_test::time_test;

    fn format_number(num: usize) -> String {
        if num >= 1_000_000 {
            format!("{:.1}M", num / 1_000_000)
        } else if num >= 1_000 {
            format!("{:.1}k", num / 1_000)
        } else {
            format!("{}", num)
        }
    }

    let scenarios = [
        (2, 3_000_000usize),
        (3, 1_700_000usize),
        (5, 800_000usize),
        (10, 120_000usize),
        (15, 50_000usize),
        (20, 25_000usize),
        (50, 2_500usize),
        (75, 800usize),
        (100, 400usize),
    ];

    for (distance, iterations) in scenarios.iter() {
        time_test!(format!(
            "\n{} iterations of path finding of distance {}",
            format_number(*iterations),
            distance,
        ));

        for _ in 0..(*iterations) {
            let from = TilePosition::new(rand::random::<i32>(), rand::random::<i32>(), 0);
            let to = TilePosition::new(from.x + *distance, from.y + *distance, 0);

            if from.path_to(to, |_| true, None).is_none() {
                panic!("Path finding failed");
            }
        }
    }
}
