#![allow(unused_imports)]
#![allow(dead_code)]

#[path = "src/lib.rs"]
mod rusty_hooks;

use std::process::exit;
use std::{env, vec};

fn main() {
    if ci_info::is_ci() {
        exit(0);
    };

    let target_directory = env::var("OUT_DIR").unwrap();
    if let Err(err) = rusty_hooks::init_directory(
        nias::get_command_runner(),
        nias::get_file_writer(),
        nias::get_file_existence_checker(),
        Some(&target_directory),
        vec![],
    ) {
        println!(
            "Fatal error encountered during initialization. Details: {}",
            err
        );
    };
    exit(0);
}
