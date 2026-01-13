use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum TypescriptCommand {
    /// Remove unused declarations in the specified files
    RemoveUnusedDeclarations {
        /// Glob pattern to match files
        pattern: String,
    },
}
