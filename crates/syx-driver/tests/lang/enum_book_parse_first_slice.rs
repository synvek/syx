//! S1-I2: enum `case` / unlabeled associated / `: Error` parse first pack.

use syx_ast::Decl;
use syx_diagnostics::SourceFile;
use syx_parser::Parser;

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn parse_file(rel: &str) -> syx_ast::Module {
    let path = fixture(rel);
    let src = std::fs::read_to_string(&path).expect("read fixture");
    let file = SourceFile::new(path.to_string_lossy().into_owned(), src);
    Parser::parse(file).unwrap_or_else(|e| panic!("parse {rel}: {e}"))
}

#[test]
fn s1_03_enum_status_case_keyword_parses() {
    let module = parse_file("lang/enum_book_status.sy");
    let cases = module
        .decls
        .iter()
        .find_map(|d| match d {
            Decl::Enum { name, cases, .. } if name.as_str() == "Status" => Some(cases.clone()),
            _ => None,
        })
        .expect("Status enum");
    assert_eq!(cases.len(), 3);
    assert_eq!(cases[0].0.as_str(), "ok");
    assert!(cases[0].2.is_empty());
    assert_eq!(cases[1].0.as_str(), "err");
    assert_eq!(cases[2].0.as_str(), "pending");
}

#[test]
fn s1_03_enum_associated_unlabeled_and_error_conformance_parses() {
    let module = parse_file("lang/enum_book_associated.sy");
    let cases = module
        .decls
        .iter()
        .find_map(|d| match d {
            Decl::Enum {
                name, cases, ..
            } if name.as_str() == "NetworkResult" => Some(cases.clone()),
            _ => None,
        })
        .expect("NetworkResult enum");
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0].0.as_str(), "success");
    assert_eq!(cases[0].2.len(), 1);
    assert_eq!(cases[0].2[0].0.as_str(), "_0");
    assert_eq!(cases[1].0.as_str(), "failure");
    assert_eq!(cases[1].2.len(), 2);
    assert_eq!(cases[1].2[0].0.as_str(), "code");
    assert_eq!(cases[1].2[1].0.as_str(), "message");
}

#[test]
fn s1_03_legacy_enum_associated_without_case_still_parses() {
    let module = parse_file("lang/enum_associated.sy");
    assert!(module.decls.iter().any(|d| matches!(
        d,
        Decl::Enum { name, .. } if name.as_str() == "NetworkResult"
    )));
}
