use std::path::PathBuf;

use clap::Parser;

pub trait State {}

#[derive(Parser)]
#[command(version, about)]
pub struct Uninitialized {
    /// Path to a file to convert. If text, assume UTF-8.
    #[arg()]
    pub file: PathBuf,

    /// MIME type. If none specified, will determine automagically.
    #[arg(short, long)]
    pub mime_type: Option<String>,
}

impl State for Uninitialized {}

pub struct Initialized {
    pub data: Vec<u8>,
    pub mime_type: String,
}

impl State for Initialized {}
