use crate::prelude::SpriteSheets;
use rstest::{fixture, rstest};
use ryot_core::prelude::{ContentRecord, SpriteLayout, SpriteSheet};

#[rstest]
fn test_from_content(#[from(sprite_sheet_set_fixture)] sprite_sheet_set: SpriteSheets) {
    assert_eq!(2, sprite_sheet_set.len());
    assert_eq!(100, sprite_sheet_set[0].first_sprite_id);
    assert_eq!(200, sprite_sheet_set[0].last_sprite_id);
    assert_eq!(300, sprite_sheet_set[1].first_sprite_id);
    assert_eq!(400, sprite_sheet_set[1].last_sprite_id);
}

#[rstest]
fn test_set_get_by_sprite_id(#[from(sprite_sheet_set_fixture)] sprite_sheet_set: SpriteSheets) {
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
fn sprite_sheet_set_fixture() -> SpriteSheets {
    let vec = vec![
        ContentRecord::SpriteSheet(SpriteSheet {
            file: "spritesheet.png".to_string(),
            layout: SpriteLayout::default(),
            first_sprite_id: 100,
            last_sprite_id: 200,
            area: 64,
        }),
        ContentRecord::SpriteSheet(SpriteSheet {
            file: "spritesheet2.png".to_string(),
            layout: SpriteLayout::default(),
            first_sprite_id: 300,
            last_sprite_id: 400,
            area: 64,
        }),
    ];

    vec.into()
}
