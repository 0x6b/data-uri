use std::{borrow::Cow, fs::read, ops::Deref, path::Path, str::from_utf8};

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use urlencoding::encode;

use crate::state::{Initialized, State, Uninitialized};

pub struct DataUriConverter<S>
where
    S: State,
{
    state: S,
}

impl<S> Deref for DataUriConverter<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DataUriConverter<Uninitialized> {
    pub fn try_new() -> Result<DataUriConverter<Initialized>> {
        let Uninitialized { file, mime_type } = Uninitialized::parse();
        Self::new(&file, mime_type)
    }

    pub fn new<P>(file: &P, mime_type: Option<String>) -> Result<DataUriConverter<Initialized>>
    where
        P: AsRef<Path>,
    {
        let data = read(file)?;
        let mime_type = mime_type
            .clone()
            .map_or_else(|| tree_magic_mini::from_u8(&data).to_string(), |mime_type| mime_type);

        Ok(DataUriConverter { state: Initialized { data, mime_type } })
    }
}

impl DataUriConverter<Initialized> {
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
        let data = crate::DataUriConverter::new(
            &"fixtures/rust-logo-512x512.png",
            Some("image/png".to_string()),
        )?
        .convert()?;
        let fixture = std::fs::read("fixtures/rust-logo-512x512.txt")?;
        assert_eq!(data, String::from_utf8(fixture)?);
        Ok(())
    }
}
