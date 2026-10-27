//! Suite-1: arithmetic / branch / call / string / Array·Dict (M0-D02).

use super::diff_exact;

#[test]
fn suite1_arith_add() {
    diff_exact("suite1/arith_add.sy");
}

#[test]
fn suite1_branch_if() {
    diff_exact("suite1/branch_if.sy");
}

#[test]
fn suite1_call_nested() {
    diff_exact("suite1/call_nested.sy");
}

#[test]
fn suite1_string_len() {
    diff_exact("suite1/string_len.sy");
}

#[test]
fn suite1_string_concat() {
    diff_exact("suite1/string_concat.sy");
}

#[test]
fn suite1_array_index() {
    diff_exact("suite1/array_index.sy");
}

#[test]
fn suite1_array_for_in() {
    diff_exact("suite1/array_for_in.sy");
}

#[test]
fn suite1_dict_subscript() {
    diff_exact("suite1/dict_subscript.sy");
}
