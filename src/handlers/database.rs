//#region           Crates
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
//#endregion
//#region           Modules
use crate::DB_PATH;
use crate::utils::write;
use crate::utils::read;
//#endregion
//#region           Structs
#[derive(Serialize, Deserialize)]
pub struct DataBowl {
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub description: String,
    pub params: String,
}
#[derive(Serialize, Deserialize)]
pub struct PresetFile {
    array: Vec<Preset>,
}

#[derive(Debug)]
pub struct DBHandler;
pub struct PresetHandler {
    pub database_name: String,
}
//#endregion
//#region           Implementation
impl DBHandler {
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
    pub fn ensure_db_existence(&self) -> Result<&DBHandler, Error> {
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
        let database = DataBowl {
            name: name.clone(),
            description: description.clone(),
        };

        let verify_existance = DBHandler::verify_by_name(&database.name);

        if verify_existance? == false {
            fs::create_dir(DB_PATH.to_string() + "/" + database.name.as_str())
                .expect("Couldn't create directory!");

            write::toml(
                Path::new(&DB_PATH.to_string())
                    .join(&database.name)
                    .join("info.toml")
                    .to_str()
                    .unwrap(),
                &database,
            )?;

            Ok(database)
        } else {
            Err(Error::new(
                ErrorKind::AlreadyExists,
                "This DataBowl already exists",
            ))
        }
    }
    pub fn remove(name: &String) -> Result<(), Error> {
        let verify_existance = DBHandler::verify_by_name(name);

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
        let verify_existance = DBHandler::verify_by_name(old_name);

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
        let mut databases = Vec::new();

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

            databases.push((name, description));
        }

        Ok(databases)
    }
}
impl PresetHandler {
    fn get_file_path(&self) -> String {
        Path::new(&DB_PATH.to_string())
            .join(self.database_name.as_str())
            .join("presets.toml")
            .to_string_lossy()
            .to_string()
    }
    pub fn verify_preset_file_existence(&self) -> Result<bool, Error> {
        let path = self.get_file_path();

        let get_result = read::toml::<PresetFile>(path.as_str());

        let result = match get_result {
            Ok(_) => Ok(true),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(false),
                _ => Err(error),
            },
        };

        result
    }
    pub fn verify_preset_existence(&self, name: &String) -> Result<bool, Error> {
        let path = self.get_file_path();

        let get_result = read::toml::<PresetFile>(path.as_str());

        let result = match get_result {
            Ok(presets) => Ok(presets
                .array
                .iter()
                .any(|preset: &Preset| preset.name == *name)),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    let not_found_msg = "You haven't checked to see if a PresetFile exists! Look where we've come... :(";

                    Err(Error::new(ErrorKind::NotFound, not_found_msg))
                }
                _ => Err(error),
            },
        };

        result
    }
    pub fn add<T: Serialize>(
        &self,
        name: &String,
        description: &String,
        params: &T,
    ) -> Result<(), Error> {
        let str_data =
            toml::to_string(params).expect("Unable to transform parameters into a string!");

        let preset = Preset {
            name: name.clone(),
            description: description.clone(),
            params: str_data,
        };

        let path = self.get_file_path();

        if self.verify_preset_file_existence()? == true {
            let mut preset_file = read::toml::<PresetFile>(path.as_str()).unwrap();

            if preset_file.array.iter().any(|preset| preset.name == *name) {
                panic!("Preset {} already exists!", name);
            }

            preset_file.array.push(preset);

            write::toml(path.as_str(), &preset_file)?;
        } else {
            let preset_file = PresetFile {
                array: vec![preset],
            };

            write::toml(path.as_str(), &preset_file)?;
        }

        Ok(())
    }
    pub fn remove(&self, name: &String) -> Result<(), Error> {
        let path = self.get_file_path();

        if self.verify_preset_file_existence()? == true {
            let mut preset_file = read::toml::<PresetFile>(path.as_str()).unwrap();

            preset_file.array.retain(|preset| preset.name != *name);

            write::toml(path.as_str(), &preset_file)?;

            Ok(())
        } else {
            let not_found_msg = "Preset not found!";

            Err(Error::new(ErrorKind::NotFound, not_found_msg))
        }
    }
    pub fn get(&self, name: &String) -> Result<Preset, Error> {
        let path = self.get_file_path();

        if self.verify_preset_file_existence()? == true {
            let preset_file = read::toml::<PresetFile>(path.as_str()).unwrap();

            for preset in preset_file.array {
                if preset.name == *name {
                    return Ok(preset);
                }
            }
        }

        Err(Error::new(ErrorKind::NotFound, "Preset not found!"))
    }
    pub fn list(&self) -> Result<Vec<(String, String)>, Error> {
        let path = self.get_file_path();

        let mut presets = Vec::new();

        if self.verify_preset_file_existence()? == true {
            let preset_file = read::toml::<PresetFile>(path.as_str()).unwrap();

            for preset in preset_file.array {
                presets.push((preset.name, preset.description));
            }
        }

        Ok(presets)
    }
}
//#endregion
