use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::{chumsky_folder::{utils::{parse_ident, parse_type_as_ident}, ChumskyFoldError, ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Id, WId}, file::{ArpFile, Import}}};


impl Folder<Spanned<ChumskyNode>, ArpFile> for ChumskyNodeVisitor {
    type Error = ChumskyFoldError;
    
    fn fold(&mut self, item: &Spanned<ChumskyNode>, parent: WId) -> Result<Id<ArpFile>, Self::Error> {
        let declarations = match item.get_value() {
            ChumskyNode::File(decl) => Ok(decl),
            _ => Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "file".into()))
        }?;

        let arp_file_index = self.ast.push(ArpFile::default(), parent);
        let mut tmp_file = ArpFile::default();

        for node in declarations {
            match node.get_value() {
                ChumskyNode::ImportDecl(is_extern, path, types) => {
            
                    let path = path.iter().map(|p| match p.get_value() {
                            ChumskyNode::Identifier(name) => Ok(name.as_ref()),
                            _ => Err(ChumskyFoldError::UnexpectedChumsky(p.clone(), "identifier".into()))
                        }).collect::<Result<Vec<_>, _>>()?.join(".");


                    let import_types = types.iter()
                        .map(|t| {
                            match t.get_value() {
                                ChumskyNode::Identifier(_) => parse_ident(t),
                                ChumskyNode::Type(_) => parse_type_as_ident(t),
                                _ => unreachable!(),
                            }
                            
                        })
                        .collect::<Result<Vec<_>, _>>()?;


                    let import_declaration = Import {
                        is_extern: *is_extern,
                        path: path.into(),
                        import_types,
                    };
                    tmp_file.imports.push(import_declaration);
                },
                ChumskyNode::ImplementationDecl(..) => tmp_file.implementations.push(self.fold(node, arp_file_index.as_weak())?),
                ChumskyNode::Structure(..) => tmp_file.structures.push(self.fold(node, arp_file_index.as_weak())?),
                ChumskyNode::FuncDecl(..) => tmp_file.functions.push(self.fold(node, arp_file_index.as_weak())?),
                _ => return Err(ChumskyFoldError::UnexpectedChumsky(item.clone(), "file".into()))
            };
        }

        let file = self.ast.get_mut(&arp_file_index);
        file.imports.extend(tmp_file.imports);
        file.structures.extend(tmp_file.structures);
        file.implementations.extend(tmp_file.implementations);
        file.functions.extend(tmp_file.functions);

        Ok(arp_file_index)
    }

}