use crate::{ast::{index::WeakIndex, traits::{AstNodeUnion, GetChildren, PushRemoveRootChildren}}, derive_implementations};

use super::{block_scope::BlockScope, declaration::Declaration, expression::Expression, file::ArpFile, function::Function, implementation::Implementation, statement::Statement, structure::Structure};



#[derive(Debug, PartialEq, Clone)]
pub enum AstNodeValue {
    Expression(Expression),
    Statement(Statement),
    BlockScope(BlockScope),
    Declaration(Declaration),
    Structure(Structure),
    Function(Function),
    Implementation(Implementation),
    ArpFile(ArpFile),

    Root(Vec<WId>),
}

impl Default for AstNodeValue {
    fn default() -> Self {
        AstNodeValue::Root(vec![])
    }
}


pub type Ast = crate::ast::AbstractAst<AstNodeValue>;
pub type Node = crate::ast::AbstractNode<AstNodeValue>;
pub type Id<T> = crate::ast::index::StrongIndex<T>;
pub type WId = crate::ast::index::WeakIndex;

derive_implementations!(AstNodeValue, AstNodeValue::Expression, Expression);
derive_implementations!(AstNodeValue, AstNodeValue::Statement, Statement);
derive_implementations!(AstNodeValue, AstNodeValue::BlockScope, BlockScope);

derive_implementations!(AstNodeValue, AstNodeValue::Declaration, Declaration);
derive_implementations!(AstNodeValue, AstNodeValue::Structure, Structure);
derive_implementations!(AstNodeValue, AstNodeValue::Function, Function);
derive_implementations!(AstNodeValue, AstNodeValue::Implementation, Implementation);
derive_implementations!(AstNodeValue, AstNodeValue::ArpFile, ArpFile);


impl AstNodeUnion for AstNodeValue { }

impl GetChildren for AstNodeValue {
    fn get_children(&self) -> Vec<WId> {
        match self {
            AstNodeValue::Expression(value) => value.get_children(),
            AstNodeValue::Statement(value) => value.get_children(),
            AstNodeValue::BlockScope(value) => value.get_children(),
            AstNodeValue::Function(value) => value.get_children(),
            AstNodeValue::Structure(value) => value.get_children(),
            AstNodeValue::Declaration(value) => value.get_children(),
            AstNodeValue::Implementation(value) => value.get_children(),

            AstNodeValue::Root(children) => children.clone(),
            AstNodeValue::ArpFile(value) => value.get_children(),
        }
    }
}

impl PushRemoveRootChildren for AstNodeValue {
    fn push_child(&mut self, index: WeakIndex) {
        match self {
            AstNodeValue::Root(vec) => vec.push(index),
            _ => unreachable!("It's not a root"),
        }
    }

    fn remove_child(&mut self, index: WeakIndex) {
        match self {
            AstNodeValue::Root(vec) => {
                vec.remove(vec.iter().position(|i| *i == index).expect("Node not found"));
            },
            _ => unreachable!("It's not a root"),
        }
    }
}

impl Ast {
    pub fn get_arp_file_in_parent(&self, index: WeakIndex) -> Option<&ArpFile> {
        self.get_parent_of_kind::<ArpFile, _>(index).map(|id| self.get(&id))
    }

    pub fn get_mut_arp_file_in_parent(&mut self, index: WeakIndex) -> Option<&mut ArpFile> {
        self.get_parent_of_kind::<ArpFile, _>(index).map(|id| self.get_mut(&id))
    }
    
}