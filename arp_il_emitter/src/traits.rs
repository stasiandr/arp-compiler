use arp_ast_processor::types::ast_node_value::{Ast, Id};

use crate::EmitError;



pub trait Visitor<T> {
    fn visit(&mut self, index: &Id<T>, ast: &Ast) -> Result<(), EmitError>;
}