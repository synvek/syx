//! S1-I2: book-style `struct` check (+ run) and S1-06 let+mutating reject.

use syx_ast::Decl;
use syx_diagnostics::SourceFile;
use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};
use syx_parser::Parser;

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn parse_file(rel: &str) -> syx_ast::Module {
    let path = fixture(rel);
    let src = std::fs::read_to_string(&path).expect("read fixture");
    let file = SourceFile::new(path.to_string_lossy().into_owned(), src);
    Parser::parse(file).expect("parse")
}

#[test]
fn s1_02_struct_point_parses_var_fields() {
    let module = parse_file("lang/struct_book_point.sy");
    let st = module
        .decls
        .iter()
        .find_map(|d| match d {
            Decl::Struct { name, fields, members, .. } if name.as_str() == "Point" => {
                Some((fields.clone(), members.len()))
            }
            _ => None,
        })
        .expect("Point struct");
    assert_eq!(st.0.len(), 2);
    assert_eq!(st.0[0].0.as_str(), "x");
    assert_eq!(st.0[1].0.as_str(), "y");
    assert!(st.1 >= 2, "var fields should land in members");
}

#[test]
fn s1_02_struct_counter_parses_mutating_and_init_default() {
    let module = parse_file("lang/struct_book_counter.sy");
    let (fields, members) = module
        .decls
        .iter()
        .find_map(|d| match d {
            Decl::Struct { name, fields, members, .. } if name.as_str() == "Counter" => {
                Some((fields.clone(), members.clone()))
            }
            _ => None,
        })
        .expect("Counter struct");
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].0.as_str(), "value");
    let has_mutating = members.iter().any(|m| matches!(
        m,
        Decl::Function { name, attrs, .. }
            if name.as_str() == "increment"
                && attrs.iter().any(|a| a.name.as_str() == "mutating")
    ));
    assert!(has_mutating, "mutating func increment must parse with @mutating attr");
}

#[test]
fn s1_02_struct_point_check_and_run() {
    let path = fixture("lang/struct_book_point.sy");
    compile_file_with_options(&path, &opts()).expect("Point check/compile");
    let (code, lines) = run_file_printed(&path, &opts()).expect("Point run");
    assert_eq!(code, 0, "Point exit code");
    let out = lines.join("\n");
    assert!(out.contains('1'), "expected x printed, got {out:?}");
    assert!(out.contains('2'), "expected y printed, got {out:?}");
}

#[test]
fn s1_02_struct_counter_check_and_run() {
    let path = fixture("lang/struct_book_counter.sy");
    compile_file_with_options(&path, &opts()).expect("Counter check/compile");
    let (code, lines) = run_file_printed(&path, &opts()).expect("Counter run");
    assert_eq!(code, 0, "Counter exit code");
    let out = lines.join("\n");
    assert!(
        out.contains('1'),
        "expected value 1 after increment, got {out:?}"
    );
}

#[test]
fn s1_06_let_append_must_fail() {
    let path = fixture("lang/let_append_must_fail.sy");
    let compiled = compile_file_with_options(&path, &opts());
    assert!(
        compiled.is_err(),
        "let-bound Array.append must fail semantic check (S060)"
    );
    let err = compiled.unwrap_err().to_string();
    assert!(
        err.contains("S060") || err.contains("mutating") || err.contains("immutable"),
        "expected S060 / mutating diagnostic, got: {err}"
    );
}
