extern crate embed_resource;

use std::env;

fn main() {
    println!("cargo:warning=Build script of ryot_compass is running...");

    if env::var("SKIP_BUILD_SCRIPT").is_ok() {
        println!("cargo:warning=Skipping ryot_compass build script for CI build");
        return;
    }

    build_target();
}

fn build_target() {
    let target = env::var("TARGET").expect("Failed to get target");
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE);
    }
}
