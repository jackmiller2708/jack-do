mod analyzer;
mod modifier;

use anyhow::{Context, Result};
use glob::glob;
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

use analyzer::UnusedDeclarationAnalyzer;
use modifier::apply_modifications_to_file;

pub async fn remove_unused_declarations(pattern: &str) -> Result<()> {
    info!("Removing unused declarations for pattern: {}", pattern);

    for entry in glob(pattern)? {
        match entry {
            Ok(path) => {
                info!("Processing file: {:?}", path.display());

                if let Err(e) = process_file(&path).await {
                    warn!("Failed to process file {:?}: {:?}", path, e);
                }
            }
            Err(e) => anyhow::bail!("Error reading glob entry: {:?}", e),
        }
    }

    Ok(())
}

async fn process_file(path: &Path) -> Result<()> {
    let source_text =
        fs::read_to_string(path).with_context(|| format!("Failed to read file {:?}", path))?;

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).unwrap_or_default();

    let oxc_parser::ParserReturn {
        program, errors, ..
    } = Parser::new(&allocator, &source_text, source_type).parse();

    if !errors.is_empty() {
        for error in errors {
            warn!("Parse error in {:?}: {:?}", path, error);
        }
        return Ok(());
    }

    let semantic_ret = SemanticBuilder::new().build(&program);
    let semantic = semantic_ret.semantic;

    let analyzer = UnusedDeclarationAnalyzer::new(&semantic);
    let unused_spans = analyzer.find_unused_spans();

    if unused_spans.is_empty() {
        return Ok(());
    }

    apply_modifications_to_file(path, &source_text, unused_spans)?;

    Ok(())
}
