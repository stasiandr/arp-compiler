use thiserror::Error;

use super::WeakIndex;




#[derive(Error, Debug)]
pub enum AstError {

    #[error("Node({node:?}) is expected to have child({expected_child:?})")]
    ParentDoNotHaveChild{
        node: WeakIndex,
        expected_child: WeakIndex,
    },

    #[error("Expected node type is {expected}, but found {actual}")]
    WrongNodeType{
        expected: String,
        actual: String
    },

    #[error("Node({node:?}) has wrong parent. Right parent is {expected_parent:?}, but found {actual_parent:?}")]
    ChildNodeHasWrongParent{
        node: WeakIndex,
        actual_parent: WeakIndex,
        expected_parent: WeakIndex,
    }
}