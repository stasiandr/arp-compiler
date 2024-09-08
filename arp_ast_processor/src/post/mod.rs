use crate::{errors::ProcessingError, types::{ast_node_value::{Ast, Id}, function::Function, statement::Statement}};

pub fn post_process(mut ast: Ast) -> Result<Ast, ProcessingError> {

    let functions = ast.get_children_of_kind::<Function, _>(ast.get_root_index());
    for func in functions {
        ast = build_registers(ast, func)?;
    }


    Ok(ast)
}


fn build_registers(mut ast: Ast, index: Id<Function>) -> Result<Ast, ProcessingError> {
    let mut registers = vec![];

    for stmt in ast.get_children_of_kind::<Statement, _>(index).into_iter().map(|index| ast.get(&index)) {
        if let Statement::LocalVariableDeclaration { ident, ty, .. }  = stmt {
            registers.push((ident.clone(), ty.clone()));
        }
    }
    
    let stmt = ast.get_mut(&index);
    stmt.registers.extend(registers);

    Ok(ast)
} 


