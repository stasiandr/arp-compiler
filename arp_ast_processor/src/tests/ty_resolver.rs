use arp_types::sources::Source;

use crate::{chumsky_folder::ChumskyNodeVisitor, errors::ProcessingError, post::post_process, type_resolver::resolve_types_loop, types::ast_node_value::Ast, validations};

pub fn get_file_ast(input: &str, tag: Option<&str>) -> Result<Ast, ProcessingError> {
    let source = Source::new_inline(tag.unwrap_or("untagged"), input);
    let tokens = arp_lexer::lex_tokens(&source).unwrap();
    let chumsky_root = arp_parser::parse_arp_file(source.len(), &tokens).unwrap();
    let mut visitor = ChumskyNodeVisitor::default();
    visitor.fold_file(&chumsky_root);
    let ast = visitor.consume();
    let ast = resolve_types_loop(ast).unwrap();
    let ast = post_process(ast)?;
    
    dbg!(&ast);
    validations::validate(&ast).unwrap();
    
    Ok(ast)
}



#[test]
fn resolve_implicit_declaration() {
    get_file_ast("fn func() {let x = true;}", None).unwrap();
    get_file_ast("fn func() {let x = 1; let y = true;}", Some("test/expression_statement")).unwrap();
}


#[test]
fn resolve_implicit_declaration_for_expression() {
    get_file_ast("fn func() {let x = !true;}", Some("test/expression_statement")).unwrap();
    get_file_ast("fn func() {let x = 1 + 1;}", Some("test/expression_statement")).unwrap();
}


#[test]
fn resolve_implicit_declaration_for_sequential_declaration() {

    get_file_ast("fn func() { let x = 1; let y = x; }", Some("test/expression_statement")).unwrap();

    get_file_ast("fn func() {
    let x0 = 1; 
    let x1 = x0;
    let x2 = x1;
    let x3 = x2;
    let x4 = x3;
    let x5 = x4;
    let x6 = x5;
    }", Some("test/expression_statement")).unwrap();
}


#[test]
fn resolve_type_from_many_outer_scope() {
    get_file_ast("fn func() 
    {
        let x = 1;
        {{{{ let y = x; }}}}
    }
    ", Some("test/expression_statement")).unwrap();
}

#[test]
fn resolve_type_from_outer_scope() {
    get_file_ast("fn func() 
    {
        let x = 1;
        {
            let y = x;
        }
        let z = x;
    }
    ", Some("test/expression_statement")).unwrap();
}

#[test]
fn resolve_function() {
    let _ = get_file_ast("fn func(arg0: int32) { let x = arg0; }", None).unwrap();
}

#[test]
fn local_declared_type() {
    get_file_ast("class MyClass { } fn func() { let x = MyClass{}; }", None).unwrap();
    get_file_ast("class MyClass { } fn func(arg0: MyClass) { let x = arg0; }", None).unwrap();
}

#[test]
fn this_implementation() {
    get_file_ast("class MyClass { } impl MyClass { fn func(this) { let x = this; } }", None).unwrap();
}

#[test]
fn fields() {
    get_file_ast("class MyClass { field: int32 } fn func(arg0: MyClass) { let x = arg0.field; }", None).unwrap();
}


#[test]
fn method_call() {
    get_file_ast("class MyClass { field: int32 } impl MyClass { fn func(this) {} }   fn func(arg0: MyClass) { arg0.func(); }", None).unwrap();
}


#[test]
fn instance_and_static_methods() {
    get_file_ast("class MyClass { field: int32 } impl MyClass { fn func_instance(this) -> MyClass {this} fn func_static() -> int32 { 0 } }", None).unwrap();
}

#[test]
fn method_type_resolve() {
    get_file_ast("class MyClass { field: int32 } impl MyClass { fn func(this) -> MyClass {} } fn func(arg0: MyClass) { let x = arg0.func(); }", None).unwrap();
}

#[test]
fn static_method() {
    get_file_ast("class MyClass { field: int32 } impl MyClass { fn func() -> int32 { 0 } } fn func() { let x = MyClass.func(); }", None).unwrap();
}

#[test]
fn static_method_with_namespace() {
    get_file_ast("class MyNameSpace.MyClass { field: int32 } impl MyNameSpace.MyClass { fn func() -> int32 { 0 } } fn func() { let x = MyNameSpace.MyClass.func(); }", None).unwrap();
}

#[test]
fn static_method_with_namespace_long() {
    get_file_ast("class Very.Long.My.Name.Space.My.Class { field: int32 } impl Very.Long.My.Name.Space.My.Class { fn func() -> int32 { 0 } } fn func() { let x = Very.Long.My.Name.Space.My.Class.func(); }", None).unwrap();
}

#[test]
fn static_method_with_namespace_and_namespace_as_class() {
    get_file_ast("class MyNameSpace { field: int32 } class MyNameSpace.MyClass { } impl MyNameSpace.MyClass { fn func() -> int32 { 0 } } fn func() { let x = MyNameSpace.MyClass.func(); let y = MyNameSpace.field; }", None).unwrap();
}

#[test]
fn instance_and_static_methods_test() {
    get_file_ast("class MyClass { field: int32 } impl MyClass { fn func_instance(this) -> MyClass { this } fn func_static() -> MyClass { MyClass { } } } fn test() { let x = MyClass.func_static(); let y = x.func_instance(); y.func_instance(); }", None).unwrap();
}


#[test]
fn instance_and_static_methods_with_namespace_test() {
    get_file_ast("class My.Name.Space.MyClass { field: int32 } impl My.Name.Space.MyClass { fn func_instance(this) -> My.Name.Space.MyClass { this } fn func_static() -> My.Name.Space.MyClass { My.Name.Space.MyClass { } } } fn test() { let x = My.Name.Space.MyClass.func_static(); let y = x.func_instance(); y.func_instance(); }", None).unwrap();
}


#[test]
fn class_with_namespace() {
    get_file_ast("class MyNamespace.MyClass { } fn func(arg0: MyNamespace.MyClass) { let x = arg0; }", None).unwrap();
}

#[test]
fn class_with_namespace_as_constructor() {
    get_file_ast("class MyNamespace.MyClass { } fn func() { let x = MyNamespace.MyClass { }; }", None).unwrap();
}

#[test]
#[should_panic]
fn impl_before_declaration() {
    get_file_ast("fn func(arg0: MyClass) { let x = arg0.field; } class MyClass { }", None).unwrap();
}

#[test]
fn recursive_fields() {
    get_file_ast("class MyClass { field: MyClass } fn func(arg0: MyClass) { let x = arg0.field; }", None).unwrap();
}


#[test]
#[should_panic]
fn fail_resoling_before_declaration() {
    get_file_ast("fn func() 
    {
        let x = z;
        let z = 1;
    }
    ", Some("test/expression_statement")).unwrap();
}

#[test]
#[should_panic]
fn fail_resoling_from_inner_scope() {
    get_file_ast("fn func() 
    {
        let x = 1;
        {
            let y = x;
        }
        let z = y;
    }
    ", Some("test/expression_statement")).unwrap();
}



#[test]
#[should_panic]
fn fail_resoling_from_out_scope_before_declaration() {
    get_file_ast("fn func() 
    {
        let x = 1;
        {
            let y = z;
        }
        let z = x;
    }
    ", Some("test/expression_statement")).unwrap();
}

#[test]
#[should_panic]
fn mismatched_types_bin() {
    get_file_ast("fn func() {let x = 1 + true;}", Some("test/expression_statement")).unwrap();
}


// #[test]
// #[should_panic]
// fn unexpected_types_bin() {
//     get_file_ast("fn func() {let x = 1 and 2;}", Some("test/expression_statement")).unwrap();
// }


#[test]
#[should_panic]
fn unexpected_type_un() {
    get_file_ast("fn func() {let x = -false;}", Some("test/expression_statement")).unwrap();
}