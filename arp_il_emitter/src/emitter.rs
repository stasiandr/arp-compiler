use arp_ast_processor::types::{
    ast_node_value::{Ast, AstNodeValue, WId},
    expression::Expression,
    file::ArpFile,
    function::Function,
    statement::Statement,
    type_collection::TypeId,
};

use crate::{
    il_token::{ILToken, OpCode, ResolvedType},
    traits::Visitor,
    EmitError,
};

#[derive(Debug)]
pub struct Emitter {
    tokens: Vec<ILToken>,
    label_index: usize,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            label_index: 0,
        }
    }

    pub fn next_label(&mut self) -> String {
        let result = format!("ARP_{}", self.label_index);
        self.label_index += 1;

        result
    }

    pub fn emit(&mut self, ast: &Ast) -> Result<Vec<ILToken>, EmitError> {
        let id = ast.get_root_index();
        self.emit_node(ast, id)
    }

    pub fn emit_node(&mut self, ast: &Ast, id: WId) -> Result<Vec<ILToken>, EmitError> {
        let root = ast.get_weak(id);

        if let Some(node) = root {
            match node.get_value() {
                AstNodeValue::Expression(_) => self.visit(
                    &ast.try_promote::<Expression>(id)
                        .ok_or(EmitError::Unknown)?,
                    ast,
                )?,
                AstNodeValue::Statement(_) => self.visit(
                    &ast.try_promote::<Statement>(id).ok_or(EmitError::Unknown)?,
                    ast,
                )?,
                AstNodeValue::Function(_) => self.visit(
                    &ast.try_promote::<Function>(id).ok_or(EmitError::Unknown)?,
                    ast,
                )?,
                AstNodeValue::ArpFile(_) => self.visit(
                    &ast.try_promote::<ArpFile>(id).ok_or(EmitError::Unknown)?,
                    ast,
                )?,
                AstNodeValue::Root(children) => {
                    for child in children.clone() {
                        self.emit_node(ast, child)?;
                    }
                }

                val => unreachable!("You must not use emit on: {val:?}"),
                // AstNodeValue::Declaration(_) => todo!(),
                // AstNodeValue::Structure(_) => todo!(),
                // AstNodeValue::Implementation(_) => todo!(),
                // AstNodeValue::BlockScope(_) => todo!(),
            }
        }

        Ok(self.tokens.clone())
    }

    pub fn write<T: Into<ILToken>>(&mut self, i: T) {
        self.tokens.push(i.into());
    }

    pub fn write_labeled_opcode(&mut self, opcode: OpCode, label: String) {
        self.tokens
            .push(ILToken::OpCode(OpCode::LabeledOpCode(label, opcode.into())));
    }

    // let ty = ast.resolve_type(structure.self_type.clone(), &index.as_weak()).ok_or(EmitError::CantResolveType)?;
    pub(crate) fn resolve_ty<I: Into<WId>>(
        ast: &Ast,
        ty: &TypeId,
        index: I,
    ) -> Result<ResolvedType, EmitError> {
        Ok(ast
            .resolve_type(ty.clone(), &index.into())
            .ok_or(EmitError::CantResolveType)?
            .into())
    }
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}
