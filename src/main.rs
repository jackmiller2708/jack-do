mod cli;
mod domains;

use clap::Parser;
use cli::{Cli, Domain, TypescriptCommand};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.domain {
        Domain::Typescript { command } => match command {
            TypescriptCommand::RemoveUnusedDeclarations { pattern } => {
                domains::typescript::remove_unused_declarations(&pattern).await?;
            }
        },
    }

    Ok(())
}
