use super::*;
use rstest::{fixture, rstest};
use serde_json::{from_str, to_string};

#[rstest]
fn test_has_sprite(#[from(sprite_sheet_fixture)] sprite_sheet: SpriteSheet) {
    assert!(sprite_sheet.has_sprite(100));
    assert!(sprite_sheet.has_sprite(200));
    assert!(!sprite_sheet.has_sprite(99));
    assert!(!sprite_sheet.has_sprite(201));
}

#[rstest]
fn test_get_sprite_index(#[from(sprite_sheet_fixture)] sprite_sheet: SpriteSheet) {
    assert_eq!(Some(0), sprite_sheet.get_sprite_index(100));
    assert_eq!(Some(99), sprite_sheet.get_sprite_index(199));
    assert_eq!(Some(100), sprite_sheet.get_sprite_index(200));
    assert_eq!(None, sprite_sheet.get_sprite_index(99));
    assert_eq!(None, sprite_sheet.get_sprite_index(201));
}

#[rstest]
#[case(SpriteLayout::OneByOne, UVec2::new(32, 32))]
#[case(SpriteLayout::OneByTwo, UVec2::new(32, 64))]
#[case(SpriteLayout::TwoByOne, UVec2::new(64, 32))]
#[case(SpriteLayout::TwoByTwo, UVec2::new(64, 64))]
fn test_get_tile_size(#[case] layout: SpriteLayout, #[case] expected: UVec2) {
    let sprite_sheet = SpriteSheet {
        file: "spritesheet.png".to_string(),
        layout,
        first_sprite_id: 100,
        last_sprite_id: 200,
        area: 64,
    };

    assert_eq!(
        expected,
        sprite_sheet.get_tile_size(&SpriteSheetConfig::cip_sheet())
    );
}

#[rstest]
fn test_get_columns_count(#[from(sprite_sheet_fixture)] sprite_sheet: SpriteSheet) {
    assert_eq!(
        12,
        sprite_sheet.get_columns_count(&SpriteSheetConfig::cip_sheet())
    );
}

#[rstest]
fn test_get_rows_count(#[from(sprite_sheet_fixture)] sprite_sheet: SpriteSheet) {
    assert_eq!(
        12,
        sprite_sheet.get_rows_count(&SpriteSheetConfig::cip_sheet())
    );
}

#[fixture]
fn sprite_sheet_fixture() -> SpriteSheet {
    SpriteSheet {
        file: "spritesheet.png".to_string(),
        layout: SpriteLayout::OneByOne,
        first_sprite_id: 100,
        last_sprite_id: 200,
        area: 64,
    }
}

#[rstest]
#[case(
        ContentType::Appearances { file: "appearances.dat".to_string(), version: 1 },
        r#"{"type":"appearances","file":"appearances.dat","version":1}"#
    )]
#[case(
        ContentType::StaticData { file: "staticdata.dat".to_string() },
        r#"{"type":"staticdata","file":"staticdata.dat"}"#
    )]
#[case(
        ContentType::StaticMapData { file: "staticmapdata.dat".to_string() },
        r#"{"type":"staticmapdata","file":"staticmapdata.dat"}"#
    )]
#[case(
        ContentType::Map { file: "map.otbm".to_string() },
        r#"{"type":"map","file":"map.otbm"}"#
    )]
#[case(
        ContentType::Sprite(SpriteSheet {
            file: "spritesheet.png".to_string(),
            layout: SpriteLayout::OneByOne,
            first_sprite_id: 100,
            last_sprite_id: 200,
            area: 64,
        }),
        r#"{"type":"sprite","file":"spritesheet.png","spritetype":0,"firstspriteid":100,"lastspriteid":200,"area":64}"#
    )]
fn test_serialize_content_type(#[case] content: ContentType, #[case] expected_json: &str) {
    assert_eq!(to_string(&content).unwrap(), expected_json);
}

#[rstest]
#[case(
        r#"{"type":"appearances","file":"appearances.dat","version":1}"#,
        ContentType::Appearances { file: "appearances.dat".to_string(), version: 1 }
    )]
#[case(
        r#"{"type":"staticdata","file":"staticdata.dat"}"#,
        ContentType::StaticData { file: "staticdata.dat".to_string() }
    )]
#[case(
        r#"{"type":"staticmapdata","file":"staticmapdata.dat"}"#,
        ContentType::StaticMapData { file: "staticmapdata.dat".to_string() }
    )]
#[case(
        r#"{"type":"map","file":"map.otbm"}"#,
        ContentType::Map { file: "map.otbm".to_string() }
    )]
#[case(
        r#"{"type":"sprite","file":"spritesheet.png","spritetype":0,"firstspriteid":100,"lastspriteid":200,"area":64}"#,
        ContentType::Sprite(SpriteSheet {
            file: "spritesheet.png".to_string(),
            layout: SpriteLayout::OneByOne,
            first_sprite_id: 100,
            last_sprite_id: 200,
            area: 64,
        })
    )]
fn test_deserialize_content_type(#[case] json: &str, #[case] expected_content: ContentType) {
    assert_eq!(from_str::<ContentType>(json).unwrap(), expected_content);
}
