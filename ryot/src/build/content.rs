use crate::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, result};

/// Builder for content.
/// Builds the necessary content for the game to run from the original content folder.
/// It does the necessary transformations (like decompressing sprite sheets) and copies the
/// necessary files to the correct asset folder.
///
/// It also tells Cargo to rerun (or not) the build script if the content folder changes
/// based on the `rebuild_on_change` flag.
#[derive(Debug, Default)]
pub struct ContentBuild {
    pub rebuild_on_change: bool,
    pub path: Option<PathBuf>,
}

impl ContentBuild {
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            rebuild_on_change: false,
            path: Some(path),
        }
    }

    pub fn rebuilding_on_change() -> Self {
        Self {
            rebuild_on_change: true,
            path: None,
        }
    }

    pub fn rebuilding_on_change_from_path(path: PathBuf) -> Self {
        Self {
            rebuild_on_change: true,
            path: Some(path),
        }
    }

    pub fn run(self) -> result::Result<(), std::io::Error> {
        println!("cargo:warning=Running content build {:?}", self);

        let path = self
            .path
            .clone()
            .unwrap_or_else(|| assets_root_path().join(CONTENT_CONFIG_PATH));

        let content_config = read_content_configs(path.clone());

        let ContentConfigs { directories, .. } = content_config.clone();

        if self.rebuild_on_change {
            // Tell Cargo to rerun this build script if the config file changes
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

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
        }

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
}

fn copy_catalog(
    source_path: &Path,
    destination_path: &Path,
) -> result::Result<u64, std::io::Error> {
    let file_name = "catalog-content.json";

    fs::copy(
        source_path.join(file_name),
        destination_path.join(file_name),
    )
}

fn copy_appearances(
    source_path: &Path,
    destination_path: &Path,
) -> result::Result<(), std::io::Error> {
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

fn decompress_sprites(content_configs: ContentConfigs) -> result::Result<(), std::io::Error> {
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
