use crate::ast::traits::GetChildren;

use super::{ast_node_value::Id, function::Function, implementation::Implementation, simple::Identifier, structure::Structure, type_collection::TypeCollection};



#[derive(Debug, PartialEq, Clone, Default)]
pub struct ArpFile {
    pub imports: Vec<Import>,
    pub structures: Vec<Id<Structure>>,
    pub implementations: Vec<Id<Implementation>>,
    pub functions: Vec<Id<Function>>,
    pub type_collection: TypeCollection,
    pub arp_path: ArpPath
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ArpPath(pub String);

impl<T : AsRef<str>> From<T>  for ArpPath {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub is_extern: bool,
    pub path: Box<str>,
    pub import_types: Vec<Identifier>,
}

impl GetChildren for ArpFile {
    fn get_children(&self) -> Vec<crate::ast::index::WeakIndex> {
        self.structures.iter().map(|n| n.as_weak())
            .chain(self.implementations.iter().map(|n| n.as_weak()))
            .chain(self.functions.iter().map(|n| n.as_weak()))
            .collect()
    }
}