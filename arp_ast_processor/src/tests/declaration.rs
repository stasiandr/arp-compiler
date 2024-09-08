use arp_types::sources::Source;

use crate::{chumsky_folder::{ChumskyNodeVisitor, Folder}, types::{ast_node_value::{Ast, Id}, declaration::Declaration, file::ArpFile, function::Function, implementation::Implementation, structure::Structure, type_collection::TypeId}};

fn test_parse_declaration(input: &str, tag: Option<&str>) -> Ast {
    let source = Source::new_inline(tag.unwrap_or("untagged"), input);
    let tokens = arp_lexer::lex_tokens(&source).unwrap();
    let parser = arp_parser::declaration::declaration_parser();
    let chumsky_root = arp_parser::parse(source.len(), &tokens, parser).unwrap();
    let mut folder = ChumskyNodeVisitor::default();
    let _: Id<Declaration> = folder.fold(&chumsky_root, folder.ast.get_root_index()).unwrap();
    folder.ast
}

fn test_parse_file(input: &str, tag: Option<&str>) -> Ast {
    let source = Source::new_inline(tag.unwrap_or("untagged"), input);
    let tokens = arp_lexer::lex_tokens(&source).unwrap();
    let chumsky_root = arp_parser::parse_arp_file(source.len(), &tokens).unwrap();
    let mut folder = ChumskyNodeVisitor::default();
    let _: Id<Declaration> = folder.fold(&chumsky_root, folder.ast.get_root_index()).unwrap();
    folder.ast
}



#[test]
fn function() {
    let ast = test_parse_declaration("fn some_func() { }", Some("test/function"));
    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |node| {
        assert!(node.name == "some_func".into());
        assert!(node.parameters.is_empty());

        true
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_declaration("fn some_func(par: par_type, par2: par_type2) { par }", Some("test/function"));
    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |node| {
        assert!(node.name == "some_func".into());
        assert!(node.parameters.len() == 2);

        true
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_declaration("fn some_func(this, par2: par_type2) { par }", Some("test/function"));
    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |node| {
        assert!(node.name == "some_func".into());
        assert!(node.parameters.len() == 1);

        true
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_declaration("fn some_func() -> int { }", Some("test/function"));
    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |node| {
        assert!(node.name == "some_func".into());
        assert!(node.parameters.is_empty());
        assert!(matches!(node.return_type, TypeId::None));

        true
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_declaration("fn some_func() { let x = 1; if x == 2 { let y = 3; } else { let z = 4; } }", Some("test/trailing_function_decl"));
    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |_| {
        true
    });
    assert_eq!(nodes.len(), 1);
}



#[test]
fn structure() {
    let ast = test_parse_declaration("class MyClass { }", Some("test/class"));
    let nodes = ast.find_children_of_kind::<Structure, _>(ast.get_root_index(), |_| {
        true
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_declaration("class MyClass { fld: typ, fld2: typ2 }", Some("test/class"));
    let nodes = ast.find_children_of_kind::<Structure, _>(ast.get_root_index(), |_| {
        true
    });
    assert_eq!(nodes.len(), 1);
}


#[test]
fn impl_decl() {
    let ast = test_parse_declaration("impl MyClass { fn some_func() { } fn another_func(this) { } }", Some("test/import"));
    let nodes = ast.find_children_of_kind::<Implementation, _>(ast.get_root_index(), |node| {
        assert_eq!(node.functions.len(), 2);
        true
    });
    assert_eq!(nodes.len(), 1);
}


#[test]
fn trailing_function_decl() {
    let ast = test_parse_file("fn some_func() { let x = 1; }", Some("test/trailing_function_decl"));
    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |_| {
        true
    });
    assert_eq!(nodes.len(), 1);
}


#[test]
fn import() {
    let ast = test_parse_file("from path.to.file import MyClass", Some("test/import"));
    let nodes = ast.find_children_of_kind::<ArpFile, _>(ast.get_root_index(), |node| {
        assert_eq!(node.imports.len(), 1);
        true
    });
    assert_eq!(nodes.len(), 1);

    let ast = test_parse_file("from extern System.Console import Console", Some("test/import"));
    let nodes = ast.find_children_of_kind::<ArpFile, _>(ast.get_root_index(), |node| {
        assert_eq!(node.imports.len(), 1);
        assert!(node.imports[0].is_extern);
        true
    });
    assert_eq!(nodes.len(), 1);
}


#[test]
fn file() {
    let ast = test_parse_file("
    from path.to.file import AnotherClass

    class MyClass {
        fld: typ,
        fld2: typ2,
    }

    impl MyClass { fn some_func() { } fn another_func(this) { } }
    ", Some("test/file"));
    let nodes = ast.find_children_of_kind::<Structure, _>(ast.get_root_index(), |_| {
        true
    });
    assert_eq!(nodes.len(), 1);

    let nodes = ast.find_children_of_kind::<Implementation, _>(ast.get_root_index(), |node| {
        assert_eq!(node.functions.len(), 2);
        true
    });
    assert_eq!(nodes.len(), 1);

    let nodes = ast.find_children_of_kind::<Function, _>(ast.get_root_index(), |_| {
        true
    });
    assert_eq!(nodes.len(), 2);

    let nodes = ast.find_children_of_kind::<ArpFile, _>(ast.get_root_index(), |node| {
        assert_eq!(node.imports.len(), 1);
        assert_eq!(node.structures.len(), 1);
        assert_eq!(node.implementations.len(), 1);
        assert_eq!(node.functions.len(), 0);
        true
    });
    assert_eq!(nodes.len(), 1);
}



#[test]
#[should_panic]
fn invalid_file() {
    test_parse_file("
from path.to.file import AnotherClass
impl MyClass { 
    class MyClass {
        fld: typ,
        fld2: typ2,
    } fn some_func() { } fn another_func(this) { } }
    ", Some("test/file"));
}

#[test]
#[should_panic]
fn invalid_file_2() {
    test_parse_file("
from path.to.file import AnotherClass
class MyClass {
        fld: typ,
        fld2: typ2,

        fn some_func() { }
    }

impl MyClass { fn another_func(this) { } }
    ", Some("test/file"));
}


#[test]
#[should_panic]
fn invalid_file_3() {
    test_parse_file("
from path.to.file import AnotherClass, 
fn some_func() {
    class MyClass {
    }
}
    ", Some("test/file"));
}