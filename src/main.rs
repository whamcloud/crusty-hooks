use clap::Parser;
use crusty_hooks::HOOK_NAMES;
use std::{
    env,
    process::{ExitCode, exit},
};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[derive(Parser)]
#[clap(author, about, version)]
enum RustyHookOpts {
    /// Initialize crusty-hooks' git hooks in the current directory.
    #[clap(author, version)]
    Init {
        #[clap(long)]
        skip_hook_list: Option<String>,
    },
    /// Print the current version of crusty-hooks.
    #[clap(author, version, alias = "-v")]
    Version,
    /// Run a git hook using the current directory's configuration.
    /// Ran automatically by crusty-hooks' git hooks.
    #[clap(author, version)]
    Run {
        #[clap(long, value_parser = clap::builder::PossibleValuesParser::new(HOOK_NAMES))]
        hook: String,
    },
}

fn init(skip_hook_list: Option<String>) {
    if ci_info::is_ci() {
        println!("[crusty-hooks] CI Environment detected. Skipping hook install");

        return;
    }

    let skip_hook_list = skip_hook_list
        .as_deref()
        .map_or(vec![], |s| s.split(',').collect());

    if let Err(err) = crusty_hooks::init(
        nias::get_command_runner(),
        nias::get_file_writer(),
        nias::get_file_existence_checker(),
        skip_hook_list,
    ) {
        eprintln!(
            "[crusty-hooks] Fatal error encountered during initialization. Details: {}",
            err
        );
        exit(1);
    };
}

async fn run(hook: String) -> ExitCode {
    let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer)
        .init();

    if let Err(err) = crusty_hooks::run(
        nias::get_command_runner(),
        nias::get_file_existence_checker(),
        nias::get_file_reader(),
        &hook,
    )
    .await
    {
        match err {
            err if err.to_string() == crusty_hooks::NO_CONFIG_FILE_FOUND => {
                return ExitCode::from(crusty_hooks::NO_CONFIG_FILE_FOUND_ERROR_CODE);
            }
            err => {
                tracing::error!("{err}");

                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

#[tokio::main]
async fn main() -> ExitCode {
    let opts = RustyHookOpts::parse();

    match opts {
        RustyHookOpts::Init { skip_hook_list } => init(skip_hook_list),
        RustyHookOpts::Version => println!(env!("CARGO_PKG_VERSION")),
        RustyHookOpts::Run { hook } => return run(hook).await,
    };

    ExitCode::SUCCESS
}
