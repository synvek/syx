//! Suite-1: arithmetic / branch / call / string / Array·Dict (M0-D02).
//! S2-I141: `*_vm_mir` wrappers bind the S2-03 floor (VM+MIR only).

use super::{diff_exact, diff_exact_vm_mir};

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

#[test]
fn suite1_arith_add_vm_mir() {
    diff_exact_vm_mir("suite1/arith_add.sy");
}

#[test]
fn suite1_branch_if_vm_mir() {
    diff_exact_vm_mir("suite1/branch_if.sy");
}

#[test]
fn suite1_call_nested_vm_mir() {
    diff_exact_vm_mir("suite1/call_nested.sy");
}

#[test]
fn suite1_string_len_vm_mir() {
    diff_exact_vm_mir("suite1/string_len.sy");
}

#[test]
fn suite1_string_concat_vm_mir() {
    diff_exact_vm_mir("suite1/string_concat.sy");
}

#[test]
fn suite1_array_index_vm_mir() {
    diff_exact_vm_mir("suite1/array_index.sy");
}

#[test]
fn suite1_array_for_in_vm_mir() {
    diff_exact_vm_mir("suite1/array_for_in.sy");
}

#[test]
fn suite1_dict_subscript_vm_mir() {
    diff_exact_vm_mir("suite1/dict_subscript.sy");
}
