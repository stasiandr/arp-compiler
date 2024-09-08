use std::fmt::Debug;

use arp_types::span::Span;
use index::WeakIndex;
use traits::{AstNodeUnion, GetChildren};

pub mod add_get_mut;
pub mod errors;
pub mod index;
pub mod iterators;
pub mod traits;
pub mod traversing;
pub mod validators;

#[macro_use]
pub mod macros;

pub mod tests;

#[derive(Debug)]
pub struct AbstractAst<T: GetChildren> {
    nodes: Vec<AbstractNode<T>>,
    root_index: WeakIndex,
}

#[derive(Clone, Copy)]
pub struct AbstractNode<T : GetChildren> {
    value: T,
    parent_index: WeakIndex,
    self_index: WeakIndex,
    pub no_emit: bool,
    pub span: Span,
}

impl<U: AstNodeUnion> Default for AbstractAst<U> {
    fn default() -> Self {
        let zero_index = WeakIndex::new(0);
        AbstractAst {
            nodes: vec![AbstractNode {value:U::default(),parent_index:zero_index,self_index:zero_index,no_emit:false, span: Span::default() }],
            root_index: zero_index,
        }
    }
}

impl<U: AstNodeUnion> AbstractAst<U> {

    pub fn get_root_index(&self) -> WeakIndex {
        self.root_index
    }
    
}

impl<U: GetChildren> AbstractNode<U>  {

    #[inline]
    pub fn get_value(&self) -> &U {
        &self.value
    }

    #[inline]
    pub fn get_index(&self) -> WeakIndex {
        self.self_index
    }

    #[inline]
    pub fn get_parent(&self) -> WeakIndex {
        self.parent_index
    }


    pub fn with_parent<W : Into<WeakIndex> + Clone>(mut self, parent: W) -> Self {
        self.parent_index = parent.into();
        self
    }

    pub fn __unsafe_set_parent<W : Into<WeakIndex> + Clone>(&mut self, parent: W) {
        self.parent_index = parent.into();
    }
    
}


impl<T: GetChildren + Debug> Debug for AbstractNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("{}: [{}] | {:?}", self.self_index.index, self.parent_index.index, &self.value))
    }
}
