use crate::levels::Level;
use crate::resources::{EditorData, LevelEdit};
use amethyst::config::ConfigError;
use amethyst::{
    prelude::{Config, World, WorldExt},
    utils::application_root_dir,
};

use std::path::PathBuf;

/// Returns a PathBuf to the file that is used to store auto saves.
pub fn auto_save_file() -> PathBuf {
    get_levels_dir().join("auto_save.ron")
}

/// Load and return the auto save level.
/// If there is no auto save file to load from, the default implementation will be used.
pub fn load_auto_save() -> Result<LevelEdit, ConfigError> {
    let level_file = auto_save_file();
    if level_file.exists() {
        read_level_file(level_file)
    } else {
        Ok(LevelEdit::default())
    }
}

/// Load and return the level with the given name.
pub fn load(name: String) -> Result<LevelEdit, ConfigError> {
    let level_file = get_levels_dir().join(name + ".ron");
    read_level_file(level_file)
}

fn read_level_file(level_file: PathBuf) -> Result<LevelEdit, ConfigError> {
    let level = Level::load(level_file)?;
    Ok(level.into())
}

/// Write the current state of the LevelEdit to the auto save file, overwriting what is already
/// there.
pub fn auto_save(world: &mut World) -> Result<(), ConfigError> {
    write_level_file(auto_save_file(), world)
}

/// Store the current state of the LevelEdit to file. The given name will be used as a filename.
pub fn save(name: String, world: &mut World) -> Result<(), ConfigError> {
    let level_file = get_levels_dir().join(name + ".ron");
    write_level_file(level_file, world)
}

fn write_level_file(level_file: PathBuf, world: &mut World) -> Result<(), ConfigError> {
    let data = world.write_resource::<EditorData>();
    let level: Level = (&*data).level.clone().into();
    level.write(level_file)
}

fn get_levels_dir() -> PathBuf {
    application_root_dir()
        .expect("Root dir not found!")
        .join("assets/")
        .join("levels/")
}
