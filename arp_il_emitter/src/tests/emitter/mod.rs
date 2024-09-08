use arp_ast_processor::types::{
    expression::Expression, file::ArpFile, function::Function, statement::Statement,
};

use crate::{
    il_token::{ILToken, OpCode},
    tests::test_emit,
};

#[test]
fn int_literal() {
    let tokens = test_emit::<Expression, _>("fn main() { let x = 1; }").unwrap();
    assert!(matches!(tokens[0], ILToken::OpCode(OpCode::LoadInt(1))));
}

#[test]
fn binary_op() {
    let tokens = test_emit::<Expression, _>("fn main() { let x = 1 + 2; }").unwrap();
    assert!(matches!(tokens[0], ILToken::OpCode(OpCode::LoadInt(1))));
    assert!(matches!(tokens[1], ILToken::OpCode(OpCode::LoadInt(2))));
    assert!(matches!(tokens[2], ILToken::OpCode(OpCode::Add)));
}

#[test]
fn declaration_statement() {
    let tokens = test_emit::<Statement, _>("fn main() { let x = 1; }").unwrap();

    assert!(matches!(tokens[0], ILToken::OpCode(OpCode::LoadInt(1))));
    assert!(matches!(tokens[1], ILToken::OpCode(OpCode::StoreLocalVariable(0))));
}

#[test]
fn function() {
    let tokens = test_emit::<Function, _>("fn main() { let x = 1; }").unwrap();

    assert!(matches!(tokens[0], ILToken::StartMethod(_)));
    assert!(matches!(tokens[1], ILToken::OpCode(OpCode::LoadInt(1))));
    assert!(matches!(tokens[2], ILToken::OpCode(OpCode::StoreLocalVariable(0))));
    assert!(matches!(tokens[3], ILToken::EndMethod(_)));
}

#[test]
fn arp_file() {
    let tokens = test_emit::<ArpFile, _>("fn main() { let x = 1; }").unwrap();

    assert!(matches!(tokens[0], ILToken::StartMethod(_)));
    assert!(matches!(tokens[1], ILToken::OpCode(OpCode::LoadInt(1))));
    assert!(matches!(tokens[2], ILToken::OpCode(OpCode::StoreLocalVariable(0))));
    assert!(matches!(tokens[3], ILToken::EndMethod(_)));
}
