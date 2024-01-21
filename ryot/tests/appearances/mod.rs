/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use rstest::rstest;
use ryot::appearances::{ContentType, SpriteSheet};
use ryot::SpriteLayout;
use serde_json::{from_str, to_string};

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
