use crate::prelude::{ContentType, SpriteLayout, SpriteSheetData, SpriteSheetDataSet};
use glam::UVec2;
use rstest::{fixture, rstest};
use serde_json::{from_str, to_string};

#[rstest]
fn test_has_sprite(#[from(sprite_sheet_fixture)] sprite_sheet: SpriteSheetData) {
    assert!(sprite_sheet.has_sprite(100));
    assert!(sprite_sheet.has_sprite(200));
    assert!(!sprite_sheet.has_sprite(99));
    assert!(!sprite_sheet.has_sprite(201));
}

#[rstest]
fn test_get_sprite_index(#[from(sprite_sheet_fixture)] sprite_sheet: SpriteSheetData) {
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
    let sprite_sheet = SpriteSheetData {
        file: "spritesheet.png".to_string(),
        layout,
        first_sprite_id: 100,
        last_sprite_id: 200,
        area: 64,
    };

    assert_eq!(expected, sprite_sheet.get_tile_size(&UVec2::new(32, 32)));
}

#[fixture]
fn sprite_sheet_fixture() -> SpriteSheetData {
    SpriteSheetData {
        file: "spritesheet.png".to_string(),
        layout: SpriteLayout::OneByOne,
        first_sprite_id: 100,
        last_sprite_id: 200,
        area: 64,
    }
}

#[rstest]
fn test_from_content(#[from(sprite_sheet_set_fixture)] sprite_sheet_set: SpriteSheetDataSet) {
    assert_eq!(2, sprite_sheet_set.len());
    assert_eq!(100, sprite_sheet_set[0].first_sprite_id);
    assert_eq!(200, sprite_sheet_set[0].last_sprite_id);
    assert_eq!(300, sprite_sheet_set[1].first_sprite_id);
    assert_eq!(400, sprite_sheet_set[1].last_sprite_id);
}

#[rstest]
fn test_set_get_by_sprite_id(
    #[from(sprite_sheet_set_fixture)] sprite_sheet_set: SpriteSheetDataSet,
) {
    assert_eq!(
        100,
        sprite_sheet_set
            .get_by_sprite_id(100)
            .unwrap()
            .first_sprite_id
    );
    assert_eq!(
        200,
        sprite_sheet_set
            .get_by_sprite_id(200)
            .unwrap()
            .last_sprite_id
    );
    assert_eq!(
        300,
        sprite_sheet_set
            .get_by_sprite_id(300)
            .unwrap()
            .first_sprite_id
    );
    assert_eq!(
        400,
        sprite_sheet_set
            .get_by_sprite_id(400)
            .unwrap()
            .last_sprite_id
    );
    assert_eq!(None, sprite_sheet_set.get_by_sprite_id(99));
    assert_eq!(None, sprite_sheet_set.get_by_sprite_id(201));
    assert_eq!(None, sprite_sheet_set.get_by_sprite_id(299));
    assert_eq!(None, sprite_sheet_set.get_by_sprite_id(401));
}

#[fixture]
fn sprite_sheet_set_fixture() -> SpriteSheetDataSet {
    let vec = vec![
        ContentType::Sprite(SpriteSheetData {
            file: "spritesheet.png".to_string(),
            layout: SpriteLayout::default(),
            first_sprite_id: 100,
            last_sprite_id: 200,
            area: 64,
        }),
        ContentType::Sprite(SpriteSheetData {
            file: "spritesheet2.png".to_string(),
            layout: SpriteLayout::default(),
            first_sprite_id: 300,
            last_sprite_id: 400,
            area: 64,
        }),
    ];

    vec.into()
}

#[rstest]
#[case(ContentType::Unknown, r#"{"type":"unknown"}"#)]
#[case(
        ContentType::Sprite(SpriteSheetData {
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
    r#"{"type":"appearances","file":"appearances.dat"}"#,
    ContentType::Unknown
)]
#[case(
    r#"{"type":"staticdata","file":"staticdata.dat"}"#,
    ContentType::Unknown
)]
#[case(
    r#"{"type":"staticmapdata","file":"staticmapdata.dat"}"#,
    ContentType::Unknown
)]
#[case(r#"{"type":"map","file":"map.otbm"}"#, ContentType::Unknown)]
#[case(
        r#"{"type":"sprite","file":"spritesheet.png","spritetype":0,"firstspriteid":100,"lastspriteid":200,"area":64}"#,
        ContentType::Sprite(SpriteSheetData {
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
