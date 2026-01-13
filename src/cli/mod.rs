mod typescript;

pub(crate) use typescript::TypescriptCommand;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jack-do")]
#[command(about = "A developer productivity CLI", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) domain: Domain,
}

#[derive(Subcommand)]
pub(crate) enum Domain {
    /// TypeScript related commands
    Typescript {
        #[command(subcommand)]
        command: TypescriptCommand,
    },
}
