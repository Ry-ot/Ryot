use crate::position::TilePosition;
use glam::Vec2;
use rstest::rstest;

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
