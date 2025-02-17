use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Path to a file to convert. If text, assume UTF-8.
    #[arg()]
    pub file: std::path::PathBuf,

    /// MIME type. If none specified, will determine automagically.
    #[arg(short, long)]
    pub mime_type: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Args { ref file, mime_type } = Args::parse();
    println!("{}", data_uri_converter::DataUriConverter::from_file(file, mime_type)?.convert()?);

    Ok(())
}
