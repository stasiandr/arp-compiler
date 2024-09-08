pub mod expression;
pub mod statement;
pub mod block_scope;
pub mod declaration;
pub mod utils;

use arp_parser::types::ChumskyNode;
use arp_types::Spanned;
use thiserror::Error;
use crate::{ast::{index::StrongIndex, traits::AstNodeKind}, types::{ast_node_value::{Ast, AstNodeValue, WId}, file::ArpFile, statement::Statement}};


#[derive(Debug, Error)]
pub enum ChumskyFoldError {

    #[error("Found {0:?} but expected {1}")]
    UnexpectedChumsky(Spanned<ChumskyNode>, Box<str>),

    #[error("Can't determine field name")]
    WrongConstructorShortInit(Spanned<ChumskyNode>),

    #[error("Can't unfold long type")]
    CantUnfoldTypeName(Vec<Spanned<ChumskyNode>>),

    #[error("This language feature is not yet implemented")]
    Unimplemented(Spanned<ChumskyNode>),
}

pub trait Folder<T, Res : AstNodeKind<AstNodeValue>> {
    type Error;

    fn fold(&mut self, item: &T, parent: WId) -> Result<StrongIndex<Res>, Self::Error>;
}


#[derive(Default)]
pub struct ChumskyNodeVisitor {
    pub(crate) ast: Ast
}

impl ChumskyNodeVisitor {
    pub fn fold_statement(mut self, node: &Spanned<ChumskyNode>) -> Ast {
        let root = self.ast.get_root_index();
        let _stmt: StrongIndex<Statement>  = self.fold(node, root).unwrap();

        self.ast
    }

    pub fn fold_file(&mut self, node: &Spanned<ChumskyNode>) -> &Ast {
        let root = self.ast.get_root_index();
        let _file: StrongIndex<ArpFile>  = self.fold(node, root).unwrap();

        &self.ast
    }

    #[inline]
    pub fn consume(self) -> Ast {
        self.ast
    }
    
    #[allow(unused)]
    pub(crate) fn fold_file_with_path(&mut self, node: &Spanned<ChumskyNode>, arp_path: String) -> &Ast {
        let root = self.ast.get_root_index();
        let file: StrongIndex<ArpFile>  = self.fold(node, root).unwrap();
        self.ast.mutate_value(&file, |file| {
            file.arp_path = arp_path.clone().into();
        });

        &self.ast
    }
}