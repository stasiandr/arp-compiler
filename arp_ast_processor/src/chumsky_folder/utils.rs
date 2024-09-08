use arp_parser::types::ChumskyNode;
use arp_types::Spanned;

use crate::types::{ast_node_value::{Ast, WId}, simple::Identifier, type_collection::TypeId};

use super::ChumskyFoldError;


pub fn parse_ident(node: &Spanned<ChumskyNode>)  -> Result<Identifier, ChumskyFoldError> {
    match node.get_value() {
        ChumskyNode::Identifier(name) => Ok(name.as_ref().into()),
        _ => Err(ChumskyFoldError::UnexpectedChumsky(node.clone(), "identifier".into()))
    }
}

pub fn parse_type_as_ident(node: &Spanned<ChumskyNode>)  -> Result<Identifier, ChumskyFoldError> {
    match node.get_value() {
        ChumskyNode::Type(long_type) => {
            let type_name = long_type.iter().map(|node| match node.get_value() {
                ChumskyNode::Identifier(s) => Ok(s.clone()),
                _ => Err(ChumskyFoldError::CantUnfoldTypeName(long_type.clone())),
            }).collect::<Result<Vec<_>, _>>()?.join(".");
            Ok(type_name.into())
        },
        _ => Err(ChumskyFoldError::UnexpectedChumsky(node.clone(), "type".into()))
    }
}

pub fn parse_type(node: &Spanned<ChumskyNode>, parent: WId, ast: &mut Ast)  -> Result<TypeId, ChumskyFoldError> {
    match node.get_value() {
        ChumskyNode::Type(long_type) => {
            let type_name = long_type.iter().map(|node| match node.get_value() {
                ChumskyNode::Identifier(s) => Ok(s.clone()),
                _ => Err(ChumskyFoldError::CantUnfoldTypeName(long_type.clone())),
            }).collect::<Result<Vec<_>, _>>()?.join(".");
            
            match ast.get_mut_arp_file_in_parent(parent) {
                Some(arp_file) => {
                    Ok(arp_file.type_collection.get_or_allocate(&type_name))
                },
                // this is only for testing purposes. For other scenarios arp_file guaranteed to exist
                None => {
                    Ok(TypeId::None)
                }, 
            }
        },
        _ => Err(ChumskyFoldError::UnexpectedChumsky(node.clone(), "type".into()))
    }
}