# rusty-hooks
Git hook utility for Rust codebases that lets you run any script for any git hook.
This is forked from [rusty-hook](https://github.com/swellaby/rusty-hook) because that crate is unmaintend.

Functional, but still in Beta!

[![Version Badge][version-badge]][crate url]
[![Downloads Badge][downloads-badge]][crate url]
[![License Badge][license-badge]][crate url]

## Quick Start
Pre-requisites: Make sure you have Rust installed and that Cargo's bin directory is on your PATH.
https://www.rust-lang.org/tools/install

1. Add `rusty-hooks` as a dev dependency in your Cargo.toml file
2. Run `cargo test` (to build your dev dependencies, including `rusty-hooks`)
3. Update the generated `.rusty-hooks.toml` file with the commands you want to run
4. Run `git commit` (or equivalent to trigger your git hook)!
    - You may also want to have your hook script fail (for example add a failing test if your commit hook is `cargo test`) to see the hooks be enforced.
    - **note the very first (and only) time you do this will take an extra ~30 seconds or so to finalize the setup**

## Setup
Just add `rusty-hooks` as a dev dependency in your Cargo.toml file:

```toml
[dev-dependencies]
rusty-hooks = "^0.12"
```

## Initialize
When you add `rusty-hooks` as a dev-dependency in your project, it will automatically configure the git hooks once it is built (for example the first time you run `cargo test`).

This will ensure that all of the client side git hooks are setup and available, and it will create a `rusty-hooks` configuration file if one does not already exist.

The git hook script will ensure that the `rusty-hooks` cli is available, so the very first time a git hook is triggered on your machine you will see a message indicating that the `rusty-hooks` setup is being finalized which may take ~30 seconds or so:
```sh
Finalizing rusty-hooks configuration...
This may take a few seconds...
```

### (Optional) Install
You can also install the `rusty-hooks` cli with cargo:
```sh
cargo install rusty-hooks
```

You can optionally manually initialize any git directory by running the `init` command in any git directory to set it up:

```sh
rusty-hooks init
```

## Configure
You define your desired [git hook][git hooks] configuration in the `rusty-hooks` configuration file (a TOML file named `.rusty-hooks.toml` or `rusty-hooks.toml`).

Here's an example `rusty-hooks` configuration that leverages multiple [git hooks][git hooks], including the [pre-commit][pre-commit hook] and the [pre-push][pre-push hook] hooks:

```toml
[hooks]
pre-commit = "cargo test"
pre-push = ["cargo check", "cargo fmt -- --check"]
post-commit = "echo yay"

[logging]
verbose = true
```
### Hooks
Under the `[hooks]` table, you can add an entry for any and every git hook you want to run by adding a key using the name of the [git hook][git hooks], and then specify the command/script you want to run for that hook. Multiple commands in a form of a toml array or via command chaining using `&&` are also allowed (Only for versions 0.12 and up). Whenever that git hook is triggered, `rusty-hooks` will run your specified command!

#### Using git arguments
In git hook commands, any instance of `%rh!` will be replaced by the arguments that git passes to this hook.

```toml
[hooks]
pre-push = "echo %rh!"
```

### Logging
Under the `[logging]` table, you can control whether to log the output of running your specified hook commands. By default `rusty-hooks` will log the results of your hook script, but you can disable this behavior by setting the `verbose` key to `false`:

```toml
[logging]
verbose = false
```

## Alternatives
There's a few other git hook utilities available on [crates.io][cratesio], but none of them quite suited our needs so we made rusty-hooks!

* [cargo-husky][cargo-husky crate]
* [shiba][shiba crate]
* [git_hooks][git_hooks crate]

Note: all of these seem to be unmaintained.

## Contributions
All contributions are welcome and appreciated! Check out our [Contributing Guidelines][contributing] for more information about opening issues, developing, and more.

## Changelog
[See CHANGELOG.md](https://github.com/kaimast/rusty-hooks/blob/master/CHANGELOG.md)

## Removing rusty-hooks
We'll be sad to see you go, but here's what to do if you'd like to remove `rusty-hooks` from your project.

1. Remove the `rusty-hooks` dev dependency from the `Cargo.toml` file in your project.
2. Remove the `.rusty-hooks.toml` configuration file from your project.
3. Remove the git hook scripts that were placed in the git hooks directory in your local project workspace (this is typically in the `.git/hooks/` directory). Note that if you were using `rusty-hooks` version `0.9.1` or newer and you skip this step, then the git hooks will still be invoked as part of your git workflow and you will see the following warning message on git commit:
```
rusty-hooks git hooks are configured, but no config file was found
In order to use rusty-hooks, your project must have a config file
See https://github.com/kaimast/rusty-hooks#configure for more information about configuring rusty-hooks

If you were trying to remove rusty-hooks, then you should also delete the git hook files to remove this warning
See https://github.com/kaimast/rusty-hooks#removing-rusty-hook for more information about removing rusty-hooks from your project
```

Please also consider [opening an issue][create-issue] to report any bugs/problems you experienced, missing features, etc. so that we can work on improving `rusty-hooks`!

[crate url]: https://crates.io/crates/rusty-hooks
[linux-ci-badge]: https://img.shields.io/github/workflow/status/kaimast/rusty-hooks/linux/main?label=linux%20build&style=flat-square
[git hooks]: https://git-scm.com/docs/githooks#_hooks
[pre-commit hook]: https://git-scm.com/docs/githooks#_pre_commit
[pre-push hook]: https://git-scm.com/docs/githooks#_pre_push
[cargo-husky crate]: https://crates.io/crates/cargo-husky
[shiba crate]: https://crates.io/crates/shiba
[git_hooks crate]: https://crates.io/crates/git_hooks
[cratesio]: https://crates.io
[contributing]: .github/CONTRIBUTING.md
[create-issue]: https://github.com/swellaby/rusty-hook/issues/new/choose
