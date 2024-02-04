use std::{borrow::Cow, error::Error, fs::read, path::PathBuf, str::from_utf8};

use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use urlencoding::encode;

#[derive(Parser)]
#[command(version, about)]
struct Opt {
    /// Path to a file to convert. If text, assume UTF-8.
    #[arg()]
    file: PathBuf,

    /// MIME type. If none specified, will determine automagically.
    #[arg(short, long)]
    mime_type: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opt { file, mime_type } = Opt::parse();
    println!("{}", convert(&read(file)?, mime_type)?);
    Ok(())
}

fn convert(content: &[u8], mime_type: Option<String>) -> Result<String, Box<dyn Error>> {
    let mime_type = mime_type.unwrap_or_else(|| tree_magic_mini::from_u8(content).to_string());

    let (encoding, data) = if mime_type.starts_with("text/") {
        ("", encode(from_utf8(content)?))
    } else {
        (";base64", Cow::from(general_purpose::STANDARD.encode(content)))
    };

    Ok(format!("data:{}{},{}", mime_type, encoding, data))
}

#[cfg(test)]
mod test {
    use std::fs::read;

    use reqwest::blocking::get;

    use crate::convert;

    #[test]
    fn test_convert() -> Result<(), Box<dyn std::error::Error>> {
        let data =
            convert(&get("https://www.rust-lang.org/logos/rust-logo-512x512.png")?.bytes()?, None)?;
        let fixture = read("fixtures/rust-logo-512x512.txt")?;
        assert_eq!(data, String::from_utf8(fixture)?);
        Ok(())
    }
}
