use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::types::{ast_node_value::{Id, WId}, block_scope::{BlockScope, BlockScopeKind}};

use super::{ChumskyFoldError, ChumskyNodeVisitor, Folder};



impl Folder<Spanned<ChumskyNode>, BlockScope> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;
    
    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<Id<BlockScope>, Self::Error> {
        let next = self.ast.next_index(parent);

        let block = match item.get_value() {
            ChumskyNode::BlockStmt(declarations, return_expression ) => {
                let declarations = declarations.iter().map(|node| {
                    self.fold(node, next)
                }).collect::<Result<Vec<_>, _>>()?;

                let return_expression = match return_expression {
                    Some(some) => self.fold(some.as_ref(), next).ok(),
                    None => None,
                };

                let block_scope = BlockScope::new(BlockScopeKind::default(), declarations, return_expression);
                
                Ok(block_scope)
            },
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "block".into()))
        }?;

        Ok(self.ast.place_spanned(next, block, item.get_span()))
    }
}