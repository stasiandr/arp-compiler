use crate::ast::{index::WeakIndex, traits::GetChildren};

use super::{ast_node_value::Id, file::ArpFile, function::Function, implementation::Implementation, structure::Structure};


#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Implementation(Id<Implementation>),
    File(Id<ArpFile>),
    Function(Id<Function>),
    Structure(Id<Structure>),
}


impl GetChildren for Declaration {
    fn get_children(&self) -> Vec<WeakIndex> {
        match self {
            Declaration::Function(child) => vec![child.as_weak()],
            Declaration::Structure(child) =>  vec![child.as_weak()],
            Declaration::File(child) =>  vec![child.as_weak()],
            Declaration::Implementation(child) =>  vec![child.as_weak()],
        }
    }
}