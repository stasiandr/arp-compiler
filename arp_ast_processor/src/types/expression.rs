use arp_parser::types::{BinaryOp, UnaryOp};
use crate::{ast::traits::GetChildren, type_resolver::TypeResolverError, types::{block_scope::BlockScope, file::ArpFile, statement::Statement}, validations::{Validate, ValidationError}};
use super::{ast_node_value::{Ast, Id, WId}, function::Function, simple::Identifier, type_collection::TypeId};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    This(TypeId),
    Variable(Identifier),
    Type(TypeId),
    Unary {
        op: UnaryOperator,
        expr: Id<Expression>,
    },
    Binary {
        lhs: Id<Expression>,
        op: BinaryOperator,
        rhs: Id<Expression>,
    },
    GetField {
        on: Id<Expression>,
        ident: Identifier,
    },
    Call {
        on: Option<Id<Expression>>,
        method: Identifier,
        args: Vec<Id<Expression>>,
    },
    Construct {
        ident: Identifier,
        args: Vec<(Identifier, Id<Expression>)>,
    }
}

impl GetChildren for Expression {
    fn get_children(&self) -> Vec<WId> {
        match self {
            Expression::Literal(_) |
            Expression::Variable(_) |
            Expression::This(_) |
            Expression::Type(_) => vec![],

            Expression::Unary { op: _, expr } => vec![expr.as_weak()],
            Expression::Binary { lhs, op: _, rhs } => vec![lhs.as_weak(), rhs.as_weak()],
            Expression::GetField { on, ident: _ } => vec![on.as_weak()],
            Expression::Call { on, method: _, args } => {
                let mut result = args.iter().map(|i| i.as_weak()).collect::<Vec<_>>();
                if let Some(on) = on {
                    result.push(on.as_weak());
                }

                result
            },
            Expression::Construct { args, .. } => {
                args.iter().map(|i| i.1.as_weak()).collect::<Vec<_>>()
            },
            
        }
    }
}

impl Ast {
    pub fn get_type(&self, index: &Id<Expression>) -> Result<TypeId, TypeResolverError> {
        if let Some(arp_file) = self.get_parent_of_kind::<ArpFile, _>(*index).map(|id| self.get(&id)) {
            match self.get(index) {
                Expression::Literal(lit) => match lit {
                    Literal::Integer(_) => Ok(arp_file.type_collection.get_int()),
                    Literal::Float(_) => Ok(arp_file.type_collection.get_float()),
                    Literal::String(_) => Ok(arp_file.type_collection.get_string()),
                    Literal::Bool(_) => Ok(arp_file.type_collection.get_bool()),
                }
                Expression::Unary { op, expr } => {
                    if let TypeId::Strong(ty) = self.get_type(expr)? {
                        match op {
                            UnaryOperator::Negate => {
                                let int = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.get_int();
                                let float = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.get_float();

                                if &ty == int.try_into_strong().unwrap() || &ty == float.try_into_strong().unwrap() {
                                    Ok(TypeId::Strong(ty))
                                } else {
                                    // TODO After IAdd and ISub remove this error.
                                    Err(TypeResolverError::UnexpectedType { expected: int.clone(), actual: ty.into() })
                                }
                            },
                            UnaryOperator::Not => {
                                let bool = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.get_bool();
                                if &ty == bool.try_into_strong().unwrap() {
                                    Ok(TypeId::Strong(ty))
                                } else {
                                    Err(TypeResolverError::UnexpectedType { expected: bool.clone(), actual: ty.into() })
                                }
                            },
                        }
                    } else {
                        Ok(TypeId::None)
                    }
                }
                Expression::Binary { lhs, op, rhs } => {
                    if let TypeId::Strong(lty) = self.get_type(lhs)? {
                        if let TypeId::Strong(rty) = self.get_type(rhs)? {
                            if lty != rty {
                                Err(TypeResolverError::BinaryMismatchedTypes(lty, rty))
                            } else {
                                match op {
                                    BinaryOperator::Or |
                                    BinaryOperator::And |
                                    BinaryOperator::Equal |
                                    BinaryOperator::NotEqual |
                                    BinaryOperator::Greater |
                                    BinaryOperator::GreaterOrEqual |
                                    BinaryOperator::Less |
                                    BinaryOperator::LessOrEqual => {
                                        let bool = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.get_bool();

                                        Ok(bool)
                                        // if &lty == bool.try_into_strong().unwrap() {
                                        //     Ok(bool)
                                        // } else {
                                        //     Err(TypeResolverError::UnexpectedType { expected: bool.clone(), actual: lty.into() })
                                        // }
                                    },
        
                                    BinaryOperator::Add |
                                    BinaryOperator::Subtract |
                                    BinaryOperator::Multiply |
                                    BinaryOperator::Divide => {
                                        let int = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.get_int();
                                        let float = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.get_float();

                                        if &lty == int.try_into_strong().unwrap() || &lty == float.try_into_strong().unwrap() {
                                            Ok(TypeId::Strong(lty))
                                        } else {
                                            // TODO After IAdd and ISub remove this error.
                                            Err(TypeResolverError::UnexpectedType { expected: int.clone(), actual: lty.into() })
                                        }
                                    },
                                }
                            }
                        } else {
                            Ok(TypeId::None)
                        }
                    } else {
                        Ok(TypeId::None)
                    }
                },
                Expression::Variable(var_ident) => {

                    let mut stmt = self.get_parent_of_kind::<Statement, _>(index.as_weak()).expect("variable must be in statement");

                    while let Some(bs) = self.get_parent_of_kind::<BlockScope, _>(stmt.as_weak()) {
                        let mut index_of_statement = self.get(&bs).statements.iter().position(|id| *id == stmt).expect("variable must be in statement");

                        loop {
                            if let Some(Statement::LocalVariableDeclaration { ident,  ty, .. }) = self.get(&bs).statements.get(index_of_statement).map(|id| self.get(id)) {
                                if var_ident == ident {
                                    return Ok(ty.clone());
                                }
                            }
    
                            if let Some(new_index_of_statement) = index_of_statement.checked_sub(1) {
                                index_of_statement = new_index_of_statement;
                            } else {
                                break;
                            }
                        }

                        if let Some(block_stmt) = self.try_promote::<Statement>(self.get_node(&bs).get_parent()).as_ref() {
                            match self.get(block_stmt) {
                                Statement::Block(_) => stmt = *block_stmt,
                                _ => break,
                            };
                        } else {
                            break;
                        }
                    }

                    if let Some(func) = self.get_parent_of_kind::<Function, _>(stmt.as_weak()).map(|id| self.get(&id)) {
                        if let Some((_, ty)) = func.parameters.iter().find(|(ident, _)| ident == var_ident) {
                            return Ok(ty.clone());
                        }
                    }

                    Ok(TypeId::None)
                },
                Expression::Construct { ident, .. } => {
                    Ok(self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.resolve_name(ident))
                },
                Expression::This(ty) => {
                    Ok(ty.clone())
                }
                Expression::GetField { on, ident } => {
                    let ty = self.get_type(on)?;
                    if let Some(ty) = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.try_get_strong(&ty) {
                        Ok(ty.fields.iter().find(|(i, _)| i == ident).map(|(_, ty)| ty.clone()).unwrap_or_default())
                    } else {
                        Ok(TypeId::None)
                    }
                },
                Expression::Call { on, method, args } => {
                    let on = on.expect("calls without targets not supported");
                    let on_type = self.get_type(&on)?;

                    let return_type = if let Some(ty) = self.get_arp_file_in_parent(index.as_weak()).unwrap().type_collection.try_get_strong(&on_type) {
                        ty.find_method(method, args.iter().flat_map(|arg| self.get_type(arg)).collect())
                    } else {
                        None
                    };

                    Ok(return_type.map(|mi| mi.return_type.clone()).unwrap_or_default())
                }
                Expression::Type(ty) => Ok(ty.clone()),
            }
        } else {
            Err(TypeResolverError::ArpFileNotFound)
        }
    }
}


impl Validate for Expression {
    fn validate(&self, index: Id<Expression>, ast: &Ast) -> Result<(), ValidationError> {
        ast.get_type(&index)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(Box<str>),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Negate,
    Not
}


impl From<UnaryOp> for UnaryOperator {
    fn from(value: UnaryOp) -> Self {
        match value {
            UnaryOp::Negate => UnaryOperator::Negate,
            UnaryOp::Not => UnaryOperator::Not,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Or,

    And,

    Equal,
    NotEqual,

    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,

    Add,
    Subtract,
    
    Multiply,
    Divide
}

impl From<BinaryOp> for BinaryOperator {
    fn from(value: BinaryOp) -> Self {
        match value {
            BinaryOp::Add => Self::Add,
            BinaryOp::Subtract => Self::Subtract,
            BinaryOp::Multiply => Self::Multiply,
            BinaryOp::Divide => Self::Divide,
            BinaryOp::Or => Self::Or,
            BinaryOp::And => Self::And,
            BinaryOp::Equals => Self::Equal,
            BinaryOp::NotEquals => Self::NotEqual,
            BinaryOp::Greater => Self::Greater,
            BinaryOp::GreaterOrEquals => Self::GreaterOrEqual,
            BinaryOp::Less => Self::Less,
            BinaryOp::LessOrEquals => Self::LessOrEqual,
        }
    }
}
