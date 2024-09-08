use crate::ast::traits::GetChildren;

use super::type_collection::TypeId;



#[derive(Debug, PartialEq, Clone)]
pub struct Structure {
    pub kind: StructureKind,
    pub self_type: TypeId,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructureKind {
    Class,
    Struct,
    Union,
}

impl GetChildren for Structure {
    fn get_children(&self) -> Vec<crate::ast::index::WeakIndex> {
        vec![]
    }
}