use arp_types::sources::Source;

use crate::{chumsky_folder::{ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Ast, Id}, block_scope::BlockScope, expression::{Expression, Literal}, statement::{IfKind, Statement}}};

pub(super) fn test_parse_stmt(input: &str, tag: Option<&str>) -> Ast {
    let source = Source::new_inline(tag.unwrap_or("untagged"), input);
    let tokens = arp_lexer::lex_tokens(&source).unwrap();
    let parser = arp_parser::statement::statement_parser();
    let chumsky_root = arp_parser::parse(source.len(), &tokens, parser).unwrap();
    let mut folder = ChumskyNodeVisitor::default();
    let _:Id<Statement> = folder.fold(&chumsky_root, folder.ast.get_root_index()).unwrap();
    folder.ast
}

#[test]
fn simple_stmt() {
    let ast = test_parse_stmt("1;", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::Expression(_))
    });
    assert_eq!(nodes.len(), 1);
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Literal(Literal::Integer(1)))
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("1 + 2 * 3;", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::Expression(_))
    });
    assert_eq!(nodes.len(), 1);


    let ast = test_parse_stmt("return 1;", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::Return(_))
    });
    assert_eq!(nodes.len(), 1);
}



#[test]
fn assi_stmt() {
    let ast = test_parse_stmt("x = 1;", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::Assignment { on: None, .. })
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("x.field = 3;", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::Assignment { on: Some(_), .. })
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("x.my_method().field = 3;", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::Assignment { on: Some(_), .. })
    });
    assert_eq!(nodes.len(), 1);
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Call { on: Some(_), .. })
    });
    assert_eq!(nodes.len(), 1);
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::GetField { .. })
    });
    assert_eq!(nodes.len(), 0);
}



#[test]
fn variable_declaration_statement() {
    let ast = test_parse_stmt("let x = 3;", Some("test/variable_declaration_is_not_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::LocalVariableDeclaration { .. })
    });
    assert_eq!(nodes.len(), 1);
}


#[test]
fn control_flow_statements() {
    let ast = test_parse_stmt("while x == 1 { }", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        matches!(node, Statement::WhileStmt { .. })
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("if x == 1 { }", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        match node {
            Statement::IfStmt(vec) => {
                vec.len() == 1 && matches!(vec[0], IfKind::If(_, _))
            },
            _ => false
        }
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("if x == 1 { } else if y == 2 { }", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        match node {
            Statement::IfStmt(vec) => {
                vec.len() == 2 && 
                matches!(vec[0], IfKind::If(_, _)) && 
                matches!(vec[1], IfKind::ElseIf(_, _))
            },
            _ => false
        }
    });
    assert_eq!(nodes.len(), 1);


    let ast  = test_parse_stmt("if x == 1 { } else if y == 2 { } else { }", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        match node {
            Statement::IfStmt(vec) => {
                vec.len() == 3 && 
                matches!(vec[0], IfKind::If(_, _)) && 
                matches!(vec[1], IfKind::ElseIf(_, _)) &&
                matches!(vec[2], IfKind::Else(_))
            },
            _ => false
        }
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("if x == 1 { } else { }", Some("test/expression_statement"));
    let nodes = ast.find_children_of_kind::<Statement, _>(ast.get_root_index(), |node| {
        match node {
            Statement::IfStmt(vec) => {
                vec.len() == 2 &&
                matches!(vec[0], IfKind::If(_, _)) && 
                matches!(vec[1], IfKind::Else(_))
            },
            _ => false
        }
    });
    assert_eq!(nodes.len(), 1);
}

#[test]
fn blocks() {
    let ast = test_parse_stmt("{ }", Some("test/block"));
    let nodes = ast.find_children_of_kind::<BlockScope, _>(ast.get_root_index(), |_| true);
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("{ let x = 2; x + 2 }", Some("test/block"));
    let nodes = ast.find_children_of_kind::<BlockScope, _>(ast.get_root_index(), |_| true);
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_stmt("{ { } { } }", Some("test/block"));
    let nodes = ast.find_children_of_kind::<BlockScope, _>(ast.get_root_index(), |_| true);
    assert_eq!(nodes.len(), 3);
}