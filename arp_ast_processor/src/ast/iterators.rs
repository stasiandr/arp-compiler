use super::{index::WeakIndex, traits::{AstNodeUnion, GetChildren}, AbstractAst};


impl<U : AstNodeUnion> AbstractAst<U> {

    pub fn sequential_iter(&self) -> AstSequentialIter<U> {
        AstSequentialIter { 
            ast: self, 
            index: 1 
        }
    }
    
    pub fn rec_iter_start_from<T : Into<WeakIndex> + Clone>(&self, index: T)  -> AstRecursiveIter<U> {
        AstRecursiveIter {
            ast: self,
            stack: vec![(index.into(), 0)],
        }
    }
    
}


pub struct AstSequentialIter<'a, U> where U: AstNodeUnion {
    ast: &'a AbstractAst<U>,
    index: usize
}

impl<'a, U: AstNodeUnion> Iterator for AstSequentialIter<'a, U> {
    type Item = WeakIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.ast.nodes.get(self.index) {
            self.index += 1;

            Some(node.self_index)
        } else {
            None
        }
    }
}


pub struct AstRecursiveIter<'a, U> where U: GetChildren {
    ast: &'a AbstractAst<U>,
    stack: Vec<(WeakIndex, usize)>
}

impl<'a, U: AstNodeUnion> Iterator for AstRecursiveIter<'a, U>  {
    type Item = (WeakIndex, usize);

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some((weak_id, rec)) = self.stack.pop() {
            if let Some(node) = self.ast.get_weak(weak_id) {
                
                let mut children = node.value.get_children();
                children.reverse(); // For prettier iterations over children using stack

                for child_id in children {
                    self.stack.push((child_id, rec + 1))
                }
                
                return Some((node.self_index, rec));
            }
        }
        
        None
    }
}

