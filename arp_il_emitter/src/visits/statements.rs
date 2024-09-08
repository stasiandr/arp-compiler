use arp_ast_processor::{
    types::{
        ast_node_value::{Ast, Id},
        statement::{IfKind, Statement},
    },
    utils::VariableSource,
};

use crate::{emitter::Emitter, il_token::OpCode, traits::Visitor, EmitError};

impl Visitor<Statement> for Emitter {
    fn visit(&mut self, index: &Id<Statement>, ast: &Ast) -> Result<(), EmitError> {
        match ast.get(index) {
            Statement::Expression(expr) => self.visit(expr, ast)?,

            Statement::LocalVariableDeclaration { ident, expr, .. } => {
                self.visit(expr, ast)?;

                let source = ast
                    .get_identifier_source(ident, &index.as_weak())
                    .ok_or(EmitError::VariableHasNoSource)?;

                match source {
                    VariableSource::Local(register_index) => {
                        self.write(OpCode::StoreLocalVariable(register_index));
                    }
                    VariableSource::Argument(arg_index) => {
                        self.write(OpCode::StoreArgument(arg_index))
                    }
                }
            }
            Statement::Block(bs) => self.visit(bs, ast)?,

            Statement::Assignment { on, field, expr } => {
                if let Some(on) = on {
                    self.visit(on, ast)?;
                    self.visit(expr, ast)?;

                    let expr_ty = Emitter::resolve_ty(
                        ast,
                        &ast.get_type(expr).map_err(|_| EmitError::CantResolveType)?,
                        *index,
                    )?;
                    let ty = Emitter::resolve_ty(
                        ast,
                        &ast.get_type(on).map_err(|_| EmitError::CantResolveType)?,
                        *index,
                    )?;

                    self.write(OpCode::SetField(
                        expr_ty,
                        ty.0.to_string(),
                        field.0.to_string(),
                    ))
                } else {
                    self.visit(expr, ast)?;

                    let source = ast
                        .get_identifier_source(field, &index.as_weak())
                        .ok_or(EmitError::VariableHasNoSource)?;

                    match source {
                        VariableSource::Local(register_index) => {
                            self.write(OpCode::StoreLocalVariable(register_index));
                        }
                        VariableSource::Argument(arg_index) => {
                            self.write(OpCode::StoreArgument(arg_index))
                        }
                    }
                }
            }
            Statement::IfStmt(if_kinds) => {
                if if_kinds.len() > 2 {
                    return Err(EmitError::UnsupportedIfChain);
                }

                let expr = match if_kinds[0] {
                    IfKind::If(expr, _) => expr,
                    _ => return Err(EmitError::UnsupportedIfChain),
                };

                let block = match if_kinds[0] {
                    IfKind::If(_, block) => block,
                    _ => return Err(EmitError::UnsupportedIfChain),
                };

                let else_block = {
                    if if_kinds.len() > 1 {
                        match if_kinds[1] {
                            IfKind::Else(block) => Some(block),
                            _ => return Err(EmitError::UnsupportedIfChain),
                        }
                    } else {
                        None
                    }
                };

                self.visit(&expr, ast)?;
                let end_label = self.next_label();
                let else_block_label = self.next_label();

                self.write(OpCode::BranchIfFalse(else_block_label.clone()));
                self.visit(&block, ast)?;

                if else_block.is_some() {
                    self.write(OpCode::BranchTo(end_label.clone()));
                }

                self.write_labeled_opcode(OpCode::NoOperation, else_block_label);

                if let Some(else_block) = else_block {
                    self.visit(&else_block, ast)?;
                }

                self.write_labeled_opcode(OpCode::NoOperation, end_label);
            }


            Statement::Return(expr) => {
                self.visit(expr, ast)?;
                self.write(OpCode::Return);
            },

            Statement::WhileStmt { expr, block } => {

                let loop_start = self.next_label();
                let condition = self.next_label();

                self.write(OpCode::BranchTo(condition.clone()));
                self.write_labeled_opcode(OpCode::NoOperation, loop_start.clone());
                self.visit(block, ast)?;
                self.write_labeled_opcode(OpCode::NoOperation, condition);
                self.visit(expr, ast)?;
                self.write(OpCode::BranchIfTrue(loop_start))
            },
            
            // Statement::ForStmt { .. } => todo!(),
            val => unimplemented!("This is not currently implemented: {val:?}"),
        }

        Ok(())
    }
}
