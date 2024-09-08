use arp_ast_processor::types::ast_node_value::Ast;
use emitter::Emitter;
use il_token::ILToken;
use thiserror::Error;

pub mod emitter;
pub mod file_writer;
pub mod il_token;
mod tests;
pub mod traits;
pub mod utils;
pub mod visits;

#[derive(Error, Debug)]
pub enum EmitError {
    #[error("Unknown")]
    Unknown,
    #[error("")]
    VariableHasNoSource,

    #[error("")]
    CantResolveType,

    #[error("")]
    ArpFileNotFound,
    
    #[error("")]
    CantResolveMethod,
    
    #[error("")]
    CantResolveField,
    
    #[error("")]
    UnsupportedIfChain,
}

pub fn emit_tokens(ast: Ast) -> Result<Vec<ILToken>, EmitError> {
    Emitter::new().emit(&ast)
}
