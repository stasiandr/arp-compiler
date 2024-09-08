use std::vec;

use crate::ast::{index::WeakIndex, traits::GetChildren};

use super::{ast_node_value::Id, block_scope::BlockScope, simple::Identifier, type_collection::TypeId};




#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub kind: FunctionKind,
    pub name: Identifier,
    pub parameters: Vec<(Identifier, TypeId)>,
    pub return_type: TypeId,
    pub block: Id<BlockScope>,

    pub registers: Vec<(Identifier, TypeId)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionKind {
    Method {
        is_this_mutable: bool
    },
    Static,
}

impl GetChildren for Function {
    fn get_children(&self) -> Vec<WeakIndex> {
        vec![self.block.as_weak()]
    }
}

impl Function {
    pub fn register_index_of(&self, ident: &Identifier) -> Option<usize> {
        self.registers.iter().position(|i| i.0 == *ident)
    }
    
}