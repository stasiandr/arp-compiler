use arp_types::span::Span;

use super::{index::{StrongIndex, WeakIndex}, traits::{AstNodeKind, AstNodeUnion}, AbstractAst, AbstractNode};



impl<U : AstNodeUnion> AbstractAst<U> {

    // --- Root --- 

    pub fn add_node_to_root(&mut self, index: WeakIndex) {
        self.nodes[self.root_index.index].value.push_child(index);
    }

    pub fn remove_node_from_root(&mut self, index: WeakIndex) {
        self.nodes[self.root_index.index].value.remove_child(index);
    }

    // --- Add ---

    pub fn push<T : Into<U>, W : Into<WeakIndex> + Clone>(&mut self, value: T, parent: W ) -> StrongIndex<T> {
        let index = self.nodes.len();

        let parent_index = parent.into();
        let self_index = WeakIndex::new(index);

        if parent_index == self.root_index {
            self.add_node_to_root(self_index);
        }

        self.nodes.push(AbstractNode { value: value.into(), parent_index, self_index, no_emit: false, span: Span::default() });

        StrongIndex::new(index)
    }

    pub fn push_spanned<T : Into<U>, W : Into<WeakIndex> + Clone, S: Into<Span>>(&mut self, value: T, span: S, parent: W ) -> StrongIndex<T> {
        let index = self.nodes.len();

        let parent_index = parent.into();
        let self_index = WeakIndex::new(index);

        if parent_index == self.root_index {
            self.add_node_to_root(self_index);
        }

        self.nodes.push(AbstractNode { value: value.into(), parent_index, self_index, no_emit: false, span: span.into() });

        StrongIndex::new(index)
    }

    pub fn next_index<W : Into<WeakIndex> + Clone>(&mut self, parent_index: W) -> WeakIndex {
        let index = self.nodes.len();

        let parent_index = parent_index.into();
        let self_index = WeakIndex::new(index);

        if parent_index == self.root_index {
            self.add_node_to_root(self_index);
        }

        self.nodes.push(AbstractNode { value: U::default(), parent_index, self_index, no_emit: false, span: Span::default() });

        self_index
    }

    pub fn place<T : Into<U>>(&mut self, index: WeakIndex, value: T) -> StrongIndex<T> {
        self.nodes[index.index].value = value.into();
        index.promote()
    }

    pub fn place_spanned<T : Into<U>, S: Into<Span>>(&mut self, index: WeakIndex, value: T, span: S) -> StrongIndex<T> {
        self.nodes[index.index].value = value.into();
        self.nodes[index.index].span = span.into();
        index.promote()
    }

    // --- Get ---

    pub fn get<T>(&self, index: &StrongIndex<T>) -> &T
        where U : AsRef<T>, 
    {
        let value = &self.nodes[index.index].value;
        value.as_ref()
    }

    pub fn get_node<T>(&self, index: &StrongIndex<T>) -> &AbstractNode<U>
        where U : AsRef<T>, 
    {
        &self.nodes[index.index]
    }

    pub fn get_mut<T>(&mut self, index: &StrongIndex<T>) -> &mut T
        where U : AsMut<T>, 
    {
        let value = &mut self.nodes[index.index].value;
        value.as_mut()
    }

    pub fn get_node_mut<T>(&mut self, index: &StrongIndex<T>) -> &mut AbstractNode<U>
        where U : AsMut<T>, 
    {
        &mut self.nodes[index.index]
    }

    pub fn get_weak<T : Into<WeakIndex> + Clone>(&self, index: T) -> Option<&AbstractNode<U>> {
        self.nodes.get(index.into().index)
    }

    pub fn get_weak_mut<T : Into<WeakIndex> + Clone>(&mut self, index: T) -> Option<&mut AbstractNode<U>> {
        if index.clone().into() == self.root_index {
            None
        } else {
            self.nodes.get_mut(index.into().index)
        }
    }


    // --- Mut ---

    pub fn mutate_value<T : AstNodeKind<U>>(&mut self, index: &StrongIndex<T>, mutator: impl Fn(&mut T))
        where U : AsRef<T> + AsMut<T> {
        let node = self.get_mut(index);
        mutator(node);
    }

    pub fn set_value<T : AstNodeKind<U>>(&mut self, index: &StrongIndex<T>, mutator: impl Fn() -> T)
        where U : AsRef<T> + AsMut<T> {
        let node = self.get_node_mut(index);
        node.value = mutator().into();
    }

    pub fn mutate_node<T : AstNodeKind<U>>(&mut self, index: &StrongIndex<T>, mutator: impl Fn(&mut AbstractNode<U>))
        where U : AsRef<T> + AsMut<T> {
        let node = self.get_node_mut(index);
        mutator(node);
    }


    // --- Index ---

    pub fn try_promote<T : AstNodeKind<U>>(&self, wid: WeakIndex) -> Option<StrongIndex<T>> {
        if let Some(node) = self.get_weak(wid) {
            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());
            
            if result.is_ok() {
                Some(wid.promote())
            } else {
                None
            }
        } else {
            None
        }
    }

}