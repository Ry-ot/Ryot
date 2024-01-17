use std::fs::File;
use std::io;

pub fn compress<C: Compression>(source_path: &str, level: Option<i32>) -> io::Result<()> {
    C::compress(
        source_path,
        format!("{}.{}", source_path, C::get_extension()).as_str(),
        level,
    )?;

    Ok(())
}

pub fn decompress<C: Compression>(source_path: &str) -> io::Result<()> {
    C::decompress(
        source_path,
        source_path
            .to_string()
            .replace(C::get_extension(), "2")
            .as_str(),
    )?;

    Ok(())
}

pub trait Compression {
    fn compress(source: &str, destination: &str, level: Option<i32>) -> io::Result<()>;
    fn decompress(source: &str, destination: &str) -> io::Result<()>;
    fn get_extension() -> &'static str;
    fn default_level() -> i32 {
        0
    }
}

pub struct Zstd;

impl Compression for Zstd {
    fn compress(source: &str, destination: &str, level: Option<i32>) -> io::Result<()> {
        let mut encoder = zstd::Encoder::new(
            File::create(destination)?,
            level.unwrap_or(Self::default_level()),
        )?;

        io::copy(&mut File::open(source)?, &mut encoder)?;

        encoder.finish()?;

        Ok(())
    }

    fn decompress(source: &str, destination: &str) -> io::Result<()> {
        let mut decoder = {
            let file = File::open(source)?;
            zstd::Decoder::new(file)?
        };

        let mut target = File::create(destination)?;

        io::copy(&mut decoder, &mut target)?;

        Ok(())
    }

    fn get_extension() -> &'static str {
        "snp"
    }

    fn default_level() -> i32 {
        zstd::DEFAULT_COMPRESSION_LEVEL
    }
}
