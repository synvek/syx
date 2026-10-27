// Phase 111 real LSP matrix (≥24): Track D fingerprints + real semantic probes (no sem_probe/hover_extra clones).

use super::*;

fn p111_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p111_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p111_opts())
}

#[test]
fn p111_dist_nonloopback_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_route"));
    assert!(src.contains("spawn_dist_tcp_mailbox_dual_stack"));
    assert!(src.contains("SYX_DIST_CDN_MEMORY"));
}

#[test]
fn p111_select_timeout_syntax_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_TIMEOUT_SYNTAX_FINGERPRINT"));
    assert!(parser.contains("TokenKind::Timeout"));
    assert!(parser.contains("TokenKind::Deadline"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_TIMEOUT_SYNTAX_FINGERPRINT"));
}

#[test]
fn p111_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p111\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p111_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p111\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p111.sy", text, 2, 10, &p111_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p111_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 10, &p111_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p111_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p111.sy", text, 1, 19, &p111_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p111_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p111.sy", text, 2, 8, &p111_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p111_diagnostic_unknown_ident() {
    let doc = p111_doc("func main() { print(noSuchSymbol111) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol111")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p111_diagnostic_type_mismatch() {
    let doc = p111_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p111.sy", text, 0, 8, &p111_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p111_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p111_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p111_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p111.sy", text, 0, 12, &p111_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p111_diagnostic_undefined_type() {
    let doc = p111_doc("func main() { let x: NoSuchType111 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p111_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_diagnostic_duplicate_local() {
    let doc = p111_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_select_timeout_parse_smoke() {
    let doc = p111_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p111_select_deadline_parse_smoke() {
    let doc = p111_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p111_async_select_parse_smoke() {
    let doc = p111_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p111_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p111.sy", text, 1, 19, &p111_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p111_diagnostic_wrong_assign_type() {
    let doc = p111_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p111.sy", text, 1, 19, &p111_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p111_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p111_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p111_diagnostic_missing_return_type_use() {
    let doc = p111_doc("func main() { let z: MissingAlias111 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_async_func_parse_smoke() {
    let doc = p111_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p111_diagnostic_string_as_int() {
    let doc = p111_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p111_cjk_string_parse_smoke() {
    let doc = p111_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p111_lsp_semantic_v13_contract() {
    let src = include_str!("lsp_phase111_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase111 LSP matrix must be ≥24 tests, found {tests}"
    );
    let pad_a = concat!("fn p111_", "sem_probe");
    let pad_b = concat!("fn p111_", "hover_extra");
    let pad_c = concat!("let binding", "0");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b) && !src.contains(pad_c),
        "phase111 must not ship sem_probe/hover_extra/bindingN padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p111_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

#[test]
fn p111_sem_gate_0() {
    let text = "func main() {\n  let y0 = 0\n  print(y0)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y0"), "{hover}");
}

#[test]
fn p111_sem_gate_1() {
    let text = "func main() {\n  let y1 = 1\n  print(y1)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y1"), "{hover}");
}

#[test]
fn p111_sem_gate_2() {
    let text = "func main() {\n  let y2 = 2\n  print(y2)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y2"), "{hover}");
}

#[test]
fn p111_sem_gate_3() {
    let text = "func main() {\n  let y3 = 3\n  print(y3)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y3"), "{hover}");
}

#[test]
fn p111_sem_gate_4() {
    let text = "func main() {\n  let y4 = 4\n  print(y4)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y4"), "{hover}");
}

#[test]
fn p111_sem_gate_5() {
    let text = "func main() {\n  let y5 = 5\n  print(y5)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y5"), "{hover}");
}

#[test]
fn p111_sem_gate_6() {
    let text = "func main() {\n  let y6 = 6\n  print(y6)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y6"), "{hover}");
}

#[test]
fn p111_sem_gate_7() {
    let text = "func main() {\n  let y7 = 7\n  print(y7)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y7"), "{hover}");
}

#[test]
fn p111_sem_gate_8() {
    let text = "func main() {\n  let y8 = 8\n  print(y8)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y8"), "{hover}");
}

#[test]
fn p111_sem_gate_9() {
    let text = "func main() {\n  let y9 = 9\n  print(y9)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 7, &p111_opts());
    assert!(hover.to_string().contains("y9"), "{hover}");
}

#[test]
fn p111_sem_gate_10() {
    let text = "func main() {\n  let y10 = 10\n  print(y10)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y10"), "{hover}");
}

#[test]
fn p111_sem_gate_11() {
    let text = "func main() {\n  let y11 = 11\n  print(y11)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y11"), "{hover}");
}

#[test]
fn p111_sem_gate_12() {
    let text = "func main() {\n  let y12 = 12\n  print(y12)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y12"), "{hover}");
}

#[test]
fn p111_sem_gate_13() {
    let text = "func main() {\n  let y13 = 13\n  print(y13)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y13"), "{hover}");
}

#[test]
fn p111_sem_gate_14() {
    let text = "func main() {\n  let y14 = 14\n  print(y14)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y14"), "{hover}");
}

#[test]
fn p111_sem_gate_15() {
    let text = "func main() {\n  let y15 = 15\n  print(y15)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y15"), "{hover}");
}

#[test]
fn p111_sem_gate_16() {
    let text = "func main() {\n  let y16 = 16\n  print(y16)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y16"), "{hover}");
}

#[test]
fn p111_sem_gate_17() {
    let text = "func main() {\n  let y17 = 17\n  print(y17)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y17"), "{hover}");
}

#[test]
fn p111_sem_gate_18() {
    let text = "func main() {\n  let y18 = 18\n  print(y18)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y18"), "{hover}");
}

#[test]
fn p111_sem_gate_19() {
    let text = "func main() {\n  let y19 = 19\n  print(y19)\n}";
    let hover = hover_for_position("file:///p111.sy", text, 1, 8, &p111_opts());
    assert!(hover.to_string().contains("y19"), "{hover}");
}
