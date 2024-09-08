#![cfg(test)]

use crate::{ast::AbstractAst, derive_implementations};

use super::{index::{StrongIndex, WeakIndex}, traits::{AstNodeUnion, GetChildren, PushRemoveRootChildren}};


// --- Test AstNodeValue ---

#[derive(Debug, Clone, PartialEq)]
pub enum TestAstNodeValue {
    TestExpression(TestExpression),
    TestLiteral(TestLiteral),

    Root(Vec<WeakIndex>),
}

impl Default for TestAstNodeValue {
    fn default() -> Self {
        Self::Root(vec![])
    }
}

impl PushRemoveRootChildren for TestAstNodeValue {
    fn push_child(&mut self, index: WeakIndex) {
        match self {
            TestAstNodeValue::Root(vec) => vec.push(index),
            _ => unreachable!("It's not root!"),
        };
    }

    fn remove_child(&mut self, index: WeakIndex) {
        match self {
            TestAstNodeValue::Root(vec) => vec.remove(vec.iter().position(|i| *i == index).expect("Node not found")),
            _ => unreachable!("It's not root!"),
            
        };
    }
}

impl AstNodeUnion for TestAstNodeValue { }

impl GetChildren for TestAstNodeValue {
    fn get_children(&self) -> Vec<WeakIndex> {
        match self {
            TestAstNodeValue::TestExpression(expr) => expr.get_children(),
            TestAstNodeValue::TestLiteral(lit) => lit.get_children(),
            TestAstNodeValue::Root(vec) => vec.clone(),
        }
    }
}

// --- Test Literal ---


#[derive(Debug, Clone, PartialEq)]
pub enum TestLiteral {
    Integer(i64),
    Float(f64),
}

derive_implementations!(TestAstNodeValue, TestAstNodeValue::TestLiteral, TestLiteral);

impl GetChildren for TestLiteral {
    fn get_children(&self) -> Vec<WeakIndex> {
        vec![]
    }
}


// --- Test Expression ---


#[derive(Debug, Clone, PartialEq)]
pub enum TestExpression {
    Binary {
        lhs: StrongIndex<TestExpression>,
        rhs: StrongIndex<TestExpression>
    },
    Unary(StrongIndex<TestExpression>),
    Literal(StrongIndex<TestLiteral>),
}

derive_implementations!(TestAstNodeValue, TestAstNodeValue::TestExpression, TestExpression);

impl GetChildren for TestExpression {
    fn get_children(&self) -> Vec<WeakIndex> {

        let mut v = Vec::new();

        match self {
            TestExpression::Binary { lhs, rhs } => {
                v.push((*lhs).demote());
                v.push((*rhs).demote());
            },
            TestExpression::Unary(expr) => v.push((*expr).demote()),
            TestExpression::Literal(expr) => v.push((*expr).demote()),
        }

        v
    }
}

#[test]
pub fn mutate_simple_type() {
    let mut ast = AbstractAst::<TestAstNodeValue>::default();
    let literal = TestLiteral::Integer(123);
    let index = ast.push(literal, ast.get_root_index());

    let value = ast.get(&index);
    assert!(*value == TestLiteral::Integer(123));

    let mut_value = ast.get_mut(&index);
    match mut_value {
        TestLiteral::Integer(i) => {*i = 44},
        _ => panic!(),
    }

    let value = ast.get(&index);
    assert!(*value == TestLiteral::Integer(44));

    let mut_value = ast.get_mut(&index);
    match mut_value {
        TestLiteral::Integer(i) => {*i = 12},
        _ => panic!(),
    }

    let value = ast.get(&index);
    assert!(*value == TestLiteral::Integer(12));

    ast.validate().unwrap();
}


#[test]
pub fn complex_type() {
    let mut ast = AbstractAst::<TestAstNodeValue>::default();

    let expr_index = ast.next_index(ast.get_root_index());

    let literal_index = ast.push(TestLiteral::Float(12.34), expr_index);

    let expr_index = ast.place(expr_index, TestExpression::Literal(literal_index));

    let expr = ast.get(&expr_index);
    assert!(matches!(expr, TestExpression::Literal(_)));

    if let TestExpression::Literal(lit_index) = expr {
        assert!(*lit_index == literal_index);

        let lit = ast.get(lit_index);
        assert!(matches!(lit, TestLiteral::Float(12.34)))

    } else {
        panic!()
    }

    ast.validate().unwrap();
}

fn generate_ast() -> (AbstractAst<TestAstNodeValue>, StrongIndex<TestExpression>)  {
    let mut ast = AbstractAst::<TestAstNodeValue>::default();

    let bin_index = ast.next_index(ast.get_root_index());

    let lhs_index = ast.next_index(bin_index);
    let rhs_index = ast.next_index(bin_index);

    let left_index = ast.push(TestLiteral::Float(12.34), lhs_index);
    let right_index = ast.push(TestLiteral::Float(12.34), rhs_index);

    let lhs_index = ast.place(lhs_index, TestExpression::Literal(left_index));
    let rhs_index = ast.place(rhs_index, TestExpression::Literal(right_index));

    let bin_index = ast.place(bin_index, TestExpression::Binary { lhs: lhs_index, rhs: rhs_index });

    (ast, bin_index)
}

#[test]
pub fn recursive_type() {
    let (ast, _) = generate_ast();

    ast.validate().unwrap();
}

#[test]
#[should_panic]
pub fn validation_failed() {

    let mut ast = AbstractAst::<TestAstNodeValue>::default();

    let expr_index = ast.next_index(ast.get_root_index());

    let left_index = ast.push(TestLiteral::Float(12.34), expr_index);
    let _right_index = ast.push(TestLiteral::Float(12.34), expr_index);

    ast.place(expr_index, TestExpression::Literal(left_index));

    match ast.validate() {
        Ok(_) => {
            println!("{:#?}", ast);
        },
        Err(err) => {
            panic!("{}", err);
        },
    };
}

#[test]
pub fn get_relatives_of_kind() {
    let (ast, root_expr) = generate_ast();

    assert!(ast.get_child_of_kind::<TestExpression, _>(root_expr).is_some());
    assert!(ast.get_child_of_kind::<TestLiteral, _>(root_expr.demote()).is_some());
    
    let children = ast.get_children_of_kind::<TestLiteral, _>(root_expr);

    assert_eq!(children.len(), 2);

    assert!(ast.get_child_of_kind::<TestLiteral, _>(*children.first().unwrap()).is_some());
}

#[test]
pub fn modifications() {
    let (mut ast, root_expr) = generate_ast();
    if let Some(literal_id) = ast.get_child_of_kind::<TestLiteral, _>(root_expr) {
        assert!(matches!(ast.get(&literal_id), TestLiteral::Float(_)));

        ast.mutate_value(&literal_id, |val| {
            *val = TestLiteral::Integer(123);
        });
    }

    if let Some(literal_id) = ast.get_child_of_kind::<TestLiteral, _>(root_expr) {
        assert!(matches!(ast.get(&literal_id), TestLiteral::Integer(123)))
    }
}