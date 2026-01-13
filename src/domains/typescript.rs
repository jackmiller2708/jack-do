use anyhow::{Context, Result};
use glob::glob;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_parser::{Parser, ParserReturn};
use oxc_semantic::SemanticBuilder;
use oxc_span::{GetSpan, SourceType};

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

    let ParserReturn {
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

struct UnusedDeclarationAnalyzer<'a> {
    semantic: &'a oxc_semantic::Semantic<'a>,
    unused_symbol_ids: std::collections::HashSet<oxc_semantic::SymbolId>,
}

impl<'a> UnusedDeclarationAnalyzer<'a> {
    fn new(semantic: &'a oxc_semantic::Semantic<'a>) -> Self {
        let mut unused_symbol_ids = std::collections::HashSet::new();

        for symbol_id in semantic.symbols().symbol_ids() {
            if Self::should_skip_symbol(semantic, symbol_id) {
                continue;
            }

            if !Self::is_symbol_used(semantic, symbol_id)
                && !Self::is_symbol_exported(semantic, symbol_id)
            {
                unused_symbol_ids.insert(symbol_id);
            }
        }

        Self {
            semantic,
            unused_symbol_ids,
        }
    }

    fn should_skip_symbol(
        semantic: &oxc_semantic::Semantic,
        symbol_id: oxc_semantic::SymbolId,
    ) -> bool {
        let name = semantic.symbols().get_name(symbol_id);
        name == "arguments" || name == "this"
    }

    fn is_symbol_used(
        semantic: &oxc_semantic::Semantic,
        symbol_id: oxc_semantic::SymbolId,
    ) -> bool {
        semantic
            .symbols()
            .get_resolved_references(symbol_id)
            .next()
            .is_some()
    }

    fn is_symbol_exported(
        semantic: &oxc_semantic::Semantic,
        symbol_id: oxc_semantic::SymbolId,
    ) -> bool {
        let node_id = semantic.symbols().get_declaration(symbol_id);
        let mut current_node_id = Some(node_id);

        while let Some(current_id) = current_node_id {
            let node = semantic.nodes().get_node(current_id);
            match node.kind() {
                AstKind::ExportNamedDeclaration(_)
                | AstKind::ExportDefaultDeclaration(_)
                | AstKind::ExportAllDeclaration(_) => return true,
                _ => {}
            }
            current_node_id = semantic.nodes().parent_id(current_id);
        }
        false
    }

    fn find_unused_spans(&self) -> Vec<oxc_span::Span> {
        let mut nodes_to_remove = std::collections::HashSet::new();

        for &symbol_id in &self.unused_symbol_ids {
            let decl_node_id = self.semantic.symbols().get_declaration(symbol_id);
            let decl_node = self.semantic.nodes().get_node(decl_node_id);

            match decl_node.kind() {
                AstKind::VariableDeclarator(_) => {
                    if let Some(parent_id) = self.semantic.nodes().parent_id(decl_node_id) {
                        if self.is_entire_variable_declaration_unused(parent_id) {
                            nodes_to_remove.insert(parent_id);
                            continue;
                        }
                    }
                    nodes_to_remove.insert(decl_node_id);
                }
                AstKind::BindingRestElement(_) | AstKind::BindingIdentifier(_) => {
                    nodes_to_remove.insert(decl_node_id);
                }
                AstKind::ImportSpecifier(_)
                | AstKind::ImportDefaultSpecifier(_)
                | AstKind::ImportNamespaceSpecifier(_) => {
                    if let Some(parent_id) = self.semantic.nodes().parent_id(decl_node_id) {
                        if self.is_entire_import_declaration_unused(parent_id) {
                            nodes_to_remove.insert(parent_id);
                            continue;
                        }
                    }
                    nodes_to_remove.insert(decl_node_id);
                }
                _ => {
                    nodes_to_remove.insert(decl_node_id);
                }
            }
        }

        let mut spans: Vec<_> = nodes_to_remove
            .into_iter()
            .map(|id| self.semantic.nodes().get_node(id).kind().span())
            .collect();

        spans.sort_by_key(|s| s.start);
        spans.dedup_by(|a, b| a.start == b.start && a.end == b.end);
        spans.reverse();
        spans
    }

    fn is_entire_variable_declaration_unused(&self, node_id: oxc_semantic::NodeId) -> bool {
        let node = self.semantic.nodes().get_node(node_id);

        if let AstKind::VariableDeclaration(var_decl) = node.kind() {
            return var_decl
                .declarations
                .iter()
                .all(|d| self.is_pattern_entirely_unused(&d.id));
        }
        false
    }

    fn is_pattern_entirely_unused(&self, pattern: &oxc_ast::ast::BindingPattern) -> bool {
        match &pattern.kind {
            oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) => ident
                .symbol_id
                .get()
                .map_or(false, |sid| self.unused_symbol_ids.contains(&sid)),
            oxc_ast::ast::BindingPatternKind::ObjectPattern(obj) => obj
                .properties
                .iter()
                .all(|prop| self.is_pattern_entirely_unused(&prop.value)),
            oxc_ast::ast::BindingPatternKind::ArrayPattern(arr) => arr.elements.iter().all(|el| {
                el.as_ref()
                    .map_or(true, |e| self.is_pattern_entirely_unused(e))
            }),
            _ => false,
        }
    }

    fn is_entire_import_declaration_unused(&self, node_id: oxc_semantic::NodeId) -> bool {
        let node = self.semantic.nodes().get_node(node_id);

        if let AstKind::ImportDeclaration(imp_decl) = node.kind() {
            return imp_decl.specifiers.as_ref().map_or(true, |specs| {
                specs.iter().all(|spec| {
                    let spec_symbol_id = match spec {
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) => {
                            s.local.symbol_id.get()
                        }
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                            s.local.symbol_id.get()
                        }
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                            s.local.symbol_id.get()
                        }
                    };
                    spec_symbol_id.map_or(false, |sid| self.unused_symbol_ids.contains(&sid))
                })
            });
        }
        false
    }
}

fn apply_modifications_to_file(
    path: &Path,
    source: &str,
    spans: Vec<oxc_span::Span>,
) -> Result<()> {
    let mut new_source = source.to_string();

    for span in spans {
        let (expanded_start, expanded_end) =
            expand_span_for_removal(&new_source, span.start as usize, span.end as usize);

        new_source.replace_range(expanded_start..expanded_end, "");
        info!("Removed expanded span {}..{}", expanded_start, expanded_end);
    }

    fs::write(path, new_source).with_context(|| format!("Failed to write file {:?}", path))?;
    info!("Updated file: {:?}", path);
    Ok(())
}

fn expand_span_for_removal(source: &str, start: usize, end: usize) -> (usize, usize) {
    let mut s = start;
    let mut e = end;

    // 1. Try to consume trailing comma and whitespace until newline
    let mut temp_e = e;
    let bytes = source.as_bytes();

    while temp_e < bytes.len() {
        match bytes[temp_e] {
            b' ' | b'\t' | b'\r' => temp_e += 1,
            b',' => {
                temp_e += 1;
                e = temp_e;
                // After comma, also consume whitespace until next real char or newline
                while e < bytes.len()
                    && (bytes[e] == b' ' || bytes[e] == b'\t' || bytes[e] == b'\r')
                {
                    e += 1;
                }
                break;
            }
            b'\n' => {
                e = temp_e + 1; // Include newline
                break;
            }
            _ => break,
        }
    }

    // 2. If no trailing comma, try leading comma
    if s == start && e == end {
        let mut temp_s = s;

        while temp_s > 0 {
            temp_s -= 1;

            match bytes[temp_s] {
                b' ' | b'\t' | b'\r' => {}
                b',' => {
                    s = temp_s;
                    break;
                }
                _ => break,
            }
        }
    }

    // 3. If it's a whole line (or multiple), consume leading whitespace as well
    let mut temp_s = s;

    while temp_s > 0 && (bytes[temp_s - 1] == b' ' || bytes[temp_s - 1] == b'\t') {
        temp_s -= 1;
    }

    // Only expand if we are at the start of the line or only whitespace precedes
    if temp_s == 0 || bytes[temp_s - 1] == b'\n' {
        s = temp_s;
    }

    (s, e)
}
