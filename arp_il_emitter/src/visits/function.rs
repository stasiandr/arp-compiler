use std::collections::HashSet;

use arp_ast_processor::types::{
    ast_node_value::{Ast, Id},
    function::{Function, FunctionKind},
};

use crate::{
    emitter::Emitter,
    il_token::{FunctionFlags, ILToken, Method},
    traits::Visitor,
    EmitError,
};

impl Visitor<Function> for Emitter {
    fn visit(&mut self, index: &Id<Function>, ast: &Ast) -> Result<(), EmitError> {
        let func = ast.get(index);

        let is_entrypoint = {
            let file = ast
                .get_arp_file_in_parent(index.as_weak())
                .ok_or(EmitError::ArpFileNotFound)?;

            file.arp_path.0.to_lowercase() == "main" && func.name.0.to_lowercase() == "main"
        };

        let mut flags = vec![
            FunctionFlags::Cil,
            FunctionFlags::Managed,
            match func.kind {
                FunctionKind::Method { .. } => FunctionFlags::IsStatic(false),
                FunctionKind::Static => FunctionFlags::IsStatic(true),
            },
        ];

        if is_entrypoint {
            flags.push(FunctionFlags::EntryPoint);
        }

        let method = Method {
            flags: HashSet::from_iter(flags),
            params: func
                .parameters
                .iter()
                .map(|(ident, ty)| {
                    ast.resolve_type(ty.clone(), &index.as_weak())
                        .ok_or(EmitError::CantResolveType)
                        .map(|ty| (ident.0.to_string(), ty.into()))
                })
                .collect::<Result<Vec<_>, _>>()?,
            return_ty: ast
                .resolve_type(func.return_type.clone(), &index.as_weak())
                .ok_or(EmitError::CantResolveType)?
                .into(),
            name: func.name.0.to_string(),
            registers: func
                .registers
                .iter()
                .map(|(_, ty)| {
                    ast.resolve_type(ty.clone(), &index.as_weak())
                        .ok_or(EmitError::CantResolveType)
                        .map(|ty| ty.into())
                })
                .collect::<Result<Vec<_>, _>>()?,
        };

        self.write(method);

        self.visit(&func.block, ast)?;

        self.write(ILToken::EndMethod(func.name.0.to_string()));

        Ok(())
    }
}
