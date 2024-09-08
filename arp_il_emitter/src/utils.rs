use arp_ast_processor::types::type_collection::TypeInfo;

use crate::il_token::ResolvedType;

impl From<&TypeInfo> for ResolvedType {
    fn from(value: &TypeInfo) -> Self {
        match &value.short_name {
            Some(short) => Self(short.clone()),
            None => Self(format!("class {}", value.full_name).to_string()),
        }
    }
}
