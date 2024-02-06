use serde::Serialize;
use std::fs;
use std::io::Error;

pub fn toml<T>(path: &str, data: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let string_data = toml::to_string(data).unwrap();

    fs::write(path, string_data)?;

    Ok(())
}
