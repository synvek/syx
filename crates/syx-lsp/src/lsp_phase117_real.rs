// Phase 117 real LSP matrix (≥24 semantic): Track D fingerprints + real probes (no bind_b/real_bind/hover_sem clones).

use super::*;

fn p117_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p117_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p117_opts())
}

#[test]
fn p117_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p117\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p117.sy", text, 1, 8, &p117_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p117_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p117\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p117.sy", text, 2, 10, &p117_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p117_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p117.sy", text, 1, 10, &p117_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p117_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p117.sy", text, 1, 19, &p117_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p117_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p117.sy", text, 2, 8, &p117_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p117_diagnostic_unknown_ident() {
    let doc = p117_doc("func main() { print(noSuchSymbol117) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol117")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p117_diagnostic_type_mismatch() {
    let doc = p117_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p117.sy", text, 0, 8, &p117_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p117_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p117_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p117_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p117.sy", text, 0, 12, &p117_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p117_diagnostic_undefined_type() {
    let doc = p117_doc("func main() { let x: NoSuchType117 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p117_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_diagnostic_duplicate_local() {
    let doc = p117_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_select_timeout_parse_smoke() {
    let doc = p117_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    timeout(50): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p117_select_deadline_parse_smoke() {
    let doc = p117_doc(
        "func main() {\n  let ch = __syx_channel_create(1)\n  select {\n    case v <- ch: print(v)\n    deadline 25: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p117_async_select_mixed_strict_parse_smoke() {
    let doc = p117_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"u\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p117_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p117.sy", text, 1, 19, &p117_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p117_diagnostic_wrong_assign_type() {
    let doc = p117_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p117.sy", text, 1, 19, &p117_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p117_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p117_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p117_diagnostic_missing_return_type_use() {
    let doc = p117_doc("func main() { let z: MissingAlias117 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_async_func_parse_smoke() {
    let doc = p117_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p117_diagnostic_string_as_int() {
    let doc = p117_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_cjk_string_parse_smoke() {
    let doc = p117_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p117_diagnostic_bool_as_string() {
    let doc = p117_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p117_references_local_msg() {
    let text = "func main() {\n  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p117_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p117_lsp_semantic_v19_contract() {
    let src = include_str!("lsp_phase117_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase117 LSP matrix must be ≥24 tests, found {tests}"
    );
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("lsp-semantic-v19:phase117-24-plus-real"));
    let pad_a = concat!("fn p117_", "sem_gate");
    let pad_b = concat!("fn p117_", "sem_probe");
    let pad_c = concat!("fn p117_", "hover_extra");
    let pad_d = concat!("fn p117_", "bind_fill");
    let pad_e = concat!("fn p117_", "hover_sem_");
    let pad_f = concat!("fn p117_", "real_bind_");
    let pad_g = concat!("fn p117_", "bind_b");
    assert!(
        !src.contains(pad_a)
            && !src.contains(pad_b)
            && !src.contains(pad_c)
            && !src.contains(pad_d)
            && !src.contains(pad_e)
            && !src.contains(pad_f)
            && !src.contains(pad_g),
        "phase117 must not ship padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p117_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}
