use std::{borrow::Cow, fs::read, path::Path, str::from_utf8};

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use urlencoding::encode;

pub struct DataUriConverter {
    data: Vec<u8>,
    mime_type: String,
}

impl DataUriConverter {
    pub fn from_data(data: &[u8], mime_type: Option<String>) -> Result<DataUriConverter> {
        Ok(DataUriConverter {
            data: data.to_vec(),
            mime_type: mime_type.unwrap_or_else(|| tree_magic_mini::from_u8(data).to_string()),
        })
    }

    pub fn from_file<P>(file: P, mime_type: Option<String>) -> Result<DataUriConverter>
    where
        P: AsRef<Path>,
    {
        DataUriConverter::from_data(&read(file)?, mime_type)
    }

    pub fn convert(&self) -> Result<String> {
        let (encoding, data) = if self.mime_type.starts_with("text/") {
            ("", encode(from_utf8(&self.data)?))
        } else {
            (";base64", Cow::from(general_purpose::STANDARD.encode(&self.data)))
        };

        Ok(format!("data:{}{encoding},{data}", self.mime_type))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_convert() -> anyhow::Result<()> {
        let data = crate::DataUriConverter::from_file(
            "fixtures/rust-logo-512x512.png",
            Some("image/png".to_string()),
        )?
        .convert()?;
        let fixture = std::fs::read("fixtures/rust-logo-512x512.txt")?;
        assert_eq!(data, String::from_utf8(fixture)?);
        Ok(())
    }
}
