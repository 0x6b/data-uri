use anyhow::Result;
use data_uri_converter::DataUriConverter;

fn main() -> Result<()> {
    println!("{}", DataUriConverter::try_new()?.convert()?);

    Ok(())
}
