//! Language exports `export {…}` trailing-comma parser behavior (ADR-830).
//! Inventory signing removed in S2-I6; keep behavior nail only.

#[test]
fn language_exports_export_list_trailing_comma_parser_accepts() {
    let source = syx_diagnostics::SourceFile::new(
        "t.sy",
        "export { greet, }\nfunc greet() {}\nfunc main() { greet() }\n",
    );
    let module = syx_parser::Parser::parse(source).expect("trailing comma must parse");
    match &module.decls[0] {
        syx_ast::Decl::ExportList { names, .. } => {
            assert_eq!(
                names.iter().map(|n| n.as_str()).collect::<Vec<_>>(),
                vec!["greet"]
            );
        }
        other => panic!("expected ExportList, got {other:?}"),
    }
    let empty = syx_diagnostics::SourceFile::new("t.sy", "export { }\n");
    assert!(syx_parser::Parser::parse(empty).is_err());
}
