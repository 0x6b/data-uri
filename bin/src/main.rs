use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Path to a file to convert. If text, assume UTF-8.
    #[arg()]
    pub file: std::path::PathBuf,

    /// Internet media type specification (with optional parameters.) If none specified, will
    /// determine automagically.
    #[arg(short, long)]
    pub media_type: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Args { ref file, media_type } = Args::parse();
    println!("{}", data_uri_converter::DataUriConverter::from_file(file, media_type)?.convert()?);

    Ok(())
}
