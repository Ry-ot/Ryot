extern crate embed_resource;

use ryot::prelude::ContentBuild;
use std::env;

fn main() {
    println!("cargo:warning=Build script of ryot_compass is running...");

    if env::var("SKIP_BUILD_SCRIPT").is_ok() {
        println!("cargo:warning=Skipping ryot_compass build script for CI build");
        return;
    }

    build_target();

    #[cfg(feature = "content_rebuild_on_change")]
    ContentBuild::rebuilding_on_change()
        .run()
        .expect("Failed to build assets");

    #[cfg(not(feature = "content_rebuild_on_change"))]
    ContentBuild::default()
        .run()
        .expect("Failed to build assets");

    println!("cargo:warning=Build script of ryot_compass completed.");
}

fn build_target() {
    let target = env::var("TARGET").expect("Failed to get target");
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE);
    }
}
