use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/cip_content/appearances.proto"],
        &["src/cip_content/"],
    )?;
    Ok(())
}
