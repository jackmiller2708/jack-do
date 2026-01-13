use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jack-do")]
#[command(about = "A developer productivity CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub domain: Domain,
}

#[derive(Subcommand)]
pub enum Domain {
    /// TypeScript related commands
    Typescript {
        #[command(subcommand)]
        command: TypescriptCommand,
    },
}

#[derive(Subcommand)]
pub enum TypescriptCommand {
    /// Remove unused declarations in the specified files
    RemoveUnusedDeclarations {
        /// Glob pattern to match files
        pattern: String,
    },
}
