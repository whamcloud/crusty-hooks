use std::collections::HashMap;

const CONFIG_FILE_TEMPLATE: &str = "[hooks]
pre-commit = [
  [\"cargo test\"]
]

[logging]
verbose = true
";

const DEFAULT_CONFIG_FILE_NAME: &str = ".crusty-hooks.toml";
const CONFIG_FILE_NAMES: [&str; 2] = [DEFAULT_CONFIG_FILE_NAME, "crusty-hooks.toml"];
pub const NO_CONFIG_FILE_FOUND: &str = "No config file found";

pub(crate) const FATAL_ERROR_DURING_CONFIG_LOOKUP: &str =
    "Fatal error encountered while looking for existing config";

fn find_config_file<F>(root_directory_path: &str, file_exists: F) -> Result<String, String>
where
    F: Fn(&str) -> Result<bool, ()>,
{
    for &config_file_name in CONFIG_FILE_NAMES.iter() {
        let path = format!("{root_directory_path}/{config_file_name}");
        match file_exists(&path) {
            Err(_) => {
                return Err(String::from(FATAL_ERROR_DURING_CONFIG_LOOKUP));
            }
            Ok(found) => {
                if found {
                    return Ok(path);
                }
            }
        };
    }

    Ok(String::from(NO_CONFIG_FILE_FOUND))
}

pub(super) fn create_default_config_file<F, G>(
    write_file: F,
    file_exists: G,
    root_directory_path: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str, bool) -> Result<(), String>,
    G: Fn(&str) -> Result<bool, ()>,
{
    create_config_file(
        &write_file,
        &file_exists,
        root_directory_path,
        DEFAULT_CONFIG_FILE_NAME,
    )
}

pub(super) fn create_config_file<F, G>(
    write_file: F,
    file_exists: G,
    root_directory_path: &str,
    desired_config_file_name: &str,
) -> Result<(), String>
where
    F: Fn(&str, &str, bool) -> Result<(), String>,
    G: Fn(&str) -> Result<bool, ()>,
{
    match find_config_file(root_directory_path, &file_exists) {
        Err(_) => {
            return Err(String::from(FATAL_ERROR_DURING_CONFIG_LOOKUP));
        }
        Ok(path) => {
            if path != NO_CONFIG_FILE_FOUND {
                return Ok(());
            }
        }
    };

    let config_file = if CONFIG_FILE_NAMES
        .iter()
        .any(|n| n == &desired_config_file_name)
    {
        desired_config_file_name
    } else {
        DEFAULT_CONFIG_FILE_NAME
    };

    if write_file(
        &format!("{}/{}", root_directory_path, config_file),
        CONFIG_FILE_TEMPLATE,
        false,
    )
    .is_err()
    {
        return Err(String::from("Failed to create config file"));
    };
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConfigFile {
    pub hooks: HashMap<String, Vec<Vec<String>>>,
}

impl ConfigFile {
    pub fn try_from_str(x: &str) -> Result<Self, toml::de::Error> {
        toml::from_str::<Self>(x)
    }
}

pub(super) fn get_config_file_contents<F, G>(
    read_file: F,
    file_exists: G,
    root_directory_path: &str,
) -> Result<String, String>
where
    F: Fn(&str) -> Result<String, ()>,
    G: Fn(&str) -> Result<bool, ()>,
{
    let path = match find_config_file(root_directory_path, &file_exists) {
        Ok(path) => {
            if path == NO_CONFIG_FILE_FOUND {
                return Err(String::from(NO_CONFIG_FILE_FOUND));
            } else {
                path
            }
        }
        Err(_) => return Err(String::from(NO_CONFIG_FILE_FOUND)),
    };

    match read_file(&path) {
        Ok(contents) => Ok(contents),
        Err(_) => Err(String::from("Failure reading file")),
    }
}
