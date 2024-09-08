use crate::ast::{index::WeakIndex, traits::GetChildren};

use super::{ast_node_value::Id, expression::Expression, statement::Statement};



#[derive(Debug, PartialEq, Clone, Default)]
pub struct BlockScope {
    pub kind: BlockScopeKind,
    pub statements: Vec<Id<Statement>>,
    pub return_expression: Option<Id<Expression>>,
}

impl BlockScope {
    pub fn new(kind: BlockScopeKind, declarations: Vec<Id<Statement>>, return_expression: Option<Id<Expression>>) -> Self {
        Self { kind, statements: declarations, return_expression }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum BlockScopeKind {
    Class,
    Function(WeakIndex),
    ForLoop,

    #[default]
    Local,
}

impl GetChildren for BlockScope {
    fn get_children(&self) -> Vec<WeakIndex> {
        let mut children = self.statements.iter().map(|d| d.as_weak()).collect::<Vec<_>>();

        if let Some(ret) = self.return_expression {
            children.push(ret.as_weak());
        }

        children
    }
}