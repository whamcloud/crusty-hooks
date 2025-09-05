use crate::{config, git};
use std::collections::HashMap;

pub(crate) fn init_directory<F, G, H>(
    run_command: F,
    write_file: G,
    file_exists: H,
    target_directory: Option<&str>,
    hook_file_skip_list: Vec<&str>,
) -> Result<(), String>
where
    F: Fn(
        &str,
        Option<&str>,
        bool,
        Option<&HashMap<String, String>>,
    ) -> Result<Option<String>, Option<String>>,
    G: Fn(&str, &str, bool) -> Result<(), String>,
    H: Fn(&str) -> Result<bool, ()>,
{
    let root_directory_path = match git::get_root_directory_path(&run_command, target_directory) {
        Ok(Some(path)) => path,
        _ => return Err(String::from("Failure determining git repo root directory")),
    };
    if git::setup_hooks(
        &run_command,
        &write_file,
        &root_directory_path,
        &hook_file_skip_list,
    )
    .is_err()
    {
        return Err(String::from("Unable to create git hooks"));
    };

    if config::create_default_config_file(&write_file, &file_exists, &root_directory_path).is_err()
    {
        return Err(String::from("Unable to create config file"));
    }

    Ok(())
}
