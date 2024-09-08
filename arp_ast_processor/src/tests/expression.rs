use arp_types::sources::Source;

use crate::{chumsky_folder::{ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Ast, Id}, expression::{BinaryOperator, Expression, Literal, UnaryOperator}}};

// TODO this kinda needs reexport or something to accept parser as argument
fn test_parse_expr(input: &str, tag: Option<&str>) -> Ast {
    let source = Source::new_inline(tag.unwrap_or("untagged"), input);
    let tokens = arp_lexer::lex_tokens(&source).unwrap();
    let chumsky_root = arp_parser::parse(source.len(), &tokens, arp_parser::expression::expr_parser()).unwrap();
    let mut folder = ChumskyNodeVisitor::default();
    let _:Id<Expression> = folder.fold(&chumsky_root, folder.ast.get_root_index()).unwrap();
    folder.ast
}


#[test]
fn literal() {
    let ast = test_parse_expr("1", Some("test/literal"));
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Literal(Literal::Integer(1)))
    });
    assert_eq!(nodes.len(), 1);
}

#[test]
fn simple_expressions() {
    let ast = test_parse_expr("1 + 1", Some("test/binary"));
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Literal(Literal::Integer(1)))
    });
    assert_eq!(nodes.len(), 2);

    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Binary { lhs: _, op: BinaryOperator::Add, rhs: _ })
    });

    assert_eq!(nodes.len(), 1);

    let ast = test_parse_expr("-1", Some("test/unary"));
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Unary { op: UnaryOperator::Negate, expr: _ })
    });
    assert_eq!(nodes.len(), 1);
}


#[test]
fn complex_expressions() {
    let ast = test_parse_expr("x + 1", Some("test/binary"));
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Variable(_))
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_expr("x.field", Some("test/field"));

    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Variable(_))
    });
    assert_eq!(nodes.len(), 1);
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::GetField{ .. })
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_expr("x.method()", Some("test/method"));

    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Variable(_))
    });
    assert_eq!(nodes.len(), 1);
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        match node {
            Expression::Call { on: Some(_), method: _, args } => args.is_empty(),
            _ => false,
        }
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_expr("MyClass { my_field1: 1, my_field2 }", Some("test/method"));
    
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Construct { .. })
    });
    assert_eq!(nodes.len(), 1);

    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Variable(_))
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_expr("MyNamespace.MyClass { }", Some("test/method"));
    let nodes = ast.find_children_of_kind::<Expression, _>(ast.get_root_index(), |node| {
        matches!(node, Expression::Construct { .. })
    });
    assert_eq!(nodes.len(), 1);
}

#[test]
#[should_panic]
fn unexpected_expression() { test_parse_expr("x.+2", Some("test/unexpected/expr")); }