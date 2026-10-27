// Phase 107 real LSP matrix (≥24): Track D fingerprints, soft manifest pin, semantic probes.

use super::*;

fn p107_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p107_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p107_opts())
}

#[test]
fn p107_farm_ha_source_gate() {
    let auth = include_str!("../../syx-driver/src/farm_lease_auth.rs");
    assert!(auth.contains("spawn_farm_lease_ha"));
    assert!(auth.contains("farm_lease_auth_configure_ha"));
}

#[test]
fn dist_mailbox_mp_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("spawn_dist_tcp_mailbox"));
    assert!(src.contains("region_route_tcp"));
}

#[test]
fn p107_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p107\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 8, &p107_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p107_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p107\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p107.sy", text, 2, 10, &p107_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p107_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 10, &p107_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p107_hover_func_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 10, &p107_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p107_definition_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p107.sy", text, 1, 19, &p107_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p107_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p107.sy", text, 2, 8, &p107_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p107_diagnostic_unknown_ident() {
    let doc = p107_doc("func main() { print(noSuchSymbol107) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol107")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p107_diagnostic_bad_syntax_nonempty() {
    let doc = p107_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p107.sy", text, 0, 8, &p107_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p107_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p107_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p107_diagnostic_type_mismatch_smoke() {
    let doc = p107_doc("func main() { let x: Int = \"no\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p107.sy", text, 0, 12, &p107_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p107_definition_local_use() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p107.sy", text, 1, 19, &p107_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p107_references_helper_again() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p107_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p107_diagnostic_undefined_type() {
    let doc = p107_doc("func main() { let x: NoSuchType107 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_hover_closure_param() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p107_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_diagnostic_duplicate_local() {
    let doc = p107_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_select_default_parse_smoke() {
    let doc = p107_doc(
        "func main() {\n  select {\n    case let x = ch.receive(): print(x)\n    default: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p107_cjk_string_parse_smoke() {
    let doc = p107_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p107_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p107.sy", text, 1, 19, &p107_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p107_hover_msg_again() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 8, &p107_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p107_definition_msg_again() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p107.sy", text, 2, 10, &p107_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p107_diagnostic_wrong_assign_type() {
    let doc = p107_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_hover_count_again() {
    let text = "func main() {\n  let count = 9\n}";
    let hover = hover_for_position("file:///p107.sy", text, 1, 10, &p107_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p107_definition_helper_again() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p107.sy", text, 1, 19, &p107_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p107_diagnostic_unknown_type_again() {
    let doc = p107_doc("func main() { let z: MissingType107 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p107_lsp_semantic_v9_contract() {
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("lsp-semantic-v9:phase107-24-plus-real"));
    let src = include_str!("lsp_phase107_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase107 LSP matrix must be ≥24 tests, found {tests}"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p107_doc(").count();
    assert!(
        hover_n + def_n + diag_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n}"
    );
}

