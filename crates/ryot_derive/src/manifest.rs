extern crate proc_macro;

use proc_macro::TokenStream;
use std::{env, path::PathBuf};
use toml_edit::{DocumentMut, Item};

/// The path to the `Cargo.toml` file for your project.
pub struct Manifest {
    project: String,
    dirs: Vec<String>,
    manifest: DocumentMut,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            project: "ryot".to_string(),
            dirs: vec!["ryot_internal".to_string()],
            manifest: env::var_os("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .map(|mut path| {
                    path.push("Cargo.toml");
                    if !path.exists() {
                        panic!(
                            "No Cargo manifest found for crate. Expected: {}",
                            path.display()
                        );
                    }
                    let manifest = std::fs::read_to_string(path.clone()).unwrap_or_else(|_| {
                        panic!("Unable to read cargo manifest: {}", path.display())
                    });
                    manifest.parse::<DocumentMut>().unwrap_or_else(|_| {
                        panic!("Failed to parse cargo manifest: {}", path.display())
                    })
                })
                .expect("CARGO_MANIFEST_DIR is not defined."),
        }
    }
}

impl Manifest {
    /// Attempt to retrieve the [path](syn::Path) of a particular package in
    /// the [manifest](Manifest) by [name](str).
    pub fn maybe_get_path(&self, name: &str) -> Option<syn::Path> {
        fn dep_package(dep: &Item) -> Option<&str> {
            if dep.as_str().is_some() {
                None
            } else {
                dep.get("package").map(|name| name.as_str().unwrap())
            }
        }

        let find_in_deps = |deps: &Item| -> Option<syn::Path> {
            let package = if let Some(dep) = deps.get(name) {
                return Some(Self::parse_str(dep_package(dep).unwrap_or(name)));
            } else if let Some(dep) = deps.get(&self.project) {
                dep_package(dep).unwrap_or(&self.project)
            } else {
                for dir in &self.dirs {
                    if let Some(dep) = deps.get(dir) {
                        return Some(Self::parse_str(dep_package(dep).unwrap_or(dir)));
                    }
                }

                return None;
            };

            let mut path = Self::parse_str::<syn::Path>(package);
            if let Some(module) = name.strip_prefix(&self.get_prefix()) {
                path.segments.push(Self::parse_str(module));
            }
            Some(path)
        };

        let deps = self.manifest.get("dependencies");
        let deps_dev = self.manifest.get("dev-dependencies");

        deps.and_then(find_in_deps)
            .or_else(|| deps_dev.and_then(find_in_deps))
    }

    /// Returns the path for the crate with the given name.
    ///
    /// This is a convenience method for constructing a [manifest] and
    /// calling the [`get_path`] method.
    ///
    /// This method should only be used where you just need the path and can't
    /// cache the [manifest]. If caching is possible, it's recommended to create
    /// the [manifest] yourself and use the [`get_path`] method.
    ///
    /// [`get_path`]: Self::get_path
    /// [manifest]: Self
    #[allow(dead_code)]
    pub fn get_path_direct(name: &str) -> syn::Path {
        Self::default().get_path(name)
    }

    /// Returns the path for the crate with the given name.
    pub fn get_path(&self, name: &str) -> syn::Path {
        self.maybe_get_path(name)
            .unwrap_or_else(|| Self::parse_str(name))
    }

    /// Attempt to parse the provided [path](str) as a [syntax tree node](syn::parse::Parse)
    pub fn try_parse_str<T: syn::parse::Parse>(path: &str) -> Option<T> {
        syn::parse(path.parse::<TokenStream>().ok()?).ok()
    }

    /// Attempt to parse provided [path](str) as a [syntax tree node](syn::parse::Parse).
    ///
    /// # Panics
    ///
    /// Will panic if the path is not able to be parsed. For a non-panicing option, see [`try_parse_str`]
    ///
    /// [`try_parse_str`]: Self::try_parse_str
    pub fn parse_str<T: syn::parse::Parse>(path: &str) -> T {
        Self::try_parse_str(path).unwrap()
    }

    /// Attempt to get a subcrate [path](syn::Path) under Project by [name](str)
    #[allow(dead_code)]
    pub fn get_subcrate(&self, subcrate: &str) -> Option<syn::Path> {
        self.maybe_get_path(&self.project)
            .map(|project_path| {
                let mut segments = project_path.segments;
                segments.push(Manifest::parse_str(subcrate));
                syn::Path {
                    leading_colon: None,
                    segments,
                }
            })
            .or_else(|| self.maybe_get_path(&format!("{}{subcrate}", self.get_prefix())))
    }

    fn get_prefix(&self) -> String {
        format!("{}_", self.project)
    }
}
