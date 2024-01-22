use ryot::{
    assets_root_path, decompress_sprite_sheets, read_content_configs, ContentConfigs,
    CONTENT_CONFIG_PATH,
};
use std::fs;
use std::path::Path;

pub fn run() -> Result<(), std::io::Error> {
    let path = assets_root_path().join(CONTENT_CONFIG_PATH);
    let path = match path.to_str() {
        Some(path) => path,
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to convert path to str",
        ))?,
    };

    // Tell Cargo to rerun this build script if the config file changes
    println!("cargo:rerun-if-changed={}", path);

    let content_config = read_content_configs(path);

    let ContentConfigs { directories, .. } = content_config.clone();

    // Tell Cargo to rerun this build script if our content folder changes
    println!(
        "cargo:rerun-if-changed={}",
        directories.source_path.display()
    );

    // Tell Cargo to rerun this build script if our destination folder changes
    println!(
        "cargo:rerun-if-changed={}",
        directories.destination_path.display()
    );

    directories.source_path.try_exists().unwrap_or_else(|_| {
        panic!(
            "Source path {} does not exist",
            directories.source_path.display()
        )
    });

    if copy_catalog(&directories.source_path, &directories.destination_path).is_err() {
        println!(
            "cargo:warning=Catalog file not found in {}",
            directories.source_path.display()
        );
        return Ok(());
    }

    copy_appearances(&directories.source_path, &directories.destination_path)?;
    decompress_sprites(content_config)?;

    Ok(())
}

pub fn is_path_within_root(
    destination_path: &Path,
    root_path: &Path,
) -> Result<bool, std::io::Error> {
    Ok(fs::canonicalize(destination_path)?.starts_with(fs::canonicalize(root_path)?))
}

fn copy_catalog(source_path: &Path, destination_path: &Path) -> Result<u64, std::io::Error> {
    let file_name = "catalog-content.json";

    fs::copy(
        source_path.join(file_name),
        destination_path.join(file_name),
    )
}

fn copy_appearances(source_path: &Path, destination_path: &Path) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(source_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("appearances-")
                    && file_name.ends_with(&format!(".{}", "dat"))
                {
                    let new_path = destination_path.join("appearances.dat");

                    fs::copy(&path, new_path)?;

                    break;
                }
            }
        }
    }

    Ok(())
}

fn decompress_sprites(content_configs: ContentConfigs) -> Result<(), std::io::Error> {
    let ContentConfigs { directories, .. } = content_configs.clone();

    let files = fs::read_dir(directories.source_path)?
        .filter_map(|e| {
            if let Ok(entry) = e {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("sprites-")
                            && file_name.ends_with(&format!(".{}", "bmp.lzma"))
                        {
                            return Some(file_name.to_string());
                        }
                    }
                }
            }

            None
        })
        .collect::<Vec<String>>();

    decompress_sprite_sheets(content_configs, &files);

    Ok(())
}
