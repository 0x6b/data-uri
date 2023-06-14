use std::{
    error::Error,
    fs,
    path::PathBuf,
};

use structopt::StructOpt;

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
    let opt = Opt::parse();
    let content = fs::read(opt.file)?;

    let mime = match opt.mime_type {
        None => tree_magic_mini::from_u8(&content).to_string(),
        Some(m) => m
    };

    let (encoding, data) = if mime.starts_with("text/") {
        ("", urlencoding::encode(std::str::from_utf8(&content)?).into_owned())
    } else {
        (";base64", base64::encode(&content))
    };

    print!("data:{}{},{}", mime, encoding, data);
    Ok(())
}
