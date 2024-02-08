use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::utils::read;
use crate::utils::write;

lazy_static! {
    static ref DB_PATH: String = {
        dirs::home_dir()
            .unwrap()
            .join(".local/share/fypm")
            .to_string_lossy()
            .into_owned()
    };
}

#[derive(Serialize, Deserialize)]
pub struct DataBowl {
    name: String,
    description: String,
}

#[derive(Debug)]
pub struct DataBowlHandler;

impl DataBowlHandler {
    pub fn verify_by_name(name: &String) -> Result<bool, Error> {
        let get_result = fs::read_dir(Path::new(&DB_PATH.to_string()).join(name.as_str()));

        let result = match get_result {
            Ok(_) => Ok(true),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(false),
                _ => Err(error),
            },
        };

        result
    }
    pub fn ensure_db_existence(&self) -> Result<&DataBowlHandler, Error> {
        let get_result = fs::read_dir(DB_PATH.to_string());

        let result = match get_result {
            Ok(_) => Ok(self),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    let create_dir = fs::create_dir_all(DB_PATH.to_string());

                    if create_dir.is_err() {
                        panic!(
                            "I can't create {} directory. Fix it immediately >:(.",
                            DB_PATH.to_string()
                        );
                    }

                    Ok(self)
                }
                ErrorKind::PermissionDenied => {
                    panic!(
                        "Fypm doesn't have permission to read into {} Fix it immediately >:(.",
                        DB_PATH.to_string()
                    );
                }
                _ => Err(error),
            },
        };

        result
    }

    pub fn create(name: &String, description: &String) -> Result<DataBowl, Error> {
        let data_bowl = DataBowl {
            name: name.clone(),
            description: description.clone(),
        };

        let verify_existance = DataBowlHandler::verify_by_name(&data_bowl.name);

        if verify_existance? == false {
            fs::create_dir(DB_PATH.to_string() + "/" + data_bowl.name.as_str())
                .expect("Couldn't create directory!");

            write::toml(
                Path::new(&DB_PATH.to_string())
                    .join(&data_bowl.name)
                    .join("info.toml")
                    .to_str()
                    .unwrap(),
                &data_bowl,
            )?;

            Ok(data_bowl)
        } else {
            Err(Error::new(
                ErrorKind::AlreadyExists,
                "This DataBowl already exists",
            ))
        }
    }
    pub fn remove(name: &String) -> Result<(), Error> {
        let verify_existance = DataBowlHandler::verify_by_name(name);

        if verify_existance? == true {
            fs::remove_dir_all(Path::new(&DB_PATH.to_string()).join(name.as_str()))?;

            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "This DataBowl was not found!",
            ))
        }
    }
    pub fn rename(old_name: &String, new_name: &String) -> Result<(), Error> {
        let verify_existance = DataBowlHandler::verify_by_name(old_name);

        if verify_existance? == true {
            let old_path = Path::new(&DB_PATH.to_string()).join(old_name.as_str());
            let new_path = Path::new(&DB_PATH.to_string()).join(new_name.as_str());

            let old_info_string = fs::read_to_string(&old_path.join("info.toml"))?;

            let mut new_info = toml::from_str::<DataBowl>(old_info_string.as_str())
                .map_err(|err| Error::new(ErrorKind::Other, err))?;

            new_info.name = new_name.clone();

            write::toml(&old_path.join("info.toml").to_str().unwrap(), &new_info)?;

            fs::rename(&old_path, &new_path)?;

            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "This DataBowl was not found!",
            ))
        }
    }
    pub fn list() -> Result<Vec<(String, String)>, Error> {
        let mut data_bowls = Vec::new();

        for folder in fs::read_dir(DB_PATH.to_string()).unwrap() {
            let folder_info = read::toml::<DataBowl>(
                Path::new(DB_PATH.as_str())
                    .join(&folder.unwrap().file_name())
                    .join("info.toml")
                    .to_str()
                    .unwrap(),
            )?;

            let name = folder_info.name;
            let description = folder_info.description;

            data_bowls.push((name, description));
        }

        Ok(data_bowls)
    }
}
