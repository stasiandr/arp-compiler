use super::{index::{StrongIndex, WeakIndex}, traits::{AstNodeKind, AstNodeUnion}, AbstractAst, AbstractNode};



impl<U : AstNodeUnion> AbstractAst<U> {

    pub fn get_child_of_kind<T : AstNodeKind<U>, I : Into<WeakIndex> + Clone>(&self, index: I) -> Option<StrongIndex<T>> {
        for node in self.rec_iter_start_from(index).flat_map(|(index, _)| self.get_weak(index)) {
            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());

            if result.is_ok() { return Some(node.self_index.promote()) }
        }

        None
    }

    pub fn get_children_of_kind<T : AstNodeKind<U>, I : Into<WeakIndex> + Clone>(&self, index: I) -> Vec<StrongIndex<T>> 
        where U : TryInto<T> { 
        let mut indices = Vec::new();
        for node in self.rec_iter_start_from(index).flat_map(|(index, _)| self.get_weak(index)) {            
            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());
    
            if result.is_ok() { indices.push(node.self_index.promote()); }
        }

        indices
    }

    pub fn find_children_of_kind<T : AstNodeKind<U>, I : Into<WeakIndex> + Clone>(&self, index: I, filter: fn(&T) -> bool) -> Vec<StrongIndex<T>>
        where U : TryInto<T> { 
        let mut indices = Vec::new();
        
        for node in self.rec_iter_start_from(index).flat_map(|(index, _)| self.get_weak(index)) {
            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());
            
            match result {
                Ok(ok) => {
                    if !filter(&ok) {
                        continue;
                    }

                    indices.push(node.self_index.promote());
                },
                Err(_) => continue,
            }
        }

        indices
    }


    pub fn get_parent_of_kind<T : AstNodeKind<U>, I : Into<WeakIndex> + Clone>(&self, index: I) -> Option<StrongIndex<T>> {
        if let Some(node) = self.get_weak(index.clone()) {
            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());

            if node.parent_index == node.self_index {
                return None;
            }

            match result {
                Ok(_) => return Some(node.self_index.promote()),
                Err(_) => return self.get_parent_of_kind(node.parent_index),
            }
        }

        None
    }

    pub fn get_parent_of_kind_with_filter<T : AstNodeKind<U>, I : Into<WeakIndex> + Clone>(&self, index: I, filter: fn(&AbstractNode<U>) -> bool) -> Option<StrongIndex<T>> {
        if let Some(node) = self.get_weak(index.clone()) {
            if !filter(node) {
                return self.get_parent_of_kind_with_filter(node.parent_index, filter);
            }

            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());

            match result {
                Ok(_) => return Some(node.self_index.promote()),
                Err(_) => return self.get_parent_of_kind_with_filter(node.parent_index, filter),
            }
        }

        None
    }

    pub fn get_nodes_of_kind<T : AstNodeKind<U>>(&self) -> Vec<StrongIndex<T>>  
        where U : TryInto<T> { 
        let mut indices = vec![];
        
        for node in self.sequential_iter().flat_map(|index| self.get_weak(index)) {
            let result: Result<T, _> = TryInto::<T>::try_into(node.value.clone());

            if result.is_ok() { indices.push(node.self_index.promote()); }
        }

        indices
    }

}