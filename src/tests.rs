use super::*;
use std::collections::HashMap;

#[cfg(test)]
pub(crate) mod utils {
    use std::collections::HashMap;

    pub(crate) const GIT_REV_PARSE_CMD: &str = "git rev-parse --show-toplevel";

    #[allow(clippy::type_complexity)]
    pub(crate) fn build_simple_command_runner(
        outcome: Result<Option<String>, Option<String>>,
    ) -> Box<
        dyn Fn(
            &str,
            Option<&str>,
            bool,
            Option<&HashMap<String, String>>,
        ) -> Result<Option<String>, Option<String>>,
    > {
        Box::new(
            move |_: &str, _: Option<&str>, _: bool, _: Option<&HashMap<String, String>>| {
                outcome.to_owned()
            },
        )
    }
}

#[cfg(test)]
mod init_directory_tests {
    use super::utils::build_simple_command_runner;
    use super::*;

    #[test]
    fn returns_error_when_root_directory_detect_fails() {
        let exp_err = "Failure determining git repo root directory";
        let run_command = build_simple_command_runner(Err(Some(String::from(exp_err))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| {
            panic!("Should not get here");
        };
        let file_exists = |_path: &str| panic!("Should not get here");
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Err(String::from(exp_err)));
    }

    #[test]
    fn should_return_error_when_hook_creation_fails() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Err(String::from(""));
        let file_exists = |_path: &str| panic!("Should not get here");
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Err(String::from("Unable to create git hooks")));
    }

    #[test]
    fn should_return_error_when_config_creation_fails() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Err(());
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Err(String::from("Unable to create config file")));
    }

    #[test]
    fn should_return_ok_on_success() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Ok(false);
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Ok(()));
    }
}

mod init_tests {
    use super::*;

    #[test]
    fn invokes_init_directory_with_cwd() {
        let run_command = |_cmd: &str,
                           dir: Option<&str>,
                           _stream_io: bool,
                           _env: Option<&HashMap<String, String>>| {
            if let Some(target_dir) = dir {
                if target_dir != "." {
                    return Err(None);
                }
                Ok(Some(String::from("")))
            } else {
                Ok(Some(String::from(".")))
            }
        };
        let write_file = |_file_path: &str, _contents: &str, _x: bool| Ok(());
        let file_exists = |_path: &str| Ok(false);
        let result = init(run_command, write_file, file_exists, vec![]);
        assert_eq!(result, Ok(()));
    }
}

#[cfg(test)]
mod run_tests {
    use super::utils::build_simple_command_runner;
    use super::*;

    #[tokio::test]
    async fn returns_error_when_root_directory_detect_fails() {
        let exp_err = "Failure determining git repo root directory";
        let run_command = build_simple_command_runner(Err(Some(String::from(exp_err))));
        let read_file = |_file_path: &str| panic!("");
        let file_exists = |_path: &str| panic!("");

        let result = run(run_command, file_exists, read_file, "")
            .await
            .unwrap_err();

        insta::assert_snapshot!(result, @"Failure determining git repo root directory");
    }

    #[tokio::test]
    async fn returns_error_when_config_file_missing() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Err(());
        let file_exists = |_path: &str| Ok(false);

        let result = run(run_command, file_exists, read_file, "")
            .await
            .unwrap_err();

        insta::assert_snapshot!(result, @"No config file found");
    }

    #[tokio::test]
    async fn returns_error_when_config_contents_unloadable() {
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Err(());
        let file_exists = |_path: &str| Ok(true);

        let result = run(run_command, file_exists, read_file, "")
            .await
            .unwrap_err();

        insta::assert_snapshot!(result, @"Failed to parse config file");
    }

    #[tokio::test]
    async fn returns_error_on_invalid_config() {
        let contents = "abc";
        let run_command = build_simple_command_runner(Ok(Some(String::from(""))));
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);

        let result = run(run_command, file_exists, read_file, "pre-push")
            .await
            .unwrap_err();

        insta::assert_snapshot!(result, @r"
        TOML parse error at line 1, column 4
          |
        1 | abc
          |    ^
        key with no value, expected `=`
        ");
    }

    #[tokio::test]
    async fn returns_err_when_script_fails() {
        let exp_err = "crashed";
        let contents = r#"[hooks]
            pre-commit = "cargo test"

            [logging]
            verbose = false
        "#;
        let run_command = |cmd: &str,
                           _dir: Option<&str>,
                           _stream_io: bool,
                           _env: Option<&HashMap<String, String>>| {
            if cmd == "cargo test" {
                return Err(Some(String::from(exp_err)));
            }
            Ok(Some(String::from("")))
        };
        let read_file = |_file_path: &str| Ok(String::from(contents));
        let file_exists = |_path: &str| Ok(true);
        let result = run(run_command, file_exists, read_file, "pre-commit")
            .await
            .unwrap_err();

        insta::assert_snapshot!(result, @r#"
        TOML parse error at line 2, column 26
          |
        2 |             pre-commit = "cargo test"
          |                          ^^^^^^^^^^^^
        invalid type: string "cargo test", expected a sequence
        "#);
    }
}
