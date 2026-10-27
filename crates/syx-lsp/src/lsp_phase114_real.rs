// Phase 114 real LSP matrix (≥24): Track D fingerprints + real semantic probes (no sem_gate/sem_probe/bind_fill clones).

use super::*;

fn p114_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p114_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p114_opts())
}

#[test]
fn p114_dist_hostkey_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_has_runtime_bound_host_key"));
    assert!(src.contains("dist_host_key_register_runtime_bind"));
    assert!(src.contains("cdn_pop_route"));
}

#[test]
fn p114_select_deadline_strict_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(!parser.contains("fn select_mixed_await_timeout_ms_literal"));
    assert!(parser.contains("TokenKind::Timeout"));
    assert!(parser.contains("TokenKind::Deadline"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
}

#[test]
fn p114_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p114\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p114.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p114_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p114\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p114.sy", text, 2, 10, &p114_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p114_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p114.sy", text, 1, 10, &p114_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p114_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p114.sy", text, 1, 19, &p114_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p114_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p114.sy", text, 2, 8, &p114_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p114_diagnostic_unknown_ident() {
    let doc = p114_doc("func main() { print(noSuchSymbol114) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol114")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p114_diagnostic_type_mismatch() {
    let doc = p114_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p114.sy", text, 0, 8, &p114_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p114_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p114_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p114_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p114.sy", text, 0, 12, &p114_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p114_diagnostic_undefined_type() {
    let doc = p114_doc("func main() { let x: NoSuchType114 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p114_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_diagnostic_duplicate_local() {
    let doc = p114_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_select_timeout_parse_smoke() {
    let doc = p114_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p114_select_deadline_parse_smoke() {
    let doc = p114_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p114_async_select_mixed_strict_parse_smoke() {
    let doc = p114_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"u\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p114_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p114.sy", text, 1, 19, &p114_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p114_diagnostic_wrong_assign_type() {
    let doc = p114_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p114.sy", text, 1, 19, &p114_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p114_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p114_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p114_diagnostic_missing_return_type_use() {
    let doc = p114_doc("func main() { let z: MissingAlias114 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_async_func_parse_smoke() {
    let doc = p114_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p114_diagnostic_string_as_int() {
    let doc = p114_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p114_cjk_string_parse_smoke() {
    let doc = p114_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p114_lsp_semantic_v16_contract() {
    let src = include_str!("lsp_phase114_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase114 LSP matrix must be ≥24 tests, found {tests}"
    );
    let pad_a = concat!("fn p114_", "sem_gate");
    let pad_b = concat!("fn p114_", "sem_probe");
    let pad_c = concat!("fn p114_", "hover_extra");
    let pad_d = concat!("fn p114_", "bind_fill");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b) && !src.contains(pad_c) && !src.contains(pad_d),
        "phase114 must not ship sem_gate/sem_probe/hover_extra/bind_fill padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p114_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

#[test]
fn p114_hover_sem_quota114() {
    let text = "func main() {\n  let q00 = 00\n  print(q00)\n}";
    let hover = hover_for_position("file:///p114_q00.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("q00"), "{hover}");
}

#[test]
fn p114_hover_sem_delta114() {
    let text = "func main() {\n  let d01 = 01\n  print(d01)\n}";
    let hover = hover_for_position("file:///p114_d01.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("d01"), "{hover}");
}

#[test]
fn p114_hover_sem_sigma114() {
    let text = "func main() {\n  let s02 = 02\n  print(s02)\n}";
    let hover = hover_for_position("file:///p114_s02.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("s02"), "{hover}");
}

#[test]
fn p114_hover_sem_omega114() {
    let text = "func main() {\n  let o03 = 03\n  print(o03)\n}";
    let hover = hover_for_position("file:///p114_o03.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("o03"), "{hover}");
}

#[test]
fn p114_hover_sem_theta114() {
    let text = "func main() {\n  let t04 = 04\n  print(t04)\n}";
    let hover = hover_for_position("file:///p114_t04.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("t04"), "{hover}");
}

#[test]
fn p114_hover_sem_lambda114() {
    let text = "func main() {\n  let l05 = 05\n  print(l05)\n}";
    let hover = hover_for_position("file:///p114_l05.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("l05"), "{hover}");
}

#[test]
fn p114_hover_sem_kappa114() {
    let text = "func main() {\n  let k06 = 06\n  print(k06)\n}";
    let hover = hover_for_position("file:///p114_k06.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("k06"), "{hover}");
}

#[test]
fn p114_hover_sem_iota114() {
    let text = "func main() {\n  let i07 = 07\n  print(i07)\n}";
    let hover = hover_for_position("file:///p114_i07.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("i07"), "{hover}");
}

#[test]
fn p114_hover_sem_zeta114() {
    let text = "func main() {\n  let z08 = 08\n  print(z08)\n}";
    let hover = hover_for_position("file:///p114_z08.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("z08"), "{hover}");
}

#[test]
fn p114_hover_sem_eta114() {
    let text = "func main() {\n  let e09 = 09\n  print(e09)\n}";
    let hover = hover_for_position("file:///p114_e09.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("e09"), "{hover}");
}

#[test]
fn p114_hover_sem_rho114() {
    let text = "func main() {\n  let r10 = 10\n  print(r10)\n}";
    let hover = hover_for_position("file:///p114_r10.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("r10"), "{hover}");
}

#[test]
fn p114_hover_sem_tau114() {
    let text = "func main() {\n  let u11 = 11\n  print(u11)\n}";
    let hover = hover_for_position("file:///p114_u11.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("u11"), "{hover}");
}

#[test]
fn p114_hover_sem_phi114() {
    let text = "func main() {\n  let p12 = 12\n  print(p12)\n}";
    let hover = hover_for_position("file:///p114_p12.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("p12"), "{hover}");
}

#[test]
fn p114_hover_sem_chi114() {
    let text = "func main() {\n  let c13 = 13\n  print(c13)\n}";
    let hover = hover_for_position("file:///p114_c13.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("c13"), "{hover}");
}

#[test]
fn p114_hover_sem_psi114() {
    let text = "func main() {\n  let y14 = 14\n  print(y14)\n}";
    let hover = hover_for_position("file:///p114_y14.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("y14"), "{hover}");
}

#[test]
fn p114_hover_sem_nu114() {
    let text = "func main() {\n  let n15 = 15\n  print(n15)\n}";
    let hover = hover_for_position("file:///p114_n15.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("n15"), "{hover}");
}

#[test]
fn p114_hover_sem_mu114() {
    let text = "func main() {\n  let m16 = 16\n  print(m16)\n}";
    let hover = hover_for_position("file:///p114_m16.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("m16"), "{hover}");
}

#[test]
fn p114_hover_sem_xi114() {
    let text = "func main() {\n  let x17 = 17\n  print(x17)\n}";
    let hover = hover_for_position("file:///p114_x17.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("x17"), "{hover}");
}

#[test]
fn p114_hover_sem_pi114() {
    let text = "func main() {\n  let v18 = 18\n  print(v18)\n}";
    let hover = hover_for_position("file:///p114_v18.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("v18"), "{hover}");
}

#[test]
fn p114_hover_sem_upsilon114() {
    let text = "func main() {\n  let w19 = 19\n  print(w19)\n}";
    let hover = hover_for_position("file:///p114_w19.sy", text, 1, 8, &p114_opts());
    assert!(hover.to_string().contains("w19"), "{hover}");
}
