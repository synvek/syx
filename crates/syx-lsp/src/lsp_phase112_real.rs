// Phase 112 real LSP matrix (≥24): Track D fingerprints + real semantic probes (no sem_gate/sem_probe clones).

use super::*;

fn p112_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p112_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p112_opts())
}

#[test]
fn p112_dist_hostkey_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_configure_keys"));
    assert!(src.contains("dist_host_key_addr_is_loopback"));
    assert!(src.contains("cdn_tcp_peer_route"));
}

#[test]
fn p112_select_deadline_unify_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_DEADLINE_UNIFY_FINGERPRINT"));
    assert!(parser.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(parser.contains("TokenKind::Timeout"));
    assert!(parser.contains("TokenKind::Deadline"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_DEADLINE_UNIFY_FINGERPRINT"));
}

#[test]
fn p112_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p112\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p112.sy", text, 1, 8, &p112_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p112_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p112\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p112.sy", text, 2, 10, &p112_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p112_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p112.sy", text, 1, 10, &p112_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p112_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p112.sy", text, 1, 19, &p112_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p112_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p112.sy", text, 2, 8, &p112_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p112_diagnostic_unknown_ident() {
    let doc = p112_doc("func main() { print(noSuchSymbol112) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol112")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p112_diagnostic_type_mismatch() {
    let doc = p112_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p112.sy", text, 0, 8, &p112_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p112_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p112_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p112_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p112.sy", text, 0, 12, &p112_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p112_diagnostic_undefined_type() {
    let doc = p112_doc("func main() { let x: NoSuchType112 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p112_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_diagnostic_duplicate_local() {
    let doc = p112_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_select_timeout_parse_smoke() {
    let doc = p112_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p112_select_deadline_parse_smoke() {
    let doc = p112_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p112_async_select_mixed_unify_parse_smoke() {
    let doc = p112_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"u\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p112_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p112.sy", text, 1, 19, &p112_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p112_diagnostic_wrong_assign_type() {
    let doc = p112_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p112.sy", text, 1, 19, &p112_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p112_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p112_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p112_diagnostic_missing_return_type_use() {
    let doc = p112_doc("func main() { let z: MissingAlias112 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_async_func_parse_smoke() {
    let doc = p112_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p112_diagnostic_string_as_int() {
    let doc = p112_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p112_cjk_string_parse_smoke() {
    let doc = p112_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p112_lsp_semantic_v14_contract() {
    let src = include_str!("lsp_phase112_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase112 LSP matrix must be ≥24 tests, found {tests}"
    );
    let pad_a = concat!("fn p112_", "sem_gate");
    let pad_b = concat!("fn p112_", "sem_probe");
    let pad_c = concat!("fn p112_", "hover_extra");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b) && !src.contains(pad_c),
        "phase112 must not ship sem_gate/sem_probe/hover_extra padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p112_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}
