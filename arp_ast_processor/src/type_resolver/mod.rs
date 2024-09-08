pub mod imports_graph;
pub mod managed_dll_info;

use crate::types::{ast_node_value::{Ast, Id}, block_scope::BlockScope, expression::Expression, function::Function, statement::Statement, structure::Structure, type_collection::{StrongTypeId, TypeId}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeResolverError {

    #[error("Binary expression's variables has different types")]
    BinaryMismatchedTypes(StrongTypeId, StrongTypeId),

    #[error("Unexpected type")]
    UnexpectedType {
        expected: TypeId,
        actual: TypeId,
    },

    #[error("Unexpected type")]
    FunctionTypeMustBeDeclared(Id<Function>),

    #[error("Unexpected type")]
    FieldTypeMustBeDeclared(Id<Structure>),

    #[error("Type don't have field")]
    TypeFieldNotFound(),

    #[error("Path not found")]
    PathNotFound(String),

    #[error("Recursive import detected")]
    RecursiveImportDetected(String),


    #[error("TypeNotFoundInFilePath")]
    TypeNotFoundInFilePath(String),

    #[error("Parent Arp file not found")]
    ArpFileNotFound,

    #[error("Infinite type loop. No idea why")]
    InfiniteTypeLoopDetected,
}

pub enum TypeMutation {
    ImplicitVariableDeclaration(Id<BlockScope>, Id<Statement>, TypeId),
    ConvertVariableToTypeExpression(Id<Expression>, TypeId),
}

impl ExecuteMutation for TypeMutation {
    fn execute(&self, mut ast: Ast) -> Result<Ast, TypeResolverError> {
        match self {
            TypeMutation::ImplicitVariableDeclaration(_block, stmt, new_ty) => {
                ast.mutate_value(stmt, |value| {
                    if let Statement::LocalVariableDeclaration { ty, .. } = value {
                        *ty = new_ty.clone();
                    }
                });
            },
            TypeMutation::ConvertVariableToTypeExpression(expr, ty) => {
                ast.set_value(expr, || {
                    Expression::Type(ty.clone())
                })
            },
        }

        Ok(ast)
    }
}

pub trait ExecuteMutation {
    fn execute(&self, ast: Ast) -> Result<Ast, TypeResolverError>;
}

pub fn resolve_types_loop(mut ast: Ast) -> Result<Ast, TypeResolverError> {
    let mut i = 0;
    while let Some(mu) = resolve_types(&ast)? {
        ast = mu.execute(ast)?;

        i += 1;
        if i == 100000 {
            return Err(TypeResolverError::InfiniteTypeLoopDetected);
        }
    }

    Ok(ast)
}

pub fn resolve_types(ast: &Ast) -> Result<Option<TypeMutation>, TypeResolverError> {
    for variable in ast.get_children_of_kind(ast.get_root_index()) {
        if let Some(m)  = convert_variables_to_known_types(ast, &variable)? {
            return Ok(Some(m));
        }
    }

    for bs in ast.get_children_of_kind(ast.get_root_index()) {
        if let Some(m)  = resolve_types_in_local_block(ast, &bs)? {
            return Ok(Some(m));
        }
    }

    Ok(None)
}

pub fn convert_variables_to_known_types(ast: &Ast, expr: &Id<Expression>) -> Result<Option<TypeMutation>, TypeResolverError> {
    match ast.get(expr) {
        Expression::Variable(ident) => {
            let ty = ast.get_arp_file_in_parent(expr.as_weak())
                .unwrap()
                .type_collection
                .resolve_name(ident);

            match ty {
                TypeId::Strong(_) => {
                    Ok(Some(TypeMutation::ConvertVariableToTypeExpression(*expr, ty)))
                },
                _ => Ok(None)
            }
        },
        Expression::GetField { .. } => {
            let collected_type = try_collect_get_chain(ast, expr, "".to_string())?;

            if let Some(collected_type) = collected_type {
                let transformed = &mut collected_type[1..].to_string();

                let ty = ast.get_arp_file_in_parent(expr.as_weak())
                    .unwrap()
                    .type_collection
                    .resolve_name(&transformed);

                match ty {
                    TypeId::Strong(_) => {
                        Ok(Some(TypeMutation::ConvertVariableToTypeExpression(*expr, ty)))
                    },
                    _ => Ok(None)
                }
            } else {
                Ok(None)
            }
        },
        _ => Ok(None)
    }
}

fn try_collect_get_chain(ast: &Ast, expr: &Id<Expression>, sum_str: String) -> Result<Option<String>, TypeResolverError> {
    match ast.get(expr) {
        // Expression::Type(_) => todo!(),
        Expression::Variable(ident) => {
            Ok(Some(format!("{}.{}", sum_str, ident.as_ref()).to_string()))
        },
        Expression::GetField { on, ident } => {
            let collected = try_collect_get_chain(ast, on, sum_str.clone())?;
            if let Some(collected) = collected {
                Ok(Some(format!("{}.{}", collected, ident.as_ref()).to_string()))
            } else {
                Ok(None)
            }  
        },
        _ => Ok(None)
    }
}

pub fn resolve_types_in_local_block(ast: &Ast, block: &Id<BlockScope>) -> Result<Option<TypeMutation>, TypeResolverError> {
    for stmt in ast.get(block).statements.iter() {
        if let Statement::LocalVariableDeclaration { ty: TypeId::None | TypeId::Weak(_), expr, .. } = ast.get(stmt) {
            match ast.get_type(expr) {
                Ok(TypeId::Strong(ty)) => {
                    return Ok(Some(TypeMutation::ImplicitVariableDeclaration(*block, *stmt, TypeId::Strong(ty))));
                },
                Err(err) => return Err(err),
                _ => {}
            }
        }
    }

    Ok(None)
}