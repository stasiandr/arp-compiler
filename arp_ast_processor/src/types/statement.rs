use crate::{ast::{index::WeakIndex, traits::GetChildren}, validations::{Validate, ValidationError}};

use super::{ast_node_value::{Ast, Id}, block_scope::BlockScope, expression::Expression, function::Function, simple::Identifier, type_collection::TypeId};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Id<Expression>),
    Assignment {
        on: Option<Id<Expression>>,
        field: Identifier,
        expr: Id<Expression>,
    },
    LocalVariableDeclaration {
        is_mutable: bool,
        ident: Identifier,
        ty: TypeId,
        expr: Id<Expression>,
    },
    IfStmt(Vec<IfKind>),
    WhileStmt{
        expr: Id<Expression>,
        block: Id<BlockScope>
    },
    ForStmt{
        ident: Identifier,
        enumerable: Id<Expression>,
        block: Id<BlockScope>,
    },
    Block(Id<BlockScope>),
    Return(Id<Expression>),
}


impl Validate for Statement {
    fn validate(&self, index: Id<Statement>, ast: &Ast) -> Result<(), ValidationError> { 
        match self {
            Statement::Expression(_) => Ok(()),
            Statement::Assignment { .. } => {
                Ok(())
            },
            Statement::LocalVariableDeclaration { ident, ty, .. } => {

                let func = ast.get_parent_of_kind::<Function, _>(index).ok_or(ValidationError::StatementOutsideFunction)?;
                ast.get(&func).register_index_of(ident).ok_or(ValidationError::IdentifierHasNoRegister)?;


                if ty.is_none() | ty.is_weak() {
                    Err(ValidationError::VariableHasUndeclaredType(ident.as_ref().into()))
                } else {
                    Ok(())
                }
            },
            Statement::IfStmt(_) => Ok(()),
            Statement::WhileStmt { .. } => Ok(()),
            Statement::ForStmt { .. } => Ok(()),
            Statement::Block(_) => Ok(()),
            Statement::Return(_) => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IfKind {
    If(Id<Expression>, Id<BlockScope>),
    ElseIf(Id<Expression>, Id<BlockScope>),
    Else(Id<BlockScope>)
}

impl GetChildren for Statement {
    fn get_children(&self) -> Vec<WeakIndex> {
        match self {
            Statement::Expression(value) => vec![value.as_weak()],
            Statement::Assignment { on, field: _, expr } => match on {
                Some(on) => vec![on.as_weak(), expr.as_weak()],
                None => vec![expr.as_weak()],
            },
            Statement::IfStmt(ifs) => {
                let mut result = vec![];

                for if_kind in ifs {
                    match if_kind {
                        IfKind::If(v0, v1) => {
                            result.push(v0.as_weak());
                            result.push(v1.as_weak());
                        },
                        IfKind::ElseIf(v0, v1) => {
                            result.push(v0.as_weak());
                            result.push(v1.as_weak());
                        },
                        IfKind::Else(v0) => {
                            result.push(v0.as_weak());
                        },
                    }
                }
                result
            },
            Statement::WhileStmt { expr, block } => vec![expr.as_weak(), block.as_weak()],
            Statement::Block(value) => vec![value.as_weak()],
            Statement::Return(value) => vec![value.as_weak()],
            Statement::ForStmt { ident: _, enumerable, block } => vec![enumerable.as_weak(), block.as_weak()],
            Statement::LocalVariableDeclaration { expr, .. } => vec![expr.as_weak()],
        }
    }
}