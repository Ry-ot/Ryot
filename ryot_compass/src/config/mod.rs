use bevy::prelude::Resource;
use config::Config;
use itertools::Itertools;
use log::error;
use ryot::get_decompressed_file_name;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[allow(unused)]
#[derive(Default)]
pub struct Settings {
    pub debug: bool,
    pub content: Content,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct Content {
    pub path: PathBuf,
    pub catalog_name: String,
    pub decompressed_cache: DecompressedCache,
}

impl Content {
    pub fn build_content_file_path(&self, file: &str) -> PathBuf {
        self.path.as_path().join(file)
    }

    pub fn build_asset_path(&self, file: &str) -> PathBuf {
        let DecompressedCache::Path(decompressed_path) = &self.decompressed_cache else {
            panic!("invalid path");
        };
        let path = decompressed_path
            .as_path()
            .join(get_decompressed_file_name(file));
        path.components()
            .find_position(|c| c.as_os_str() == "assets")
            .map_or_else(
                || panic!("decompressed path must be within assets/ folder"),
                |(i, _)| path.components().skip(i + 1).collect(),
            )
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            path: "assets/cip-catalog".into(),
            catalog_name: "catalog-content.json".to_owned(),
            decompressed_cache: DecompressedCache::default(),
        }
    }
}

#[derive(Debug)]
pub enum DecompressedCache {
    Disabled,
    Path(PathBuf),
}

impl Default for DecompressedCache {
    fn default() -> Self {
        DecompressedCache::Path("assets/sprite-sheets".into())
    }
}

impl serde::Serialize for DecompressedCache {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DecompressedCache::Disabled => serializer.serialize_bool(false),
            DecompressedCache::Path(ref path) => {
                serializer.serialize_str(path.to_str().expect("invalid path"))
            }
        }
    }
}

impl<'de> Deserialize<'de> for DecompressedCache {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DecompressedCacheVisitor;

        impl<'de> Visitor<'de> for DecompressedCacheVisitor {
            type Value = DecompressedCache;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a boolean or a string")
            }

            fn visit_bool<E>(self, value: bool) -> Result<DecompressedCache, E>
            where
                E: de::Error,
            {
                Ok(if value {
                    DecompressedCache::default()
                } else {
                    DecompressedCache::Disabled
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<DecompressedCache, E>
            where
                E: de::Error,
            {
                Ok(DecompressedCache::Path(value.into()))
            }
        }

        deserializer.deserialize_any(DecompressedCacheVisitor)
    }
}

pub fn build() -> Settings {
    Config::builder()
        .add_source(config::File::from_str(
            &serde_json::to_string(&Settings::default())
                .expect("Failed to serialize default config"),
            config::FileFormat::Json,
        ))
        .add_source(config::File::with_name("target/config/custom").required(false))
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .unwrap_or_else(|err| {
            error!("Couldn't load custom configs: {}", err.to_string());

            Settings::default()
        })
}
