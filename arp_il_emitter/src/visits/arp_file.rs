use arp_ast_processor::types::{
    ast_node_value::{Ast, Id},
    file::ArpFile,
};

use crate::{emitter::Emitter, traits::Visitor};

impl Visitor<ArpFile> for Emitter {
    fn visit(&mut self, index: &Id<ArpFile>, ast: &Ast) -> Result<(), crate::EmitError> {
        let file = ast.get(index);

        for func in &file.functions {
            self.visit(func, ast)?;
        }

        for structure in &file.structures {
            self.visit(structure, ast)?;
        }

        


        Ok(())
    }
}
