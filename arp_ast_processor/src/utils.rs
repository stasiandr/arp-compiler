use crate::types::{
    ast_node_value::{Ast, WId},
    function::{Function, FunctionKind},
    simple::Identifier,
    type_collection::{TypeId, TypeInfo},
};

pub enum VariableSource {
    Local(usize),
    Argument(usize),
}

impl Ast {
    pub fn get_identifier_source(&self, ident: &Identifier, id: &WId) -> Option<VariableSource> {
        let func = self
            .get_parent_of_kind::<Function, _>(*id)
            .map(|index| self.get(&index));

        match func {
            Some(func) => match func.register_index_of(ident) {
                Some(register_index) => Some(VariableSource::Local(register_index)),
                None => {
                    func.parameters.iter().position(|(i, _)| i == ident).map(|i| if matches!(func.kind, FunctionKind::Static) { i } else { i + 1 }).map(VariableSource::Argument)
                }
            },
            None => None,
        }
    }

    pub fn resolve_type(&self, ty: TypeId, index: &WId) -> Option<&TypeInfo> {
        let arp_file = self.get_arp_file_in_parent(*index)?;
        arp_file.type_collection.try_get_strong(&ty)
    }
}
