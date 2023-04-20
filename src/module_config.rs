use serde::Deserialize;
use std::error::Error;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

pub const MODULE_DIRECTORY: &str = "modules";
pub const MODULE_CONFIG: &str = "ini.toml";

#[derive(Deserialize, Debug)]
enum Core {
    Native,
    Python,
}

#[derive(Deserialize, Debug)]
pub struct ModuleConfig {
    core: Core,
}

pub fn load_module_config(module_name: &String) -> Option<ModuleConfig> {
    fn load_config_file(file: PathBuf) -> Result<ModuleConfig, Box<dyn Error>> {
        Ok(toml::from_str(read_to_string(&file)?.as_str())?)
    }

    read_dir(MODULE_DIRECTORY)
        .expect("Could not access modules directory. Terminating")
        .find_map(|dir_entry| {
            let file = dir_entry.unwrap().path().join(MODULE_CONFIG);
            let name = file
                .parent()
                .unwrap()
                .strip_prefix(MODULE_DIRECTORY)
                .unwrap()
                .to_string_lossy()
                .to_string();
            if &name != module_name {
                return None;
            }
            match load_config_file(file) {
                Ok(config) => Some(config),
                Err(e) => {
                    println!("{:?}", e);
                    None
                }
            }
        })
}
