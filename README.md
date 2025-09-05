# crusty-hooks
Git hook utility for Rust codebases that lets you run any script for any git hook.

This is a fork of [rusty-hook](https://github.com/swellaby/rusty-hook) which seems to be unmaintained.

[![Version Badge][version-badge]][crate url]
[![License Badge][license-badge]][crate url]

## Quick Start
Pre-requisites: Make sure you have Rust installed and that Cargo's bin directory is on your PATH.
https://www.rust-lang.org/tools/install

1. Add `crusty-hooks` as a dev dependency in your Cargo.toml file
2. Run `cargo test` (to build your dev dependencies, including `crusty-hooks`)
3. Update the generated `.crusty-hooks.toml` file with the commands you want to run
4. Run `git commit` (or equivalent to trigger your git hook)!
    - You may also want to have your hook script fail (for example add a failing test if your commit hook is `cargo test`) to see the hooks be enforced.
    - **note the very first (and only) time you do this will take an extra ~30 seconds or so to finalize the setup**

## Setup
Just add `crusty-hooks` as a dev dependency in your Cargo.toml file:

```toml
[dev-dependencies]
crusty-hooks = "0.1"
```

## Initialize
When you add `crusty-hooks` as a dev-dependency in your project, it will automatically configure the git hooks once it is built (for example the first time you run `cargo test`).

This will ensure that all of the client side git hooks are setup and available, and it will create a `crusty-hooks` configuration file if one does not already exist.

The git hook script will ensure that the `crusty-hooks` cli is available, so the very first time a git hook is triggered on your machine you will see a message indicating that the `crusty-hooks` setup is being finalized which may take ~30 seconds or so:
```sh
Finalizing crusty-hooks configuration...
This may take a few seconds...
```

### (Optional) Install
You can also install the `crusty-hooks` cli with cargo:
```sh
cargo install crusty-hooks
```

Or you can directly install it from source
```sh
cargo install --path=cmd
```

You can optionally manually initialize any git directory by running the `init` command in any git directory to set it up:

```sh
crusty-hooks init
```

## Configure
You define your desired [git hook][git hooks] configuration in the `crusty-hooks` configuration file (a TOML file named `.crusty-hooks.toml` or `crusty-hooks.toml`).

Here's an example `crusty-hooks` configuration that leverages multiple [git hooks][git hooks], including the [pre-commit][pre-commit hook] and the [pre-push][pre-push hook] hooks:

```toml
[hooks]
pre-commit = "cargo test"
pre-push = ["cargo check", "cargo fmt -- --check"]
post-commit = "echo yay"

[logging]
verbose = true
```
### Hooks
Under the `[hooks]` table, you can add an entry for any and every git hook you want to run by adding a key using the name of the [git hook][git hooks], and then specify the command/script you want to run for that hook. Multiple commands in a form of a toml array or via command chaining using `&&` are also allowed (Only for versions 0.12 and up). Whenever that git hook is triggered, `crusty-hooks` will run your specified command!

#### Using git arguments
In git hook commands, any instance of `%rh!` will be replaced by the arguments that git passes to this hook.

```toml
[hooks]
pre-push = "echo %rh!"
```

### Logging
Under the `[logging]` table, you can control whether to log the output of running your specified hook commands. By default `crusty-hooks` will log the results of your hook script, but you can disable this behavior by setting the `verbose` key to `false`:

```toml
[logging]
verbose = false
```

## Alternatives
There's a few other git hook utilities available on [crates.io][cratesio], but none of them quite suited our needs so we made crusty-hooks!

* [cargo-husky][cargo-husky crate]
* [shiba][shiba crate]
* [git_hooks][git_hooks crate]

Note: all of these seem to be unmaintained.

## Contributions
All contributions are welcome and appreciated! Check out our [Contributing Guidelines][contributing] for more information about opening issues, developing, and more.

## Changelog
[See CHANGELOG.md](https://github.com/whamcloud/crusty-hooks/blob/master/CHANGELOG.md)

## Removing crusty-hooks
We'll be sad to see you go, but here's what to do if you'd like to remove `crusty-hooks` from your project.

1. Remove the `crusty-hooks` dev dependency from the `Cargo.toml` file in your project.
2. Remove the `.crusty-hooks.toml` configuration file from your project.
3. Remove the git hook scripts that were placed in the git hooks directory in your local project workspace (this is typically in the `.git/hooks/` directory). Note that if you skip this step, then the git hooks will still be invoked as part of your git workflow and you will see the following warning message on git commit:

```console
crusty-hooks git hooks are configured, but no config file was found
In order to use crusty-hooks, your project must have a config file
See https://github.com/whamcloud/crusty-hooks#configure for more information about configuring crusty-hooks

If you were trying to remove crusty-hooks, then you should also delete the git hook files to remove this warning
See https://github.com/whamcloud/crusty-hooks#removing-rusty-hook for more information about removing crusty-hooks from your project
```

Please also consider [opening an issue][create-issue] to report any bugs/problems you experienced, missing features, etc. so that we can work on improving `crusty-hooks`!

[version-badge]: https://img.shields.io/crates/v/crusty-hooks.svg?style=flat-square
[license-badge]: https://img.shields.io/crates/l/crusty-hooks.svg?style=flat-square
[crate url]: https://crates.io/crates/crusty-hooks
[linux-ci-badge]: https://img.shields.io/github/workflow/status/whamcloud/crusty-hooks/linux/main?label=linux%20build&style=flat-square
[git hooks]: https://git-scm.com/docs/githooks#_hooks
[pre-commit hook]: https://git-scm.com/docs/githooks#_pre_commit
[pre-push hook]: https://git-scm.com/docs/githooks#_pre_push
[cargo-husky crate]: https://crates.io/crates/cargo-husky
[shiba crate]: https://crates.io/crates/shiba
[git_hooks crate]: https://crates.io/crates/git_hooks
[cratesio]: https://crates.io
[contributing]: .github/CONTRIBUTING.md
[create-issue]: https://github.com/whamcloud/crusty-hooks/issues/new/choose
