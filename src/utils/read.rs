use serde::de::DeserializeOwned;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn toml<T>(path: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let string_data = fs::read_to_string(path.to_string())?;

    let toml_data = toml::from_str::<T>(string_data.as_str())
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(toml_data)
}
