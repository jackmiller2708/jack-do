use oxc_ast::AstKind;
use oxc_semantic::{NodeId, Semantic, SymbolId};
use oxc_span::{GetSpan, Span};
use std::collections::HashSet;

pub(crate) struct UnusedDeclarationAnalyzer<'a> {
    semantic: &'a Semantic<'a>,
    unused_symbol_ids: HashSet<SymbolId>,
}

impl<'a> UnusedDeclarationAnalyzer<'a> {
    pub(crate) fn new(semantic: &'a Semantic<'a>) -> Self {
        let mut unused_symbol_ids = HashSet::new();

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

    fn should_skip_symbol(semantic: &Semantic, symbol_id: SymbolId) -> bool {
        let name = semantic.symbols().get_name(symbol_id);
        name == "arguments" || name == "this"
    }

    fn is_symbol_used(semantic: &Semantic, symbol_id: SymbolId) -> bool {
        semantic
            .symbols()
            .get_resolved_references(symbol_id)
            .next()
            .is_some()
    }

    fn is_symbol_exported(semantic: &Semantic, symbol_id: SymbolId) -> bool {
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

    pub(crate) fn find_unused_spans(&self) -> Vec<Span> {
        let mut nodes_to_remove = HashSet::new();

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

    fn is_entire_variable_declaration_unused(&self, node_id: NodeId) -> bool {
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

    fn is_entire_import_declaration_unused(&self, node_id: NodeId) -> bool {
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
