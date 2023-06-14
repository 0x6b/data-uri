use std::{error::Error, fs, path::PathBuf};

use base64::{engine::general_purpose, Engine as _};
use clap::Parser;

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
    let content = fs::read(file)?;
    println!("{}", convert(&content, mime_type)?);
    Ok(())
}

fn convert(content: &[u8], mime_type: Option<String>) -> Result<String, Box<dyn Error>> {
    let mime_type = match mime_type {
        None => tree_magic_mini::from_u8(content).to_string(),
        Some(m) => m,
    };

    let (encoding, data) = if mime_type.starts_with("text/") {
        (
            "",
            urlencoding::encode(std::str::from_utf8(content)?).into_owned(),
        )
    } else {
        (";base64", general_purpose::STANDARD.encode(content))
    };

    Ok(format!("data:{}{},{}", mime_type, encoding, data))
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::convert;

    #[test]
    fn test_convert() {
        let content =
            reqwest::blocking::get("https://www.rust-lang.org/logos/rust-logo-512x512.png")
                .unwrap()
                .bytes()
                .unwrap();
        let data = convert(content.as_ref(), None).unwrap();
        let test = fs::read("fixtures/rust-logo-512x512.txt").unwrap();

        assert_eq!(data, String::from_utf8(test).unwrap());
    }
}
