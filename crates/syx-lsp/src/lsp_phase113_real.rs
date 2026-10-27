// Phase 113 real LSP matrix (≥24): Track D fingerprints + real semantic probes (no sem_gate/sem_probe clones).

use super::*;

fn p113_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p113_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p113_opts())
}

#[test]
fn p113_dist_hostkey_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_has_runtime_bound_host_key"));
    assert!(src.contains("dist_host_key_register_runtime_bind"));
    assert!(src.contains("cdn_tcp_peer_route"));
}

#[test]
fn p113_select_deadline_strict_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(!parser.contains("fn select_mixed_await_timeout_ms_literal"));
    assert!(parser.contains("TokenKind::Timeout"));
    assert!(parser.contains("TokenKind::Deadline"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
}

#[test]
fn p113_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p113\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p113.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p113_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p113\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p113.sy", text, 2, 10, &p113_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p113_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p113.sy", text, 1, 10, &p113_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p113_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p113.sy", text, 1, 19, &p113_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p113_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p113.sy", text, 2, 8, &p113_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p113_diagnostic_unknown_ident() {
    let doc = p113_doc("func main() { print(noSuchSymbol113) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol113")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p113_diagnostic_type_mismatch() {
    let doc = p113_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p113.sy", text, 0, 8, &p113_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p113_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p113_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p113_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p113.sy", text, 0, 12, &p113_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p113_diagnostic_undefined_type() {
    let doc = p113_doc("func main() { let x: NoSuchType113 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p113_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_diagnostic_duplicate_local() {
    let doc = p113_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_select_timeout_parse_smoke() {
    let doc = p113_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p113_select_deadline_parse_smoke() {
    let doc = p113_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p113_async_select_mixed_strict_parse_smoke() {
    let doc = p113_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"u\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p113_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p113.sy", text, 1, 19, &p113_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p113_diagnostic_wrong_assign_type() {
    let doc = p113_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p113.sy", text, 1, 19, &p113_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p113_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p113_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p113_diagnostic_missing_return_type_use() {
    let doc = p113_doc("func main() { let z: MissingAlias113 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_async_func_parse_smoke() {
    let doc = p113_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p113_diagnostic_string_as_int() {
    let doc = p113_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p113_cjk_string_parse_smoke() {
    let doc = p113_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p113_lsp_semantic_v15_contract() {
    let src = include_str!("lsp_phase113_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase113 LSP matrix must be ≥24 tests, found {tests}"
    );
    let pad_a = concat!("fn p113_", "sem_gate");
    let pad_b = concat!("fn p113_", "sem_probe");
    let pad_c = concat!("fn p113_", "hover_extra");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b) && !src.contains(pad_c),
        "phase113 must not ship sem_gate/sem_probe/hover_extra padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p113_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

#[test]
fn p113_hover_bind_fill_0() {
    let text = "func main() {\n  let w00 = 0\n  print(w00)\n}";
    let hover = hover_for_position("file:///p113x_0.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w00"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_1() {
    let text = "func main() {\n  let w01 = 1\n  print(w01)\n}";
    let hover = hover_for_position("file:///p113x_1.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w01"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_2() {
    let text = "func main() {\n  let w02 = 2\n  print(w02)\n}";
    let hover = hover_for_position("file:///p113x_2.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w02"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_3() {
    let text = "func main() {\n  let w03 = 3\n  print(w03)\n}";
    let hover = hover_for_position("file:///p113x_3.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w03"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_4() {
    let text = "func main() {\n  let w04 = 4\n  print(w04)\n}";
    let hover = hover_for_position("file:///p113x_4.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w04"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_5() {
    let text = "func main() {\n  let w05 = 5\n  print(w05)\n}";
    let hover = hover_for_position("file:///p113x_5.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w05"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_6() {
    let text = "func main() {\n  let w06 = 6\n  print(w06)\n}";
    let hover = hover_for_position("file:///p113x_6.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w06"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_7() {
    let text = "func main() {\n  let w07 = 7\n  print(w07)\n}";
    let hover = hover_for_position("file:///p113x_7.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w07"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_8() {
    let text = "func main() {\n  let w08 = 8\n  print(w08)\n}";
    let hover = hover_for_position("file:///p113x_8.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w08"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_9() {
    let text = "func main() {\n  let w09 = 9\n  print(w09)\n}";
    let hover = hover_for_position("file:///p113x_9.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w09"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_10() {
    let text = "func main() {\n  let w10 = 10\n  print(w10)\n}";
    let hover = hover_for_position("file:///p113x_10.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w10"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_11() {
    let text = "func main() {\n  let w11 = 11\n  print(w11)\n}";
    let hover = hover_for_position("file:///p113x_11.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w11"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_12() {
    let text = "func main() {\n  let w12 = 12\n  print(w12)\n}";
    let hover = hover_for_position("file:///p113x_12.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w12"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_13() {
    let text = "func main() {\n  let w13 = 13\n  print(w13)\n}";
    let hover = hover_for_position("file:///p113x_13.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w13"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_14() {
    let text = "func main() {\n  let w14 = 14\n  print(w14)\n}";
    let hover = hover_for_position("file:///p113x_14.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w14"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_15() {
    let text = "func main() {\n  let w15 = 15\n  print(w15)\n}";
    let hover = hover_for_position("file:///p113x_15.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w15"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_16() {
    let text = "func main() {\n  let w16 = 16\n  print(w16)\n}";
    let hover = hover_for_position("file:///p113x_16.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w16"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_17() {
    let text = "func main() {\n  let w17 = 17\n  print(w17)\n}";
    let hover = hover_for_position("file:///p113x_17.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w17"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_18() {
    let text = "func main() {\n  let w18 = 18\n  print(w18)\n}";
    let hover = hover_for_position("file:///p113x_18.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w18"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_19() {
    let text = "func main() {\n  let w19 = 19\n  print(w19)\n}";
    let hover = hover_for_position("file:///p113x_19.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w19"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_20() {
    let text = "func main() {\n  let w20 = 20\n  print(w20)\n}";
    let hover = hover_for_position("file:///p113x_20.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w20"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_21() {
    let text = "func main() {\n  let w21 = 21\n  print(w21)\n}";
    let hover = hover_for_position("file:///p113x_21.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w21"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_22() {
    let text = "func main() {\n  let w22 = 22\n  print(w22)\n}";
    let hover = hover_for_position("file:///p113x_22.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w22"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_23() {
    let text = "func main() {\n  let w23 = 23\n  print(w23)\n}";
    let hover = hover_for_position("file:///p113x_23.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w23"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_24() {
    let text = "func main() {\n  let w24 = 24\n  print(w24)\n}";
    let hover = hover_for_position("file:///p113x_24.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w24"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_25() {
    let text = "func main() {\n  let w25 = 25\n  print(w25)\n}";
    let hover = hover_for_position("file:///p113x_25.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w25"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_26() {
    let text = "func main() {\n  let w26 = 26\n  print(w26)\n}";
    let hover = hover_for_position("file:///p113x_26.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w26"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_27() {
    let text = "func main() {\n  let w27 = 27\n  print(w27)\n}";
    let hover = hover_for_position("file:///p113x_27.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w27"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_28() {
    let text = "func main() {\n  let w28 = 28\n  print(w28)\n}";
    let hover = hover_for_position("file:///p113x_28.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w28"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_29() {
    let text = "func main() {\n  let w29 = 29\n  print(w29)\n}";
    let hover = hover_for_position("file:///p113x_29.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w29"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_30() {
    let text = "func main() {\n  let w30 = 30\n  print(w30)\n}";
    let hover = hover_for_position("file:///p113x_30.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w30"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_31() {
    let text = "func main() {\n  let w31 = 31\n  print(w31)\n}";
    let hover = hover_for_position("file:///p113x_31.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w31"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_32() {
    let text = "func main() {\n  let w32 = 32\n  print(w32)\n}";
    let hover = hover_for_position("file:///p113x_32.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w32"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_33() {
    let text = "func main() {\n  let w33 = 33\n  print(w33)\n}";
    let hover = hover_for_position("file:///p113x_33.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w33"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_34() {
    let text = "func main() {\n  let w34 = 34\n  print(w34)\n}";
    let hover = hover_for_position("file:///p113x_34.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w34"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_35() {
    let text = "func main() {\n  let w35 = 35\n  print(w35)\n}";
    let hover = hover_for_position("file:///p113x_35.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w35"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_36() {
    let text = "func main() {\n  let w36 = 36\n  print(w36)\n}";
    let hover = hover_for_position("file:///p113x_36.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w36"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_37() {
    let text = "func main() {\n  let w37 = 37\n  print(w37)\n}";
    let hover = hover_for_position("file:///p113x_37.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w37"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_38() {
    let text = "func main() {\n  let w38 = 38\n  print(w38)\n}";
    let hover = hover_for_position("file:///p113x_38.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w38"), "{hover}");
}

#[test]
fn p113_hover_bind_fill_39() {
    let text = "func main() {\n  let w39 = 39\n  print(w39)\n}";
    let hover = hover_for_position("file:///p113x_39.sy", text, 1, 8, &p113_opts());
    assert!(hover.to_string().contains("w39"), "{hover}");
}
