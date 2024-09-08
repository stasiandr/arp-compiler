use super::{errors::AstError, traits::AstNodeUnion, AbstractAst};

impl<U: AstNodeUnion> AbstractAst<U> {

    pub fn validate(&self) -> Result<(), AstError> {
        self.validate_all_parents_have_valid_children()?;
        self.validate_all_children_have_valid_parents()?;

        Ok(())
    }

    fn validate_all_children_have_valid_parents(&self) -> Result<(), AstError> {
        for node in self.sequential_iter().flat_map(|index| self.get_weak(index)) {
            if let Some(parent_node) = self.get_weak(node.parent_index) {
                let parents_children = parent_node.value.get_children();

                if !parents_children.contains(&node.self_index) {
                    return Err(AstError::ParentDoNotHaveChild { 
                        node: parent_node.self_index, 
                        expected_child: node.self_index
                    });
                }   
            }
        }

        Ok(())
    }

    fn validate_all_parents_have_valid_children(&self) -> Result<(), AstError> {
        for node in self.sequential_iter().flat_map(|index| self.get_weak(index)) {
            let children = node.value.get_children();

            for child in children.iter().flat_map(|index| self.get_weak(*index)) {

                if child.parent_index != node.self_index {
                    return Err(AstError::ChildNodeHasWrongParent { 
                        node: child.self_index, 
                        actual_parent: child.parent_index, 
                        expected_parent: node.self_index,
                    });
                }
            }
        }

        Ok(())
    }
}