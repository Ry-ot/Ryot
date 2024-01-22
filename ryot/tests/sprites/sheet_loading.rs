use image::RgbaImage;
use rstest::{fixture, rstest};
use ryot::{
    decompress_sprite_sheet, decompress_sprite_sheets, load_sprite_sheet_image, ContentConfigs,
    SpriteSheetConfig, SPRITE_SHEET_FOLDER,
};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[rstest]
fn test_load_sprite_sheet_image_for_compressed_image(#[from(image_fixture)] expected: RgbaImage) {
    let sheet_config = SpriteSheetConfig::cip_sheet();

    let img =
        load_sprite_sheet_image(&PathBuf::from("tests/fixtures/1.bmp.lzma"), sheet_config).unwrap();

    assert_eq!(img.dimensions(), (384, 384));
    assert_eq!(img, expected);
}

#[rstest]
fn test_load_sprite_sheet_image_for_uncompressed_image(#[from(image_fixture)] expected: RgbaImage) {
    let sheet_config = SpriteSheetConfig {
        tile_size: glam::UVec2::new(32, 32),
        sheet_size: glam::UVec2::new(384, 384),
        compression_config: None,
        encoding_config: None,
    };

    let img = load_sprite_sheet_image(&PathBuf::from("tests/fixtures/expected.png"), sheet_config)
        .unwrap();

    assert_eq!(img.dimensions(), (384, 384));
    assert_eq!(img, expected);
}

#[rstest]
fn test_decompress_sprite_sheet(#[from(image_fixture)] expected: RgbaImage) {
    let sheet_config = SpriteSheetConfig::cip_sheet();

    decompress_sprite_sheet(
        "2.bmp.lzma",
        &PathBuf::from("tests/fixtures"),
        &PathBuf::from("tests/fixtures/sprite-sheets"),
        sheet_config,
    );

    let expected_path = PathBuf::from("tests/fixtures/sprite-sheets/2.png");
    assert!(expected_path.exists());

    assert_eq!(load_fixture_image(&expected_path).unwrap(), expected);
}

#[rstest]
fn test_decompress_sprite_sheets(#[from(image_fixture)] expected: RgbaImage) {
    let content_config = ContentConfigs {
        directories: ryot::DirectoryConfigs {
            source_path: PathBuf::from("tests/fixtures"),
            destination_path: PathBuf::from("tests/fixtures"),
        },
        sprite_sheet: SpriteSheetConfig::cip_sheet(),
    };

    decompress_sprite_sheets(
        content_config,
        &vec!["1.bmp.lzma".to_string(), "2.bmp.lzma".to_string()],
    );

    for expected_file in ["1.png", "2.png"] {
        let expected_path = PathBuf::from(format!(
            "tests/fixtures/{}/{}",
            SPRITE_SHEET_FOLDER, expected_file
        ));
        assert!(expected_path.exists());

        assert_eq!(load_fixture_image(&expected_path).unwrap(), expected);
    }
}

#[fixture]
fn image_fixture() -> RgbaImage {
    load_fixture_image(&PathBuf::from("tests/fixtures/expected.png")).unwrap()
}

fn load_fixture_image(file_name: &PathBuf) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let mut reader = image::io::Reader::new(BufReader::new(File::open(file_name)?));

    reader.set_format(image::ImageFormat::Png);

    let img = reader.decode()?.to_rgba8();

    Ok(img)
}
