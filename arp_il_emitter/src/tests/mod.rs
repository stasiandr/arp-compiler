#![cfg(test)]

pub mod emitter;
pub mod writer;
mod multi_file;

use arp_ast_processor::{
    ast::traits::AstNodeKind, build_multiple_sources, types::ast_node_value::AstNodeValue,
};
use arp_types::sources::Source;

use crate::{emitter::Emitter, il_token::ILToken, EmitError};

fn test_emit<T: AstNodeKind<AstNodeValue>, S: Into<String>>(
    input: S,
) -> Result<Vec<ILToken>, EmitError> {
    let sources = [Source::new_inline("Main.arp", input)];
    let ast = build_multiple_sources(&sources).unwrap();
    let ast = dbg!(ast);
    let node = ast.get_child_of_kind::<T, _>(ast.get_root_index()).unwrap();

    Emitter::new().emit_node(&ast, node.as_weak())
}
