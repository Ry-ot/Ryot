use super::super::*;
use glam::UVec2;
use rstest::rstest;
use serde_json::to_string;

#[rstest]
#[case(SpriteLayout::OneByOne, 0)]
#[case(SpriteLayout::OneByTwo, 1)]
#[case(SpriteLayout::TwoByOne, 2)]
#[case(SpriteLayout::TwoByTwo, 3)]
fn test_serialize(#[case] layout: SpriteLayout, #[case] expected: u32) {
    let serialized = to_string(&layout).unwrap();
    assert_eq!(serialized, expected.to_string());
}

#[rstest]
#[case(SpriteLayout::OneByOne, 0)]
#[case(SpriteLayout::OneByTwo, 1)]
#[case(SpriteLayout::TwoByOne, 2)]
#[case(SpriteLayout::TwoByTwo, 3)]
fn test_deserialize(#[case] layout: SpriteLayout, #[case] expected: u32) {
    let deserialized: SpriteLayout = serde_json::from_str(&expected.to_string()).unwrap();
    assert_eq!(deserialized, layout);
}

#[test]
fn test_deserialize_invalid() {
    let deserialized: Result<SpriteLayout, _> = serde_json::from_str("4");
    assert!(deserialized.is_err());
}

#[rstest]
#[case(UVec2::new(16, 16))]
#[case(UVec2::new(32, 32))]
#[case(UVec2::new(64, 64))]
#[case(UVec2::new(128, 128))]
#[case(UVec2::new(256, 256))]
#[case(UVec2::new(512, 512))]
fn test_layout_dimensions(#[case] tile_size: UVec2) {
    let sheet_config = SpriteSheetConfig {
        tile_size,
        sheet_size: UVec2::new(1024, 1024),
        compression_config: None,
    };

    assert_eq!(SpriteLayout::OneByOne.get_width(&sheet_config), tile_size.x);
    assert_eq!(
        SpriteLayout::OneByOne.get_height(&sheet_config),
        tile_size.y
    );

    assert_eq!(SpriteLayout::OneByTwo.get_width(&sheet_config), tile_size.x);
    assert_eq!(
        SpriteLayout::OneByTwo.get_height(&sheet_config),
        2 * tile_size.y
    );

    assert_eq!(
        SpriteLayout::TwoByOne.get_width(&sheet_config),
        2 * tile_size.x
    );
    assert_eq!(
        SpriteLayout::TwoByOne.get_height(&sheet_config),
        tile_size.y
    );

    assert_eq!(
        SpriteLayout::TwoByTwo.get_width(&sheet_config),
        2 * tile_size.x
    );
    assert_eq!(
        SpriteLayout::TwoByTwo.get_height(&sheet_config),
        2 * tile_size.y
    );
}
