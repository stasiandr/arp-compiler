use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::{chumsky_folder::{utils::{parse_ident, parse_type}, ChumskyFoldError, ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Id, WId}, structure::{Structure, StructureKind}, type_collection::TypeId}};


impl Folder<Spanned<ChumskyNode>, Structure> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;

    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<Id<Structure>, Self::Error> {
        let next = self.ast.next_index(parent);

        let structure = match item.get_value() {
            ChumskyNode::Structure(structure_type, _inherits_from, fields) => {
                let mut parsed_fields = vec![];

                for parameter in fields {
                    match parameter.get_value() {
                        ChumskyNode::VarAndType(ident, t) => {
                            let ident = parse_ident(ident);
                            let ty = parse_type(t, parent, &mut self.ast)?;

                            parsed_fields.push((ident?, ty));
                        },
                        _ => return Err(ChumskyFoldError::UnexpectedChumsky(parameter.clone(), "identifier".into()))
                    };
                }

                let self_type = match parse_type(structure_type, parent, &mut self.ast) {
                    Ok(TypeId::Weak(id)) => match self.ast.get_mut_arp_file_in_parent(parent) {
                        Some(file) => {
                            file.type_collection.try_allocate(id,  parsed_fields.clone())
                        },
                        None => TypeId::None,
                    },
                    _ => TypeId::None,
                };

                Ok(Structure {
                    kind: StructureKind::Class,
                    self_type,
                })
            }
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "function".into()))
        }?;

        let self_type = structure.self_type.clone();
        let index = self.ast.place_spanned(next, structure, item.get_span());

        if let Some(file) = self.ast.get_mut_arp_file_in_parent(parent) {
            file.type_collection.resolve_recursive(self_type);
        };

        Ok(index)
    }
}
