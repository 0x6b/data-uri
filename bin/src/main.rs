use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use data_uri_converter::DataUriConverter;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Path to a file to convert. If text, assume UTF-8.
    #[arg()]
    pub file: PathBuf,

    /// Internet media type specification (with optional parameters.) If none specified, will
    /// determine automagically.
    #[arg(short, long)]
    pub media_type: Option<String>,
}

fn main() -> Result<()> {
    let Args { ref file, media_type } = Args::parse();
    println!("{}", &DataUriConverter::from_file(file, media_type)?.convert()?);

    Ok(())
}
