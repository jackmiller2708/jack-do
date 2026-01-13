mod cli;
mod typescript;

use clap::Parser;
use cli::{Cli, Domain, TypescriptCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.domain {
        Domain::Typescript { command } => match command {
            TypescriptCommand::RemoveUnusedDeclarations { pattern } => {
                typescript::remove_unused_declarations(&pattern).await?;
            }
        },
    }

    Ok(())
}
