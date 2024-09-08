mod tests;
pub mod errors;
pub mod ast;
pub mod types;
pub mod chumsky_folder;
pub mod validations;
pub mod type_resolver;
mod post;
pub mod utils;

use arp_parser::types::ChumskyNode;
use arp_types::{sources::Source, Spanned};
use chumsky_folder::ChumskyNodeVisitor;
use errors::ProcessingError;
use post::post_process;
use type_resolver::{imports_graph::resolve_imports, resolve_types_loop};
use types::ast_node_value::Ast;


pub fn process_file_ast(root: Box<Spanned<ChumskyNode>>) -> Result<Ast, ProcessingError> {
    let mut visitor = ChumskyNodeVisitor::default();
    visitor.fold_file(&root);
    let ast = visitor.consume();
    
    let ast = resolve_types_loop(ast)?;
    validations::validate(&ast)?;
    Ok(ast)
}

pub fn build_multiple_sources(sources: &[Source]) -> Result<Ast, ProcessingError> {
    if sources.is_empty() { return Err(ProcessingError::NotSourcesProvided) }

    let mut visitor = ChumskyNodeVisitor::default();

    for source in sources {
        let tokens = arp_lexer::lex_tokens(source).unwrap();
        let chumsky_root = arp_parser::parse_arp_file(source.len(), &tokens).unwrap();
        visitor.fold_file_with_path(&chumsky_root, source.get_module_string());
    }

    let ast = visitor.consume();
    let ast = resolve_imports(ast)?;
    let ast = resolve_types_loop(ast)?;
    let ast = post_process(ast)?;

    validations::validate(&ast)?;
    
    Ok(ast)
}