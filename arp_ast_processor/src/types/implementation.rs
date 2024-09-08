use crate::ast::{index::WeakIndex, traits::GetChildren};

use super::{ast_node_value::Id, function::Function, type_collection::TypeId};



#[derive(Debug, PartialEq, Clone)]
pub struct Implementation {
    pub impl_type: TypeId,
    pub functions: Vec<Id<Function>>,
}


impl GetChildren for Implementation {
    fn get_children(&self) -> Vec<WeakIndex> {
        self.functions.iter().map(|n|n.as_weak()).collect()
    }
}