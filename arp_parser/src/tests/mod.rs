#![cfg(test)]

use arp_types::{sources::Source, traits::AppendToReport, Spanned};
use chumsky::{prelude::end, Parser, Stream};

use crate::{declaration::declaration_parser, errors::ParserError, expression::expr, types::ChumskyNode};


fn test_parse_expr<'a, S : Into<&'a str>>(path: S, input: S) -> Spanned<ChumskyNode> {
    let source = Source::new_inline(path.into(), input.into());
    let tokens = arp_lexer::lex_tokens(&source).unwrap();

    let stream = Stream::from_iter(
        source.len()..source.len(), 
        tokens.iter().map(|spanned| spanned.clone().destruct()));
        
    let chumsky_node = match expr().then_ignore(end()).parse(stream) {
        Ok(ok) => ok,
        Err(errors) => {
            let errors = errors.iter().map(|e| ParserError::SimpleError(e.clone())).collect::<Vec<_>>();
            let report = ParserError::build_report(&errors, &source);

            report.print(ariadne::sources([
                (source.get_path_string(), source.content())
            ])).unwrap();

            panic!("Encountered errors!");
        },
    };
    chumsky_node.clone()
}

fn test_parse_stmt<'a, S : Into<&'a str>>(path: S, input: S) -> Spanned<ChumskyNode> {
    let source = Source::new_inline(path.into(), input.into());
    let tokens = arp_lexer::lex_tokens(&source).unwrap();

    let stream = Stream::from_iter(
        source.len()..source.len(), 
        tokens.iter().map(|spanned| spanned.clone().destruct()));
        
    let chumsky_node = match crate::statement::statement().then_ignore(end()).parse(stream) {
        Ok(ok) => ok,
        Err(errors) => {
            let errors = errors.iter().map(|e| ParserError::SimpleError(e.clone())).collect::<Vec<_>>();
            let report = ParserError::build_report(&errors, &source);

            report.print(ariadne::sources([
                (source.get_path_string(), source.content())
            ])).unwrap();

            panic!("Encountered errors!");
        },
    };
    chumsky_node.clone()
}


fn test_parse_decl<'a, S : Into<&'a str>>(path: S, input: S) -> Spanned<ChumskyNode> {
    let source = Source::new_inline(path.into(), input.into());
    let tokens = arp_lexer::lex_tokens(&source).unwrap();

    let stream = Stream::from_iter(
        source.len()..source.len(), 
        tokens.iter().map(|spanned| spanned.clone().destruct()));
        
    let chumsky_node = match declaration_parser().then_ignore(end()).parse(stream) {
        Ok(ok) => ok,
        Err(errors) => {
            let errors = errors.iter().map(|e| ParserError::SimpleError(e.clone())).collect::<Vec<_>>();
            let report = ParserError::build_report(&errors, &source);

            report.print(ariadne::sources([
                (source.get_path_string(), source.content())
            ])).unwrap();

            panic!("Encountered errors!");
        },
    };
    chumsky_node.clone()
}

#[test]
fn atom() {
    test_parse_expr("test/atom", "123");
}

#[test]
fn unary() {
    test_parse_expr("test/unary", "-1");
    test_parse_expr("test/unary", "---1");
}

#[test]
fn binary() {
    test_parse_expr("test/binary", "1 + 2");

    test_parse_expr("test/binary", "1 + 2");
    test_parse_expr("test/binary", "1 + 2 * 3");
    test_parse_expr("test/binary", "(1 + 2) * 3");

    test_parse_expr("test/construct", "NameSpace.MyClass { }");
    test_parse_expr("test/construct", "NameSpace.MyClass { 1, 2 }");
    test_parse_expr("test/construct", "NameSpace.MyClass { field: 1, other_field: 2, 3, }");
}

#[test]
fn identifier() {
    test_parse_expr("test/identifier", "variable");

    test_parse_expr("test/identifier", "variable + 1");
}


#[test]
fn call() {
    test_parse_expr("test/call", "func()");
}




#[test]
fn get() {
    test_parse_expr("test/method_call", "variable.func()");

    test_parse_expr("test/method_call", "variable.func(1 + 2, 3)");
    test_parse_expr("test/method_call", "variable.field.func(1 + 2, 3)");
    test_parse_expr("test/field", "variable.field");
}



#[test]
fn statement() {
    test_parse_stmt("test/statement/ExpressionStmt", "1 + 1;");
    test_parse_stmt("test/statement/AssignmentStmt", "x.field = 1;");
    test_parse_stmt("test/statement/BlockStmt", "{ x = 1; 2 + 2; }");
    test_parse_stmt("test/statement/BlockStmt/Return", "{ 1 + 2 }");
    test_parse_stmt("test/statement/IfStmt/Else", "if x == 1 { y = 2; } else { 3; }");
    test_parse_stmt("test/statement/IfStmt/ElseIfElse", "if x == 1 { y = 2; } else if x == 2 { y = 3; } else { 3; }");
    test_parse_stmt("test/statement/IfStmt", "if x == 1 { 2; 3; }");
    test_parse_stmt("test/statement/ForStmt", "for x in arr { }");
    test_parse_stmt("test/statement/WhileStmt", "while x == 1 { 2; }");
    test_parse_stmt("test/statement/ReturnStmt", "return 1;");
}




#[test]
fn declaration() {
    test_parse_decl("test/declaration/StatementDecl", "1 + 1;");
    test_parse_decl("test/declaration/VariableDecl", "let mut x: i32 = 12;");
    test_parse_decl("test/declaration/VariableDecl/Some", "fn func(arg1: typ1, arg2: type2) -> return_type { }");
    test_parse_decl("test/declaration/VariableDecl/None", "fn func(arg1: typ1, arg2: type2) { }");
    test_parse_decl("test/declaration/VariableDecl/BlockInside", "fn func(this, arg1: typ1) { }");
    test_parse_decl("test/declaration/VariableDecl/BlockInside", "fn func() { let x = 1; }");
    test_parse_decl("test/declaration/ClassDecl", "class MyClass : BaseClass, IInterface { ident1: typ1, ident2: type2, }");
    test_parse_decl("test/declaration/ClassDecl", "class MyClass { ident1: typ1, ident2: type2, }");
    test_parse_decl("test/declaration/ImplementationDecl", "impl Namespace.MyClass { fn func(arg1: typ1, arg2: type2) -> return_type { } fn func(arg1: typ1, arg2: type2) { } } ");
    test_parse_decl("test/declaration/ImportDecl", "from path.to.file import OuterClass, static_func");
    test_parse_decl("test/declaration/ImportDecl", "from extern path.to.file import OuterClass");
    test_parse_decl("test/declaration/ImportDecl", "from path.to.file import MyNamespace.MyClass");
}
