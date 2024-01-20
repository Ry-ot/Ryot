extern crate embed_resource;

mod build_scripts;

pub use build_scripts::*;
use std::env;

fn main() {
    // Check if the SKIP_BUILD_SCRIPT environment variable is set
    if env::var("SKIP_BUILD_SCRIPT").is_ok() {
        println!("Skipping build script for CI build");
        return;
    }

    build_target::run();
    content_builder::run().expect("Failed to build assets");
}
