use std::fmt::Debug;

use super::WeakIndex;

pub trait AstNodeUnion: GetChildren + Clone + Debug + Default + PartialEq + PushRemoveRootChildren {}
pub trait AstNodeKind<U : AstNodeUnion>: TryFrom<U> + Into<U> + GetChildren + Debug + PartialEq { }

pub trait GetChildren {
    fn get_children(&self) -> Vec<WeakIndex>;
}

pub trait PushRemoveRootChildren {
    fn push_child(&mut self, index: WeakIndex);
    fn remove_child(&mut self, index: WeakIndex);
}

