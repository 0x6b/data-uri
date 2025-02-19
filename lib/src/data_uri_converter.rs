use std::{borrow::Cow, fs::read, ops::Deref, path::Path, str::from_utf8};

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use urlencoding::encode;

/// A marker trait for state, to prevent impossible thing (e.g. converting data to data URI without
/// data) from being called.
pub trait State {}

/// Uninitialized state, to denote that the converter is not ready to convert anything.
pub struct Uninitialized;

/// Initialized state, to represent that the converter has data and media type prepared and ready to
/// convert to data URI.
pub struct ReadyToConvert {
    data: Vec<u8>,
    media_type: String,
}

impl State for Uninitialized {}
impl State for ReadyToConvert {}

/// Convert data to a data URI
///
/// # Reference
///
/// - [RFC 2397: The "data" URL scheme](https://www.rfc-editor.org/rfc/rfc2397)
pub struct DataUriConverter<S>
where
    S: State,
{
    inner: S,
}

/// Convenience method to access the inner state
impl<S> Deref for DataUriConverter<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DataUriConverter<Uninitialized> {
    /// Create a new instance from data
    ///
    /// # Arguments
    ///
    /// - `data` - Data to convert
    /// - `media_type` - MIME type of the data. If [`None`], will determine automagically.
    ///
    /// # Returns
    ///
    /// A new instance of [`DataUriConverter`] that is ready to convert the data to a data URI.
    pub fn from_data(
        data: &[u8],
        media_type: Option<String>,
    ) -> Result<DataUriConverter<ReadyToConvert>> {
        Ok(DataUriConverter {
            inner: ReadyToConvert {
                data: data.to_vec(),
                media_type: media_type
                    .unwrap_or_else(|| tree_magic_mini::from_u8(data).to_string()),
            },
        })
    }

    /// Create a new instance from a file
    ///
    /// # Arguments
    ///
    /// - `file` - Path to a file to convert
    /// - `media_type` - MIME type of the data. If [`None`], will determine automagically.
    ///
    /// # Returns
    ///
    /// A new instance of [`DataUriConverter`] that is ready to convert the data to a data URI.
    pub fn from_file<P>(
        file: P,
        media_type: Option<String>,
    ) -> Result<DataUriConverter<ReadyToConvert>>
    where
        P: AsRef<Path>,
    {
        DataUriConverter::from_data(&read(file)?, media_type)
    }
}

impl DataUriConverter<ReadyToConvert> {
    /// Convert the data to a data URI
    pub fn convert(&self) -> Result<String> {
        let (encoding, data) = if self.media_type.starts_with("text/") {
            // If the MIME type starts with `text/`, the data should be represented using ASCII
            // encoding for octets inside the range of safe URL characters and using the standard
            // `%xx` hex encoding of URLs for octets outside that range.
            ("", encode(from_utf8(&self.data)?))
        } else {
            // Otherwise, the data should be represented using base64 encoding.
            (";base64", Cow::from(general_purpose::STANDARD.encode(&self.data)))
        };

        // The URLs are of the form:
        //
        // ```
        // data:[<mediatype>][;base64],<data>
        // ```
        Ok(format!("data:{}{encoding},{data}", self.media_type))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_convert() -> anyhow::Result<()> {
        let data = crate::DataUriConverter::from_file("fixtures/rust-logo-512x512.png", None)?
            .convert()?;
        let fixture = std::fs::read("fixtures/rust-logo-512x512.txt")?;
        assert_eq!(data, String::from_utf8(fixture)?);
        Ok(())
    }
}
