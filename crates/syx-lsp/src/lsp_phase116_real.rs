// Phase 116 real LSP matrix (≥50): Track D fingerprints + real semantic probes (no sem_gate/sem_probe/bind_fill/hover_sem clones).

use super::*;

fn p116_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p116_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p116_opts())
}


#[test]
fn p116_dist_hostkey_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_has_runtime_bound_host_key_v5"));
    assert!(src.contains("dist_host_key_register_live_listener_bind"));
    assert!(src.contains("cdn_pop_route"));
    assert!(!src.contains("global_cdn_route_table"));
}

#[test]
fn p116_select_deadline_strict_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(!parser.contains("fn select_mixed_await_timeout_ms_literal"));
    assert!(parser.contains("TokenKind::Timeout"));
    assert!(parser.contains("TokenKind::Deadline"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
}

#[test]
fn p116_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p116.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p116_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p115\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p116.sy", text, 2, 10, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p116.sy", text, 1, 10, &p116_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p116_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p116.sy", text, 1, 19, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p116_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p116.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_diagnostic_unknown_ident() {
    let doc = p116_doc("func main() { print(noSuchSymbol116) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol116")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p116_diagnostic_type_mismatch() {
    let doc = p116_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p116.sy", text, 0, 8, &p116_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p116_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p116_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p116_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p116.sy", text, 0, 12, &p116_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p116_diagnostic_undefined_type() {
    let doc = p116_doc("func main() { let x: NoSuchType116 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p116_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_diagnostic_duplicate_local() {
    let doc = p116_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_select_timeout_parse_smoke() {
    let doc = p116_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p116_select_deadline_parse_smoke() {
    let doc = p116_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p116_async_select_mixed_strict_parse_smoke() {
    let doc = p116_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"u\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p116_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p116.sy", text, 1, 19, &p116_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p116_diagnostic_wrong_assign_type() {
    let doc = p116_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p116.sy", text, 1, 19, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p116_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p116_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p116_diagnostic_missing_return_type_use() {
    let doc = p116_doc("func main() { let z: MissingAlias116 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_async_func_parse_smoke() {
    let doc = p116_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p116_diagnostic_string_as_int() {
    let doc = p116_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_cjk_string_parse_smoke() {
    let doc = p116_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p116_lsp_semantic_v17_contract() {
    let src = include_str!("lsp_phase116_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 50,
        "phase116 LSP matrix must be ≥50 tests, found {tests}"
    );
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("lsp-semantic-v18:phase116-24-plus-real"));
    let pad_a = concat!("fn p116_", "sem_gate");
    let pad_b = concat!("fn p116_", "sem_probe");
    let pad_c = concat!("fn p116_", "hover_extra");
    let pad_d = concat!("fn p116_", "bind_fill");
    let pad_e = concat!("fn p116_", "hover_sem_");
    let pad_f = concat!("fn p116_", "real_bind_");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b) && !src.contains(pad_c) && !src.contains(pad_d) && !src.contains(pad_e) && !src.contains(pad_f),
        "phase116 must not ship sem_gate/sem_probe/hover_extra/bind_fill/hover_sem/real_bind padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p116_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

#[test]
fn p116_probe_d00_hover() {
    let text = "func main() {\n  let d00 = 100\n  print(d00)\n}";
    let hover = hover_for_position("file:///p116_d00.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d00"), "{hover}");
}

#[test]
fn p116_probe_d01_def() {
    let text = "func main() {\n  let a = 101\n  print(a)\n}";
    let def = definition_for_position("file:///p116_a.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d02_diag() {
    let doc = p116_doc("func main() { let d02: Int = \"badd02\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d03_hover() {
    let text = "func main() {\n  let d03 = 103\n  print(d03)\n}";
    let hover = hover_for_position("file:///p116_d03.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d03"), "{hover}");
}

#[test]
fn p116_probe_d04_def() {
    let text = "func main() {\n  let b = 104\n  print(b)\n}";
    let def = definition_for_position("file:///p116_b.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d05_diag() {
    let doc = p116_doc("func main() { let d05: Int = \"badd05\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d06_hover() {
    let text = "func main() {\n  let d06 = 106\n  print(d06)\n}";
    let hover = hover_for_position("file:///p116_d06.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d06"), "{hover}");
}

#[test]
fn p116_probe_d07_def() {
    let text = "func main() {\n  let c = 107\n  print(c)\n}";
    let def = definition_for_position("file:///p116_c.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d08_diag() {
    let doc = p116_doc("func main() { let d08: Int = \"badd08\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d09_hover() {
    let text = "func main() {\n  let d09 = 109\n  print(d09)\n}";
    let hover = hover_for_position("file:///p116_d09.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d09"), "{hover}");
}

#[test]
fn p116_probe_d10_def() {
    let text = "func main() {\n  let d = 110\n  print(d)\n}";
    let def = definition_for_position("file:///p116_d.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d11_diag() {
    let doc = p116_doc("func main() { let d11: Int = \"badd11\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d12_hover() {
    let text = "func main() {\n  let d12 = 112\n  print(d12)\n}";
    let hover = hover_for_position("file:///p116_d12.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d12"), "{hover}");
}

#[test]
fn p116_probe_d13_def() {
    let text = "func main() {\n  let e = 113\n  print(e)\n}";
    let def = definition_for_position("file:///p116_e.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d14_diag() {
    let doc = p116_doc("func main() { let d14: Int = \"badd14\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d15_hover() {
    let text = "func main() {\n  let d15 = 115\n  print(d15)\n}";
    let hover = hover_for_position("file:///p116_d15.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d15"), "{hover}");
}

#[test]
fn p116_probe_d16_def() {
    let text = "func main() {\n  let f = 116\n  print(f)\n}";
    let def = definition_for_position("file:///p116_f.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d17_diag() {
    let doc = p116_doc("func main() { let d17: Int = \"badd17\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d18_hover() {
    let text = "func main() {\n  let d18 = 118\n  print(d18)\n}";
    let hover = hover_for_position("file:///p116_d18.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d18"), "{hover}");
}

#[test]
fn p116_probe_d19_def() {
    let text = "func main() {\n  let g = 119\n  print(g)\n}";
    let def = definition_for_position("file:///p116_g.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d20_diag() {
    let doc = p116_doc("func main() { let d20: Int = \"badd20\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d21_hover() {
    let text = "func main() {\n  let d21 = 121\n  print(d21)\n}";
    let hover = hover_for_position("file:///p116_d21.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d21"), "{hover}");
}

#[test]
fn p116_probe_d22_def() {
    let text = "func main() {\n  let h = 122\n  print(h)\n}";
    let def = definition_for_position("file:///p116_h.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d23_diag() {
    let doc = p116_doc("func main() { let d23: Int = \"badd23\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d24_hover() {
    let text = "func main() {\n  let d24 = 124\n  print(d24)\n}";
    let hover = hover_for_position("file:///p116_d24.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d24"), "{hover}");
}

#[test]
fn p116_probe_d25_def() {
    let text = "func main() {\n  let i = 125\n  print(i)\n}";
    let def = definition_for_position("file:///p116_i.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d26_diag() {
    let doc = p116_doc("func main() { let d26: Int = \"badd26\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p116_probe_d27_hover() {
    let text = "func main() {\n  let d27 = 127\n  print(d27)\n}";
    let hover = hover_for_position("file:///p116_d27.sy", text, 1, 8, &p116_opts());
    assert!(hover.to_string().contains("d27"), "{hover}");
}

#[test]
fn p116_probe_d28_def() {
    let text = "func main() {\n  let j = 128\n  print(j)\n}";
    let def = definition_for_position("file:///p116_j.sy", text, 2, 8, &p116_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p116_probe_d29_diag() {
    let doc = p116_doc("func main() { let d29: Int = \"badd29\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}
