use thiserror::Error;

use crate::{type_resolver::TypeResolverError, types::ast_node_value::{Ast, AstNodeValue, Id}};

#[derive(Error, Debug)]
pub enum ValidationError {

    #[error("Variable has undeclared type")]
    VariableHasUndeclaredType(Box<str>),

    #[error("Variable has weak type")]
    VariableHasWeakType(Box<str>),

    #[error("Can't resolve expression type")]
    TypeResolverError(#[from] TypeResolverError),

    #[error("")]
    StatementOutsideFunction,
    
    #[error("")]
    IdentifierHasNoRegister,
}


pub trait Validate {
    fn validate(&self, index: Id<Self>, ast: &Ast) -> Result<(), ValidationError>
    where Self: std::marker::Sized;
}

pub fn validate(ast: &Ast) -> Result<(), ValidationError> {
    for node in ast.rec_iter_start_from(ast.get_root_index()).flat_map(|(id, _)| ast.get_weak(id)) {
        match node.get_value() {
            AstNodeValue::Expression(expr) => expr.validate(ast.try_promote(node.get_index()).unwrap(), ast)?,
            AstNodeValue::Statement(stmt) => stmt.validate(ast.try_promote(node.get_index()).unwrap(), ast)?,
            AstNodeValue::BlockScope(_) => {},
            AstNodeValue::Declaration(_) => {},
            AstNodeValue::Structure(_) => {},
            AstNodeValue::Function(_) => {},
            AstNodeValue::Implementation(_) => {},
            AstNodeValue::ArpFile(_) => {},
            AstNodeValue::Root(_) => {},
        }
    }
    Ok(())
}