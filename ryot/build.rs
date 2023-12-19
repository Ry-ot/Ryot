use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/appearances/appearances.proto"], &["src/appearances/"])?;
    Ok(())
}