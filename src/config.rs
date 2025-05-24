use std::path::PathBuf;

use simd_json::derived::ValueObjectTryAccess;
use simd_json::{self};

use crate::{Error, Result};

#[derive(Debug)]
pub struct Config {
    lockpick_dir: PathBuf,
}

impl Config {
    fn is_valid_path(path_buf: &PathBuf) -> Result<bool> {
        if !path_buf.exists() {
            return Err(Error::from("File not found"));
        }

        let extension = path_buf.extension();
        match extension {
            Some(ext) => {
                if ext != "json" {
                    let output = format!(
                        "Config file extension should be {:?}, not {:?}",
                        "json", ext
                    );
                    return Err(Error::from(output));
                }
            },
            None => {
                let output = format!("Path to file expected, got {:?}", path_buf);
                return Err(Error::from(output));
            },
        };

        Ok(true)
    }

    pub fn from_json(path_buf: PathBuf) -> Result<Self> {
        assert!(Self::is_valid_path(&path_buf)?);

        let mut data = std::fs::read_to_string(path_buf.as_path())?;

        let content = unsafe { simd_json::to_owned_value(data.as_bytes_mut())? };

        let lockpick_dir = match content.try_get("lockpick_dir")? {
            Some(value) => value.to_string(),
            None => {
                return Err(Error::from("lockpick_dir is not Object type"));
            },
        };

        let config = Self {
            lockpick_dir: PathBuf::from(lockpick_dir),
        };

        return Ok(config);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lockpick_dir: PathBuf::from("../lockpick"),
        }
    }
}
