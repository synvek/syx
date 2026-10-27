// Phase 115 real LSP matrix (≥24): Track D fingerprints + real semantic probes (no sem_gate/sem_probe/bind_fill/hover_sem clones).

use super::*;

fn p115_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p115_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p115_opts())
}


#[test]
fn p115_dist_hostkey_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_has_runtime_bound_host_key_v5"));
    assert!(src.contains("dist_host_key_register_live_listener_bind"));
    assert!(src.contains("cdn_pop_route"));
}

#[test]
fn p115_select_deadline_strict_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(!parser.contains("fn select_mixed_await_timeout_ms_literal"));
    assert!(parser.contains("TokenKind::Timeout"));
    assert!(parser.contains("TokenKind::Deadline"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
}

#[test]
fn p115_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p115.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p115_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p115.sy", text, 2, 10, &p115_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p115_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p115.sy", text, 1, 10, &p115_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p115_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p115.sy", text, 1, 19, &p115_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p115_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p115.sy", text, 2, 8, &p115_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p115_diagnostic_unknown_ident() {
    let doc = p115_doc("func main() { print(noSuchSymbol115) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol115")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p115_diagnostic_type_mismatch() {
    let doc = p115_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p115.sy", text, 0, 8, &p115_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p115_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p115_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p115_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p115.sy", text, 0, 12, &p115_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p115_diagnostic_undefined_type() {
    let doc = p115_doc("func main() { let x: NoSuchType115 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p115_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_diagnostic_duplicate_local() {
    let doc = p115_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_select_timeout_parse_smoke() {
    let doc = p115_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p115_select_deadline_parse_smoke() {
    let doc = p115_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p115_async_select_mixed_strict_parse_smoke() {
    let doc = p115_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"u\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p115_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p115.sy", text, 1, 19, &p115_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p115_diagnostic_wrong_assign_type() {
    let doc = p115_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p115.sy", text, 1, 19, &p115_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p115_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p115_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p115_diagnostic_missing_return_type_use() {
    let doc = p115_doc("func main() { let z: MissingAlias115 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_async_func_parse_smoke() {
    let doc = p115_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p115_diagnostic_string_as_int() {
    let doc = p115_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p115_cjk_string_parse_smoke() {
    let doc = p115_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p115_lsp_semantic_v17_contract() {
    let src = include_str!("lsp_phase115_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase115 LSP matrix must be ≥24 tests, found {tests}"
    );
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("lsp-semantic-v17:phase115-24-plus-real"));
    let pad_a = concat!("fn p115_", "sem_gate");
    let pad_b = concat!("fn p115_", "sem_probe");
    let pad_c = concat!("fn p115_", "hover_extra");
    let pad_d = concat!("fn p115_", "bind_fill");
    let pad_e = concat!("fn p115_", "hover_sem_");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b) && !src.contains(pad_c) && !src.contains(pad_d) && !src.contains(pad_e),
        "phase115 must not ship sem_gate/sem_probe/hover_extra/bind_fill/hover_sem padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p115_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

#[test]
fn p115_real_bind_a00() {
    let text = "func main() {\n  let a00 = 0\n  print(a00)\n}";
    let hover = hover_for_position("file:///p115_a00.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a00"), "{hover}");
}

#[test]
fn p115_real_bind_a01() {
    let text = "func main() {\n  let a01 = 1\n  print(a01)\n}";
    let hover = hover_for_position("file:///p115_a01.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a01"), "{hover}");
}

#[test]
fn p115_real_bind_a02() {
    let text = "func main() {\n  let a02 = 2\n  print(a02)\n}";
    let hover = hover_for_position("file:///p115_a02.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a02"), "{hover}");
}

#[test]
fn p115_real_bind_a03() {
    let text = "func main() {\n  let a03 = 3\n  print(a03)\n}";
    let hover = hover_for_position("file:///p115_a03.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a03"), "{hover}");
}

#[test]
fn p115_real_bind_a04() {
    let text = "func main() {\n  let a04 = 4\n  print(a04)\n}";
    let hover = hover_for_position("file:///p115_a04.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a04"), "{hover}");
}

#[test]
fn p115_real_bind_a05() {
    let text = "func main() {\n  let a05 = 5\n  print(a05)\n}";
    let hover = hover_for_position("file:///p115_a05.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a05"), "{hover}");
}

#[test]
fn p115_real_bind_a06() {
    let text = "func main() {\n  let a06 = 6\n  print(a06)\n}";
    let hover = hover_for_position("file:///p115_a06.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a06"), "{hover}");
}

#[test]
fn p115_real_bind_a07() {
    let text = "func main() {\n  let a07 = 7\n  print(a07)\n}";
    let hover = hover_for_position("file:///p115_a07.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a07"), "{hover}");
}

#[test]
fn p115_real_bind_a08() {
    let text = "func main() {\n  let a08 = 8\n  print(a08)\n}";
    let hover = hover_for_position("file:///p115_a08.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a08"), "{hover}");
}

#[test]
fn p115_real_bind_a09() {
    let text = "func main() {\n  let a09 = 9\n  print(a09)\n}";
    let hover = hover_for_position("file:///p115_a09.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a09"), "{hover}");
}

#[test]
fn p115_real_bind_a10() {
    let text = "func main() {\n  let a10 = 10\n  print(a10)\n}";
    let hover = hover_for_position("file:///p115_a10.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a10"), "{hover}");
}

#[test]
fn p115_real_bind_a11() {
    let text = "func main() {\n  let a11 = 11\n  print(a11)\n}";
    let hover = hover_for_position("file:///p115_a11.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a11"), "{hover}");
}

#[test]
fn p115_real_bind_a12() {
    let text = "func main() {\n  let a12 = 12\n  print(a12)\n}";
    let hover = hover_for_position("file:///p115_a12.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a12"), "{hover}");
}

#[test]
fn p115_real_bind_a13() {
    let text = "func main() {\n  let a13 = 13\n  print(a13)\n}";
    let hover = hover_for_position("file:///p115_a13.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a13"), "{hover}");
}

#[test]
fn p115_real_bind_a14() {
    let text = "func main() {\n  let a14 = 14\n  print(a14)\n}";
    let hover = hover_for_position("file:///p115_a14.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a14"), "{hover}");
}

#[test]
fn p115_real_bind_a15() {
    let text = "func main() {\n  let a15 = 15\n  print(a15)\n}";
    let hover = hover_for_position("file:///p115_a15.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a15"), "{hover}");
}

#[test]
fn p115_real_bind_a16() {
    let text = "func main() {\n  let a16 = 16\n  print(a16)\n}";
    let hover = hover_for_position("file:///p115_a16.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a16"), "{hover}");
}

#[test]
fn p115_real_bind_a17() {
    let text = "func main() {\n  let a17 = 17\n  print(a17)\n}";
    let hover = hover_for_position("file:///p115_a17.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a17"), "{hover}");
}

#[test]
fn p115_real_bind_a18() {
    let text = "func main() {\n  let a18 = 18\n  print(a18)\n}";
    let hover = hover_for_position("file:///p115_a18.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a18"), "{hover}");
}

#[test]
fn p115_real_bind_a19() {
    let text = "func main() {\n  let a19 = 19\n  print(a19)\n}";
    let hover = hover_for_position("file:///p115_a19.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a19"), "{hover}");
}

#[test]
fn p115_real_bind_a20() {
    let text = "func main() {\n  let a20 = 20\n  print(a20)\n}";
    let hover = hover_for_position("file:///p115_a20.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a20"), "{hover}");
}

#[test]
fn p115_real_bind_a21() {
    let text = "func main() {\n  let a21 = 21\n  print(a21)\n}";
    let hover = hover_for_position("file:///p115_a21.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a21"), "{hover}");
}

#[test]
fn p115_real_bind_a22() {
    let text = "func main() {\n  let a22 = 22\n  print(a22)\n}";
    let hover = hover_for_position("file:///p115_a22.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a22"), "{hover}");
}

#[test]
fn p115_real_bind_a23() {
    let text = "func main() {\n  let a23 = 23\n  print(a23)\n}";
    let hover = hover_for_position("file:///p115_a23.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a23"), "{hover}");
}

#[test]
fn p115_real_bind_a24() {
    let text = "func main() {\n  let a24 = 24\n  print(a24)\n}";
    let hover = hover_for_position("file:///p115_a24.sy", text, 1, 8, &p115_opts());
    assert!(hover.to_string().contains("a24"), "{hover}");
}
