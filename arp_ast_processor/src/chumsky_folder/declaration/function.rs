use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::{chumsky_folder::{utils::{parse_ident, parse_type}, ChumskyFoldError, ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Id, WId}, block_scope::{BlockScope, BlockScopeKind}, function::{Function, FunctionKind}, type_collection::TypeId}};



impl Folder<Spanned<ChumskyNode>, Function> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;

    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<Id<Function>, Self::Error> {
        let next = self.ast.next_index(parent);

        let func = match item.get_value() {
            ChumskyNode::FuncDecl(identifier, parameters, return_type, block) => {
                let mut parsed_parameters = vec![];
                let mut kind = FunctionKind::Static;

                for parameter in parameters {
                    match parameter.get_value() {
                        ChumskyNode::VarAndType(ident, t) => {
                            let ident = parse_ident(ident);
                            let ident_type = parse_type(t, parent, &mut self.ast)?;

                            parsed_parameters.push(
                                (ident?, ident_type)
                            );
                        },
                        ChumskyNode::MutThis(is_this_mutable) => {
                            kind = FunctionKind::Method { is_this_mutable: *is_this_mutable }
                        }
                        _ => return Err(ChumskyFoldError::UnexpectedChumsky(*identifier.clone(), "identifier".into()))
                    };
                }

                let ident = match identifier.get_value() {
                    ChumskyNode::Identifier(name) => Ok(name.as_ref().into()),
                    _ => Err(ChumskyFoldError::UnexpectedChumsky(*identifier.clone(), "identifier".into()))
                };


                let void = match self.ast.get_arp_file_in_parent(parent) {
                    Some(file) => {
                        file.type_collection.get_void()
                    },
                    None => TypeId::None,
                };

                let return_type = return_type.as_ref().map(|ty| parse_type(ty, parent, &mut self.ast)).unwrap_or(Ok(void))?;
                
                let block = self.fold(block.as_ref(), next)?;

                self.ast.mutate_value(&block, |block: &mut BlockScope| {
                    block.kind = BlockScopeKind::Function(next);
                });

                Ok(Function {
                    name: ident?,
                    parameters: parsed_parameters,
                    return_type,
                    block,
                    kind,
                    registers : vec![]
                })
            },
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "function".into()))
        };

        Ok(self.ast.place_spanned(next, func?, item.get_span()))
    }
}

