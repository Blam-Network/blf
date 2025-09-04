use std::error::Error;
use std::ffi::OsStr;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use serde::de::DeserializeOwned;
use serde::Serialize;
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};

pub mod bitstream;

pub fn read_file_to_string(path: impl Into<String>) -> BLFLibResult<String> {
    let path = path.into();
    let mut file = File::open(&path).map_err(|err|{
        Box::<dyn Error>::from(format!("read_file_to_string({path}) {}", err))
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_json_file<T: DeserializeOwned>(path: impl Into<String>) -> BLFLibResult<T> {
    let path = path.into();
    let json = read_file_to_string(&path)?;
    serde_json::from_str(&json).map_err(|e| BLFLibError::from(format!("Failed to read JSON file {}\r\n{}", path, e.to_string())))
}

pub fn write_json_file<T: Serialize>(value: &T, path: impl Into<String> + AsRef<OsStr>) -> BLFLibResult {
    let directory = OPTION_TO_RESULT!(Path::new(&path).parent(), "Invalid path: No directory")?;
    create_dir_all(directory)?;
    let json = serde_json::to_string_pretty(value)?;
    let mut text_file = File::create(path.into())?;
    text_file.write_all(json.as_bytes())?;
    Ok(())
}