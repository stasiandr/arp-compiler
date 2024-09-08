use crate::validations::validate;

use super::statement::test_parse_stmt;

#[test]
#[should_panic]
fn variable_has_to_type() {
    validate(&test_parse_stmt("{let x = 1;}", Some("test/expression_statement"))).unwrap();
}


#[test]
#[should_panic]
fn variable_has_to_type_cons() {
    validate(&test_parse_stmt("{let x = 1; let y = true;}", Some("test/expression_statement"))).unwrap();
}