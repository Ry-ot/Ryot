use rstest::rstest;
use ryot::SpriteLayout;
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
#[case(ryot::Rect::new(16, 16))]
#[case(ryot::Rect::new(32, 32))]
#[case(ryot::Rect::new(64, 64))]
#[case(ryot::Rect::new(128, 128))]
#[case(ryot::Rect::new(256, 256))]
#[case(ryot::Rect::new(512, 512))]
fn test_layout_dimensions(#[case] tile_size: ryot::Rect) {
    let sheet_config = ryot::SpriteSheetConfig {
        tile_size,
        sheet_size: ryot::Rect::new(1024, 1024),
        compression_config: None,
    };

    assert_eq!(
        SpriteLayout::OneByOne.get_width(&sheet_config),
        tile_size.width
    );
    assert_eq!(
        SpriteLayout::OneByOne.get_height(&sheet_config),
        tile_size.height
    );

    assert_eq!(
        SpriteLayout::OneByTwo.get_width(&sheet_config),
        tile_size.width
    );
    assert_eq!(
        SpriteLayout::OneByTwo.get_height(&sheet_config),
        2 * tile_size.height
    );

    assert_eq!(
        SpriteLayout::TwoByOne.get_width(&sheet_config),
        2 * tile_size.width
    );
    assert_eq!(
        SpriteLayout::TwoByOne.get_height(&sheet_config),
        tile_size.height
    );

    assert_eq!(
        SpriteLayout::TwoByTwo.get_width(&sheet_config),
        2 * tile_size.width
    );
    assert_eq!(
        SpriteLayout::TwoByTwo.get_height(&sheet_config),
        2 * tile_size.height
    );
}
