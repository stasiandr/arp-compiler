use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::{chumsky_folder::{utils::parse_type, ChumskyFoldError, ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Id, WId}, function::Function, implementation::Implementation}};




impl Folder<Spanned<ChumskyNode>, Implementation> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;

    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<Id<Implementation>, Self::Error> {
        let impl_type = if let ChumskyNode::ImplementationDecl(impl_type, _functions) = item.get_value() {
            parse_type(impl_type, parent, &mut self.ast)?
        } else {
            return Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "implementation".into()));
        };

        let im = self.ast.push(Implementation {
            impl_type: impl_type.clone(),
            functions: vec![],
        }, parent);

        let functions = match item.get_value() {
            ChumskyNode::ImplementationDecl(_impl_type, functions) => {
                Ok(functions.iter().flat_map(|node| {
                    match node.get_value() {
                        ChumskyNode::FuncDecl(..) => Ok(self.fold(node, im.as_weak())),
                        _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "function".into()))
                    }
                }).collect::<Result<Vec<Id<Function>>, _>>()?)
            },
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "implementation".into()))
        }?;

        self.ast.get_mut(&im).functions.extend(functions);

        let funcs = self.ast.get(&im).functions.iter().map(|id| (*id, self.ast.get(id).clone())).collect::<Vec<_>>();

        if let Some(file) = self.ast.get_mut_arp_file_in_parent(im.as_weak()) { 
            file.type_collection.extend_type_methods(impl_type, funcs)
        }
        

        Ok(im)
    }
}
