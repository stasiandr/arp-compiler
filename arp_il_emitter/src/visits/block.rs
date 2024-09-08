use arp_ast_processor::types::{
    ast_node_value::{Ast, Id},
    block_scope::BlockScope,
};

use crate::{emitter::Emitter, traits::Visitor, EmitError};

impl Visitor<BlockScope> for Emitter {
    fn visit(&mut self, index: &Id<BlockScope>, ast: &Ast) -> Result<(), EmitError> {
        for stmt in &ast.get(index).statements {
            self.visit(stmt, ast)?;
        }

        Ok(())
    }
}
