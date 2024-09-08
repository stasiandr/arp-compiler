use arp_ast_processor::{
    types::{
        ast_node_value::{Ast, Id},
        expression::{BinaryOperator, Expression, Literal, UnaryOperator},
        type_collection::TypeSourceKind,
    },
    utils::VariableSource,
};

use crate::{emitter::Emitter, il_token::OpCode, traits::Visitor, EmitError};

impl Visitor<Expression> for Emitter {
    fn visit(&mut self, index: &Id<Expression>, ast: &Ast) -> Result<(), EmitError> {
        match ast.get(index) {
            Expression::Literal(lit) => match lit {
                Literal::Integer(i) => self.write(OpCode::LoadInt(*i)),
                Literal::Float(f) => self.write(OpCode::LoadFloat(*f)),
                Literal::String(s) => {
                    self.write(OpCode::LoadString(s.trim_matches('\"').to_string()))
                }
                Literal::Bool(b) => self.write(OpCode::LoadBool(*b)),
            },
            Expression::Binary { lhs, op, rhs } => {
                self.visit(lhs, ast)?;
                self.visit(rhs, ast)?;

                match op {
                    BinaryOperator::Or => self.write(OpCode::Or),
                    BinaryOperator::And => self.write(OpCode::And),
                    BinaryOperator::Equal => self.write(OpCode::Equal),
                    BinaryOperator::NotEqual => {
                        self.write(OpCode::Equal);
                        self.write(OpCode::LoadBool(false));
                        self.write(OpCode::Equal);
                    }
                    BinaryOperator::Greater => self.write(OpCode::GreaterThen),
                    BinaryOperator::GreaterOrEqual => {
                        self.write(OpCode::LessThen);
                        self.write(OpCode::LoadBool(false));
                        self.write(OpCode::Equal);
                    }
                    BinaryOperator::Less => self.write(OpCode::LessThen),
                    BinaryOperator::LessOrEqual => {
                        self.write(OpCode::GreaterThen);
                        self.write(OpCode::LoadBool(false));
                        self.write(OpCode::Equal);
                    }
                    BinaryOperator::Add => self.write(OpCode::Add),
                    BinaryOperator::Subtract => self.write(OpCode::Subtract),
                    BinaryOperator::Multiply => self.write(OpCode::Multiply),
                    BinaryOperator::Divide => self.write(OpCode::Divide),
                }
            }
            Expression::Call { on, method, args } => {
                if let Some(on) = on {
                    match ast.get(on) {
                        Expression::Type(ty) => {
                            for arg in args {
                                self.visit(arg, ast)?;
                            }

                            let arp_file = ast
                                .get_arp_file_in_parent(index.as_weak())
                                .ok_or(EmitError::ArpFileNotFound)?;

                            let type_info = arp_file
                                .type_collection
                                .try_get_strong(ty)
                                .ok_or(EmitError::CantResolveType)?;

                            let args = args
                                .iter()
                                .map(|index| ast.get_type(index))
                                .collect::<Result<Vec<_>, _>>()
                                .map_err(|_| EmitError::CantResolveType)?;

                            let method_info = type_info
                                .find_method(method, args)
                                .ok_or(EmitError::CantResolveMethod)?;

                            let opcode = OpCode::Call {
                                is_instance: false,
                                return_type: ast
                                    .resolve_type(method_info.return_type.clone(), &index.as_weak())
                                    .ok_or(EmitError::CantResolveType)
                                    .map(|ty| ty.into())?,
                                external: match &type_info.source {
                                    TypeSourceKind::LocalArp
                                    | TypeSourceKind::ExternalArp(_)
                                    | TypeSourceKind::Standard => None,
                                    TypeSourceKind::ManagedDll(dll) => Some(
                                        dll.clone()
                                            .strip_suffix(".dll")
                                            .unwrap_or_default()
                                            .to_string(),
                                    ),
                                },
                                ty: type_info.clone().full_name.into(),
                                method_name: method_info.name.0.to_string(),
                                args: method_info
                                    .args
                                    .iter()
                                    .map(|(_, ty)| {
                                        ast.resolve_type(ty.clone(), &index.as_weak())
                                            .ok_or(EmitError::CantResolveType)
                                            .map(|ty| ty.into())
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            };

                            self.write(opcode);
                        }
                        _ => {
                            self.visit(on, ast)?;

                            for arg in args {
                                self.visit(arg, ast)?;
                            }

                            let ty = ast.get_type(on).map_err(|_| EmitError::CantResolveType)?;

                            let arp_file = ast
                                .get_arp_file_in_parent(index.as_weak())
                                .ok_or(EmitError::ArpFileNotFound)?;

                            let type_info = arp_file
                                .type_collection
                                .try_get_strong(&ty)
                                .ok_or(EmitError::CantResolveType)?;

                            

                            let args = args
                                .iter()
                                .map(|index| ast.get_type(index))
                                .collect::<Result<Vec<_>, _>>()
                                .map_err(|_| EmitError::CantResolveType)?;

                            let method_info = type_info
                                .find_method(method, args)
                                .ok_or(EmitError::CantResolveMethod)?;

                            let opcode = OpCode::Call {
                                is_instance: true,
                                return_type: ast
                                    .resolve_type(method_info.return_type.clone(), &index.as_weak())
                                    .ok_or(EmitError::CantResolveType)
                                    .map(|ty| ty.into())?,
                                external: match &type_info.source {
                                    TypeSourceKind::LocalArp
                                    | TypeSourceKind::ExternalArp(_)
                                    | TypeSourceKind::Standard => None,
                                    TypeSourceKind::ManagedDll(dll) => Some(
                                        dll.clone()
                                            .strip_suffix(".dll")
                                            .unwrap_or_default()
                                            .to_string(),
                                    ),
                                },
                                ty: type_info.clone().full_name.into(),
                                method_name: method_info.name.0.to_string(),
                                args: method_info
                                    .args
                                    .iter()
                                    .map(|(_, ty)| {
                                        ast.resolve_type(ty.clone(), &index.as_weak())
                                            .ok_or(EmitError::CantResolveType)
                                            .map(|ty| ty.into())
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            };

                            self.write(opcode);
                        }
                    }
                } else {
                    unreachable!("Don't know why this happened");
                }
            }
            Expression::Unary { op, expr } => {
                self.visit(expr, ast)?;
                match op {
                    UnaryOperator::Negate => {
                        self.write(OpCode::LoadInt(-1));
                        self.write(OpCode::Multiply);
                    }
                    UnaryOperator::Not => {
                        self.write(OpCode::LoadBool(false));
                        self.write(OpCode::Equal)
                    }
                };
            }
            Expression::Variable(ident) => {
                let source = ast
                    .get_identifier_source(ident, &index.as_weak())
                    .ok_or(EmitError::VariableHasNoSource)?;

                match source {
                    VariableSource::Local(loc) => self.write(OpCode::LoadLocalVariable(loc)),
                    VariableSource::Argument(loc) => self.write(OpCode::LoadArgument(loc)),
                }
            }
            Expression::Construct { ident, args } => {
                let file = ast
                    .get_arp_file_in_parent(index.as_weak())
                    .ok_or(EmitError::ArpFileNotFound)?;

                let ty = file.type_collection.resolve_name(ident);
                let ty = file
                    .type_collection
                    .try_get_strong(&ty)
                    .ok_or(EmitError::CantResolveType)?;

                let mut resolved_types = vec![];
                for fld in &ty.fields {
                    let arg = args
                        .iter()
                        .find(|(ident, _)| ident == &fld.0)
                        .ok_or(EmitError::CantResolveField)?;
                    self.visit(&arg.1, ast)?;

                    let fld_ty = Emitter::resolve_ty(ast, &fld.1, *index)?;
                    resolved_types.push(fld_ty);
                }

                self.write(OpCode::NewObject(ty.into(), resolved_types))
            }

            Expression::GetField { on, ident } => {
                self.visit(on, ast)?;

                let file = ast
                    .get_arp_file_in_parent(index.as_weak())
                    .ok_or(EmitError::ArpFileNotFound)?;
                let ty = ast.get_type(on).map_err(|_| EmitError::CantResolveType)?;
                let on_ty = file
                    .type_collection
                    .try_get_strong(&ty)
                    .ok_or(EmitError::CantResolveType)?;
                let (_, fld_ty_id) = on_ty
                    .fields
                    .iter()
                    .find(|fld| fld.0 == *ident)
                    .ok_or(EmitError::CantResolveField)?;

                let fld_ty = Emitter::resolve_ty(ast, fld_ty_id, *index)?;

                self.write(OpCode::GetField(
                    fld_ty,
                    on_ty.full_name.to_string(),
                    ident.0.to_string(),
                ))
            }

            Expression::This(_) => self.write(OpCode::LoadArgument(0)),

            val => unreachable!("Reached {:?}", val),
            // Expression::Type(_) => todo!(),
        }

        Ok(())
    }
}
