use std::{env, path::Path};

use docweave_engine::{
    adapters::{LanguageAdapter, diagnostics, node_text, source_span},
    document_project_with_adapters,
    model::{DocFile, DocSymbol, SymbolKind},
};
use tree_sitter::{Language, Node, Tree};

struct DemoAdapter;

impl LanguageAdapter for DemoAdapter {
    fn id(&self) -> &'static str {
        "demo"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["demo"]
    }

    fn language(&self) -> Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
        let path_text = path.display().to_string();
        let mut symbols = Vec::new();
        collect_functions(tree.root_node(), source, &path_text, &mut symbols);
        DocFile {
            path: path_text.clone(),
            documentation: None,
            symbols,
            diagnostics: diagnostics(tree.root_node(), &path_text, self.id()),
        }
    }
}

fn collect_functions(node: Node<'_>, source: &str, path: &str, out: &mut Vec<DocSymbol>) {
    if node.kind() == "function_item"
        && let Some(name_node) = node.child_by_field_name("name")
        && let Some(name) = node_text(name_node, source)
    {
        let signature_end = (0..node.child_count())
            .filter_map(|index| node.child(index))
            .find(|child| child.kind() == "block")
            .map_or(node.end_byte(), |body| body.start_byte());
        out.push(DocSymbol {
            kind: SymbolKind::Function,
            name: name.to_owned(),
            signature: source
                .get(node.start_byte()..signature_end)
                .map(str::trim)
                .map(str::to_owned),
            documentation: leading_docs(node, source),
            span: source_span(node, path),
        });
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_functions(child, source, path, out);
    }
}

fn leading_docs(node: Node<'_>, source: &str) -> Option<String> {
    let prefix = source.get(..node.start_byte())?.trim_end();
    let mut lines: Vec<_> = prefix
        .lines()
        .rev()
        .take_while(|line| line.trim_start().starts_with("///"))
        .map(|line| {
            let body = line.trim_start().strip_prefix("///").unwrap_or_default();
            body.strip_prefix(' ').unwrap_or(body)
        })
        .collect();
    lines.reverse();
    (!lines.is_empty()).then(|| lines.join("\n"))
}

fn main() -> anyhow::Result<()> {
    let root = env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/demo-sources".to_owned());
    let adapters: [&dyn LanguageAdapter; 1] = [&DemoAdapter];
    let project = document_project_with_adapters(Path::new(&root), &adapters)?;
    for file in project.files {
        println!("{}", file.path);
        for symbol in file.symbols {
            println!("  {:?} {}", symbol.kind, symbol.name);
        }
        for diagnostic in file.diagnostics {
            eprintln!("  {}: {}", diagnostic.code, diagnostic.message);
        }
    }
    Ok(())
}
