use std::env;

pub fn run() {
    let target = env::var("TARGET").expect("Failed to get target");
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE);
    }
}
