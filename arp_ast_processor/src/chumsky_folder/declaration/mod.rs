pub mod function;
pub mod structure;
pub mod implementation;
pub mod arp_file;

use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::types::{ast_node_value::{Id, WId}, declaration::Declaration};
use super::{ChumskyFoldError, ChumskyNodeVisitor, Folder};

impl Folder<Spanned<ChumskyNode>, Declaration> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;
    
    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<Id<Declaration>, Self::Error> {
        let next = self.ast.next_index(parent);

        let decl = match item.get_value() {
            ChumskyNode::File(..) => Ok(Declaration::File(self.fold(item, next)?)),
            ChumskyNode::ImplementationDecl(..) => Ok(Declaration::Implementation(self.fold(item, next)?)),
            ChumskyNode::Structure(..) => Ok(Declaration::Structure(self.fold(item, next)?)),
            ChumskyNode::FuncDecl(..) => {
                Ok(Declaration::Function(self.fold(item, next)?))
            }

            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "declaration".into()))
        };

        Ok(self.ast.place_spanned(next, decl?, item.get_span()))
    }

}