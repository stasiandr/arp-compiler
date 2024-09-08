use arp_parser::types::ChumskyNode;
use arp_types::Spanned;
use crate::{ast::index::StrongIndex, types::{ast_node_value::WId, expression::{Expression, Literal}, implementation::Implementation, simple::Identifier, type_collection::TypeId}};

use super::{utils::parse_ident, ChumskyFoldError, ChumskyNodeVisitor, Folder};


impl Folder<Spanned<ChumskyNode>, Expression> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;

    fn fold(&mut self, item: &Spanned<arp_parser::types::ChumskyNode>, parent: WId) -> Result<StrongIndex<Expression>, ChumskyFoldError> {
        match item.get_value() {
            ChumskyNode::LiteralInteger(i) => Ok(self.ast.push_spanned(Expression::Literal(Literal::Integer(*i)), item.get_span(), parent)),
            ChumskyNode::LiteralFloat(f) => Ok(self.ast.push_spanned(Expression::Literal(Literal::Float(f.0)), item.get_span(), parent)),
            ChumskyNode::LiteralString(s) => Ok(self.ast.push_spanned(Expression::Literal(Literal::String(s.clone())), item.get_span(), parent)),
            ChumskyNode::LiteralBool(b) => Ok(self.ast.push_spanned(Expression::Literal(Literal::Bool(*b)), item.get_span(), parent)),

            ChumskyNode::BinaryExpr(lhs, op, rhs) => {

                let next = self.ast.next_index(parent);
                let lhs = self.fold(lhs.as_ref(), next)?;
                let rhs = self.fold(rhs.as_ref(), next)?;

                let bin_expr = Expression::Binary { 
                    lhs, 
                    op: (*op).into(), 
                    rhs
                };

                Ok(self.ast.place_spanned(next, bin_expr, item.get_span()))
            },
            ChumskyNode::UnaryExpr(op, expr) => {

                let next = self.ast.next_index(parent);
                let expr = self.fold(expr.as_ref(), next)?;

                let bin_expr = Expression::Unary { 
                    op: (*op).into(), 
                    expr,
                };

                Ok(self.ast.place_spanned(next, bin_expr, item.get_span()))
            },
            ChumskyNode::Identifier(ident) => {
                Ok(self.ast.push(Expression::Variable(ident.clone().into()), parent))
            },

            ChumskyNode::GetExpr(lhs, rhs) => {
                let next = self.ast.next_index(parent);
                let on = self.fold(lhs.as_ref(), next)?;

                match rhs.get_value() {
                    ChumskyNode::CallExpr(ident, args) => {
                        let args = args.iter().map(|item| self.fold(item, next)).collect::<Result<Vec<_>, _>>()?;
                        let method = match ident.get_value() {
                            ChumskyNode::Identifier(ident) => Ok(Identifier::from(ident.as_ref())),
                            _ => Err(ChumskyFoldError::UnexpectedChumsky(*rhs.clone(), "call or identifier".into())),
                        }?;

                        Ok(self.ast.place_spanned(next, Expression::Call { on: Some(on), method, args }, item.get_span()))
                    },
                    ChumskyNode::Identifier(ident) => Ok(self.ast.place_spanned(next, Expression::GetField { on, ident: ident.clone().into() }, item.get_span())),
                    _ => Err(ChumskyFoldError::UnexpectedChumsky(*rhs.clone(), "call or identifier".into()))
                }
            },

            ChumskyNode::ConstructExpr(ident, args) => {
                let next = self.ast.next_index(parent);

                let args = args.iter().map(|(ident, expr)| {
                    let ident = match ident {
                        Some(id) => parse_ident(id),
                        None => parse_ident(expr),
                    };

                    match self.fold(expr, next) {
                        Ok(expr_id) => Ok((ident?, expr_id)),
                        Err(err) => Err(err),
                    }
                }).collect::<Result<Vec<_>, _>>()?;

                
                let ident = match ident.get_value() {
                    ChumskyNode::Type(long_type) => {
                        let type_name = long_type.iter().map(|node| match node.get_value() {
                            ChumskyNode::Identifier(s) => Ok(s.clone()),
                            _ => Err(ChumskyFoldError::CantUnfoldTypeName(long_type.clone())),
                        }).collect::<Result<Vec<_>, _>>()?.join(".");
                        
                        Ok(type_name)
                    },
                    _ => Err(ChumskyFoldError::UnexpectedChumsky(*ident.clone(), "type".into()))
                };
                
                Ok(self.ast.place_spanned(next, Expression::Construct { ident: ident?.into(), args }, item.get_span()))
            },
            ChumskyNode::CallExpr(_, _) => {
                unimplemented!("Call expression without target is not yet supported");
            }
            ChumskyNode::This => {
                if let Some(im) = self.ast.get_parent_of_kind::<Implementation, _>(parent) {
                    Ok(self.ast.push_spanned(Expression::This(self.ast.get(&im).impl_type.clone()), item.get_span(), parent))
                } else {
                    Ok(self.ast.push_spanned(Expression::This(TypeId::None), item.get_span(), parent))
                }
            }
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "expression".into()))
        }
    }
}