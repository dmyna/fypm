//#region           Crates
use crate::utils::err::{FypmError, FypmErrorKind};
use rusqlite::{Connection, Error as RusqliteError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;
//#endregion
//#region           Modules
use crate::utils::read;
use crate::utils::write;
use crate::DB_PATH;
//#endregion
//#region           Structs
#[derive(Serialize, Deserialize)]
pub struct DataBowl {
    pub name: String,
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
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
    pub table_name: String,
    pub conn: Arc<Connection>,
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
    pub fn ensure_db_path(&self) -> Result<&DBHandler, Error> {
        fs::create_dir_all(DB_PATH.to_string())?;

        Ok(self)
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
    pub fn ensure_table_existence(&self) -> Result<usize, RusqliteError> {
        self.conn.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT,
                description TEXT,
                params BLOB
            )",
                self.table_name
            )
            .as_str(),
            [],
        )
    }

    pub fn add<T: Serialize>(
        &self,
        name: &String,
        description: &String,
        params: &T,
    ) -> Result<(), FypmError> {
        let received_rows = self
            .conn
            .execute(
                format!("SELECT * FROM {} WHERE name = ?1", self.table_name).as_str(),
                [&name],
            )
            .unwrap();

        if received_rows > 0 {
            return Err(FypmError {
                kind: FypmErrorKind::AlreadyExists,
                message: "This preset already exists!".to_string(),
            });
        }

        let uuid = Uuid::now_v7().to_string();

        let str_data =
            toml::to_string(params).expect("Unable to transform parameters into a string!");

        let preset = Preset {
            name: name.clone(),
            description: description.clone(),
            params: str_data,
        };

        {
            self.ensure_table_existence().unwrap();

            self.conn
                .execute(
                    format!(
                        "INSERT INTO {} (id, name, description, params) VALUES (?1, ?2, ?3, ?4)",
                        self.table_name
                    )
                    .as_str(),
                    [&uuid, &preset.name, &preset.description, &preset.params],
                )
                .unwrap();
        }

        Ok(())
    }
    pub fn remove(&self, name: &String) -> Result<(), FypmError> {
        let removed_rows = self
            .conn
            .execute(
                format!("DELETE FROM {} WHERE name = ?1", self.table_name).as_str(),
                [&name],
            )
            .unwrap();

        if removed_rows == 0 {
            return Err(FypmError {
                kind: FypmErrorKind::NotFound,
                message: "Preset not found!".to_string(),
            });
        }

        Ok(())
    }
    pub fn get(&self, name: &String) -> Result<Preset, FypmError> {
        let mut query = self
            .conn
            .prepare(format!("SELECT * FROM {} WHERE name = ?1", self.table_name).as_str())
            .unwrap();

        let get_preset = query.query_row([&name], |row| {
            Ok(Preset {
                name: row.get(1)?,
                description: row.get(2)?,
                params: row.get(3)?,
            })
        });

        if let Ok(preset) = get_preset {
            Ok(preset)
        } else {
            Err(FypmError {
                kind: FypmErrorKind::NotFound,
                message: "Preset not found!".to_string(),
            })
        }
    }
    pub fn list(&self) -> Result<Vec<(String, String)>, Error> {
        let mut presets = Vec::new();

        {
            let mut query = self
                .conn
                .prepare(format!("SELECT name, description FROM {}", self.table_name).as_str())
                .unwrap();

            let row_iter = query
                .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
                .unwrap();

            for row in row_iter {
                let preset = row.unwrap();

                presets.push((preset.0, preset.1));
            }
        }

        Ok(presets)
    }
}
//#endregion
