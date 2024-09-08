use std::{collections::HashSet, vec};

use arp_ast_processor::types::{
    ast_node_value::{Ast, Id},
    structure::Structure,
};

use crate::{
    emitter::Emitter,
    il_token::{FunctionFlags, ILToken, Method, OpCode, StructureFlags},
    traits::Visitor,
    EmitError,
};

impl Visitor<Structure> for Emitter {
    fn visit(&mut self, index: &Id<Structure>, ast: &Ast) -> Result<(), EmitError> {
        let structure = ast.get(index);
        let ty = ast
            .resolve_type(structure.self_type.clone(), &index.as_weak())
            .ok_or(EmitError::CantResolveType)?;

        let flags = HashSet::from_iter(vec![StructureFlags::Auto]);
        self.write(ILToken::StartStructure(flags, ty.full_name.to_string()));

        for (fld, fld_ty) in &ty.fields {
            self.write(ILToken::Field(
                fld.0.to_string(),
                Emitter::resolve_ty(ast, fld_ty, *index)?,
            ))
        }

        let ty_collection = &ast.get_arp_file_in_parent(index.as_weak()).ok_or(EmitError::ArpFileNotFound)?.type_collection;

        self.write(ILToken::StartMethod(Method {
            flags: HashSet::from_iter(vec![
                FunctionFlags::Cil,
                FunctionFlags::Managed,
                FunctionFlags::IsStatic(false),
            ]),
            params: ty.fields.iter().map(|(ident, ty)| Emitter::resolve_ty(ast, ty, *index).map(|t| (ident.0.to_string(), t))).collect::<Result<Vec<_>, _>>()?,
            registers: vec![],
            return_ty: ty_collection.try_get_strong(&ty_collection.get_void()).ok_or(EmitError::CantResolveType)?.into(),
            name: ".ctor".to_string(),
        }));

        for (fld_index, (fld, fld_ty)) in ty.fields.iter().enumerate() {
            self.write(OpCode::LoadArgument(0));
            self.write(OpCode::LoadArgument(fld_index + 1));
            self.write(OpCode::SetField(Emitter::resolve_ty(ast, fld_ty, *index)?, ty.full_name.to_string(), fld.0.to_string()))
        }

        self.write(ILToken::EndMethod(".ctor".to_string()));


        for method in ty.methods.iter().flat_map(|mtd| mtd.definition) {
            self.visit(&method, ast)?;
        }

        self.write(ILToken::EndStructure(ty.full_name.to_string()));

        Ok(())
    }
}
