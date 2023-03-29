use std::error::Error;
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;

use crate::shortcuts::Shortcuts;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub playlists_dir: String,
    pub normal: Shortcuts,
}

static INSTANCE: OnceCell<Config> = OnceCell::new(); 

impl Config {
    pub fn global() -> &'static Self {
        INSTANCE.get().expect("Config instance not loaded!")
    }

    pub fn set_global(instance: Self) {
        INSTANCE.set(instance).unwrap();
    }

    /// Loads the shortcuts from the default path, which is
    /// [dirs::config_dir](https://docs.rs/dirs/latest/dirs/fn.config_dir.html)/tori.yaml
    pub fn from_default_location() -> Result<Self, Box<dyn Error>> {
        let path = dirs::config_dir().unwrap_or_default().join("tori.yaml");
        Self::from_path(path)
    }

    /// Loads the shortcuts from some path
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(file)?)
    }
}