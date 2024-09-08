use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::{ast::index::StrongIndex, types::{ast_node_value::WId, simple::Identifier, statement::{IfKind, Statement}, type_collection::TypeId}};

use super::{utils::{parse_ident, parse_type}, ChumskyFoldError, ChumskyNodeVisitor, Folder};

impl Folder<Spanned<ChumskyNode>, Statement> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;

    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<StrongIndex<Statement>, Self::Error> {

        if let ChumskyNode::StatementDecl(node) = item.get_value() {
            let statement: StrongIndex<Statement> = self.fold(node.as_ref(), parent)?;
            return Ok(statement);
        }

        let next = self.ast.next_index(parent);
        let stmt = match item.get_value() {
            ChumskyNode::ExpressionStmt(expr) => Ok(Statement::Expression(self.fold(expr.as_ref(), next)?)),
            ChumskyNode::ReturnStmt(expr) => Ok(Statement::Return(self.fold(expr.as_ref(), next)?)),

            ChumskyNode::AssignmentStmt(lhs, rhs) => {
                let expr = self.fold(rhs.as_ref(), next)?;

                match lhs.get_value() {
                    ChumskyNode::GetExpr(on, field) => {
                        match field.get_value() {
                            ChumskyNode::Identifier(ident) => {
                                let on = self.fold(on.as_ref(), next)?;
                                Ok(Statement::Assignment { on: Some(on), field: Identifier::from(ident.as_ref()), expr })
                            },
                            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "field getter".into()))
                        }
                    },
                    ChumskyNode::Identifier(ident) => Ok(Statement::Assignment { on: None, field: Identifier::from(ident.as_ref()), expr }),
                    _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "identifier or field getter".into()))
                }
            }

            ChumskyNode::WhileStmt(expr, block) => {
                let expr = self.fold(expr.as_ref(), next)?;
                let block = self.fold(block.as_ref(), next)?;

                Ok(Statement::WhileStmt { expr, block })
            }

            ChumskyNode::IfStmt(expr, block, if_else_blocks, else_block) => {
                let mut if_kinds = vec![];

                if_kinds.push(IfKind::If(self.fold(expr.as_ref(), next)?, self.fold(block.as_ref(), next)?));

                for (if_else_expr, if_else_block) in if_else_blocks {
                    if_kinds.push(IfKind::ElseIf(self.fold(if_else_expr, next)?, self.fold(if_else_block, next)?));
                }

                if let Some(else_block) = else_block {
                    if_kinds.push(IfKind::Else(self.fold(else_block.as_ref(), next)?));
                }

                Ok(Statement::IfStmt(if_kinds))
            },

            ChumskyNode::ForStmt(ident, enumerable, block) => {
                let ident = parse_ident(ident)?;

                Ok(Statement::ForStmt { ident, enumerable: self.fold(enumerable.as_ref(), next)?, block: self.fold(block.as_ref(), next)? })
            }

            ChumskyNode::VariableDecl(is_mutable, ident, declared_type, expr) => {
                let expr = self.fold(expr.as_ref(), next)?;

                let ident = parse_ident(ident)?;

                let ty = declared_type.as_ref().map(|ty| parse_type(ty, parent, &mut self.ast)).unwrap_or(Ok(TypeId::None))?;    
            
                Ok(Statement::LocalVariableDeclaration { is_mutable: *is_mutable, ident, ty, expr })
            },

            ChumskyNode::BlockStmt( .. ) => {
                let block = self.fold(item, next)?;
                Ok(Statement::Block(block))
            },
            
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "statement".into()))
        }?;

        Ok(self.ast.place_spanned(next, stmt, item.get_span()))
    }   
}