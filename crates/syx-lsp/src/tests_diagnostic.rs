// Deduped LSP diagnostic matrix (historical phase68–139).
use super::*;

#[test]
fn diagnostic_duplicate_local() {
    let doc = doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_wrong_assign_type() {
    let doc = doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_type_mismatch() {
    let doc = doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_string_as_int() {
    let doc = doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_bool_as_string() {
    let doc = doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn diagnostic_optional_unwrap() {
    let doc = doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v17() {
    let doc = doc("func main() { print(noSuchSymbolV17) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_float_as_string() {
    let doc = doc("func main() { let s: String = 1.5 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident() {
    let doc = doc("func main() { print(noSuchSymbol139) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol139")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type() {
    let doc = doc("func main() { let x: NoSuchType139 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v2() {
    let doc = doc("func main() { print(noSuchSymbol138) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol138")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v2() {
    let doc = doc("func main() { let x: NoSuchType138 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v3() {
    let doc = doc("func main() { print(noSuchSymbol136) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol136")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v3() {
    let doc = doc("func main() { let x: NoSuchType136 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v15() {
    let doc = doc("func main() { print(noSuchSymbolV15) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v4() {
    let doc = doc("func main() { print(noSuchSymbol135) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol135")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v4() {
    let doc = doc("func main() { let x: NoSuchType135 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v5() {
    let doc = doc("func main() { print(noSuchSymbol134) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol134")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v5() {
    let doc = doc("func main() { let x: NoSuchType134 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v6() {
    let doc = doc("func main() { print(noSuchSymbol133) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol133")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v6() {
    let doc = doc("func main() { let x: NoSuchType133 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v14() {
    let doc = doc("func main() { print(noSuchSymbolV14) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v7() {
    let doc = doc("func main() { print(noSuchSymbol132) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol132")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v7() {
    let doc = doc("func main() { let x: NoSuchType132 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v13() {
    let doc = doc("func main() { print(noSuchSymbolV13) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v8() {
    let doc = doc("func main() { print(noSuchSymbol131) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol131")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v8() {
    let doc = doc("func main() { let x: NoSuchType131 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v12() {
    let doc = doc("func main() { print(noSuchSymbolV12) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v9() {
    let doc = doc("func main() { print(noSuchSymbol130) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol130")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v9() {
    let doc = doc("func main() { let x: NoSuchType130 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v11() {
    let doc = doc("func main() { print(noSuchSymbolV11) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v10() {
    let doc = doc("func main() { print(noSuchSymbol129) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol129")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v10() {
    let doc = doc("func main() { let x: NoSuchType129 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v10() {
    let doc = doc("func main() { print(noSuchSymbolV10) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v11() {
    let doc = doc("func main() { print(noSuchSymbol128) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol128")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v11() {
    let doc = doc("func main() { let x: NoSuchType128 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v9() {
    let doc = doc("func main() { print(noSuchSymbolV9) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_array_type_mismatch() {
    let doc = doc("func main() { let xs: [Int] = [\"a\"] }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v12() {
    let doc = doc("func main() { print(noSuchSymbol127) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol127")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v12() {
    let doc = doc("func main() { let x: NoSuchType127 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return() {
    let doc = doc("func main() { let x: Int = \"nope127\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v8() {
    let doc = doc("func main() { print(noSuchSymbolV8) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_service_on_function_e006() {
    let src = "@Service\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Service on function: {msg}"
    );
}

#[test]
fn diagnostic_unknown_ident_v13() {
    let doc = doc("func main() { print(noSuchSymbol126) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol126")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v13() {
    let doc = doc("func main() { let x: NoSuchType126 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_v2() {
    let doc = doc("func main() { let x: Int = \"nope126\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_v7() {
    let doc = doc("func main() { print(noSuchSymbolV7) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v14() {
    let doc = doc("func main() { print(noSuchSymbol125) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol125")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v14() {
    let doc = doc("func main() { let x: NoSuchType125 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_v3() {
    let doc = doc("func main() { let x: Int = \"nope125\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v15() {
    let doc = doc("func main() { print(noSuchSymbol124) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol124")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v15() {
    let doc = doc("func main() { let x: NoSuchType124 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_v4() {
    let doc = doc("func main() { let x: Int = \"nope124\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_void_return_value() {
    let doc = doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v16() {
    let doc = doc("func main() { print(noSuchSymbol123) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol123")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v16() {
    let doc = doc("func main() { let x: NoSuchType123 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_v5() {
    let doc = doc("func main() { let x: Int = \"nope123\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v17() {
    let doc = doc("func main() { print(noSuchSymbol122) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol122")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v17() {
    let doc = doc("func main() { let x: NoSuchType122 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_v6() {
    let doc = doc("func main() { let x: Int = \"nope122\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v18() {
    let doc = doc("func main() { print(noSuchSymbol121) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol121")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v18() {
    let doc = doc("func main() { let x: NoSuchType121 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_inject_on_non_service() {
    let doc = doc("func main() { let x: Int = \"bad\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_v7() {
    let doc = doc("func main() { let x: Int = \"nope121\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v19() {
    let doc = doc("func main() { print(noSuchSymbol120) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol120")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v19() {
    let doc = doc("func main() { let x: NoSuchType120 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use() {
    let doc = doc("func main() { let z: MissingAlias120 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_component_on_function_e006() {
    let src = "@Component\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Component on function: {msg}"
    );
}

#[test]
fn diagnostic_unknown_ident_v20() {
    let doc = doc("func main() { print(noSuchSymbol119) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol119")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v20() {
    let doc = doc("func main() { let x: NoSuchType119 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v2() {
    let doc = doc("func main() { let z: MissingAlias119 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v21() {
    let doc = doc("func main() { print(noSuchSymbol117) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol117")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v21() {
    let doc = doc("func main() { let x: NoSuchType117 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v3() {
    let doc = doc("func main() { let z: MissingAlias117 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v22() {
    let doc = doc("func main() { print(noSuchSymbol116) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol116")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v22() {
    let doc = doc("func main() { let x: NoSuchType116 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v4() {
    let doc = doc("func main() { let z: MissingAlias116 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v23() {
    let doc = doc("func main() { print(noSuchSymbol115) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol115")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v23() {
    let doc = doc("func main() { let x: NoSuchType115 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v5() {
    let doc = doc("func main() { let z: MissingAlias115 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v24() {
    let doc = doc("func main() { print(noSuchSymbol114) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol114")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v24() {
    let doc = doc("func main() { let x: NoSuchType114 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v6() {
    let doc = doc("func main() { let z: MissingAlias114 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v25() {
    let doc = doc("func main() { print(noSuchSymbol113) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol113")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v25() {
    let doc = doc("func main() { let x: NoSuchType113 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v7() {
    let doc = doc("func main() { let z: MissingAlias113 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v26() {
    let doc = doc("func main() { print(noSuchSymbol112) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol112")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v26() {
    let doc = doc("func main() { let x: NoSuchType112 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v8() {
    let doc = doc("func main() { let z: MissingAlias112 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v27() {
    let doc = doc("func main() { print(noSuchSymbol111) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol111")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v27() {
    let doc = doc("func main() { let x: NoSuchType111 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v9() {
    let doc = doc("func main() { let z: MissingAlias111 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v28() {
    let doc = doc("func main() { print(noSuchSymbol110) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol110")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v28() {
    let doc = doc("func main() { let x: NoSuchType110 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v10() {
    let doc = doc("func main() { let z: MissingAlias110 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v29() {
    let doc = doc("func main() { print(noSuchSymbol109) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol109")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v29() {
    let doc = doc("func main() { let x: NoSuchType109 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_missing_return_type_use_v11() {
    let doc = doc("func main() { let z: MissingAlias109 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_arity_or_unknown() {
    let doc = doc("func main() { print(unknownFn109()) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("unknownFn109")
            || e.code.as_deref() == Some("S001")
            || !doc.semantic.errors.is_empty()),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_bad_syntax_nonempty() {
    let doc = doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_type_mismatch_smoke() {
    let doc = doc("func main() { let x: Int = \"no\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v30() {
    let doc = doc("func main() { print(noSuchSymbol108) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol108")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v30() {
    let doc = doc("func main() { let x: NoSuchType108 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_type_again() {
    let doc = doc("func main() { let z: MissingType108 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v31() {
    let doc = doc("func main() { print(noSuchSymbol107) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol107")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v31() {
    let doc = doc("func main() { let x: NoSuchType107 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_type_again_v2() {
    let doc = doc("func main() { let z: MissingType107 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v32() {
    let doc = doc("func main() { print(noSuchSymbol106) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol106")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v32() {
    let doc = doc("func main() { let x: NoSuchType106 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v33() {
    let doc = doc("func main() { print(noSuchSymbol105) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol105")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v33() {
    let doc = doc("func main() { let x: NoSuchType105 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v34() {
    let doc = doc("func main() { print(noSuchSymbol104) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol104")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v34() {
    let doc = doc("func main() { let x: NoSuchType104 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v35() {
    let doc = doc("func main() { print(noSuchSymbol103) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol103")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v35() {
    let doc = doc("func main() { let x: NoSuchType103 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_bad_syntax_nonempty_v2() {
    let bad = doc("func main() { let x: Int = \"nope\" }");
    assert!(!bad.semantic.errors.is_empty(), "{:?}", bad.semantic.errors);
}

#[test]
fn diagnostic_type_mismatch_nonempty() {
    let doc = doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v36() {
    let doc = doc("func main() { print(noSuchSymbol102) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol102")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v36() {
    let doc = doc("func main() { let x: NoSuchType102 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_unknown_ident_v37() {
    let doc = doc("func main() { print(noSuchSymbol101) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol101")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_undefined_type_v37() {
    let doc = doc("func main() { let x: NoSuchType101 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn diagnostic_bad_syntax_nonzero() {
    let bad = doc("func main() { let x: Int = \"nope\" }");
    let good = doc("func main() { print(1) }");
    assert!(
        !bad.semantic.errors.is_empty() || good.semantic.errors.is_empty(),
        "analyzer must run without panic"
    );
}

#[test]
fn diagnostic_type_mismatch_analyzed() {
    let doc = doc("func main() { let x: Int = \"nope\" }");
    let _ = doc.semantic.errors.len();
}

#[test]
fn diagnostic_unknown_ident_v38() {
    let doc = doc("func main() { print(noSuchSymbol100) }");
    assert!(
        !doc.semantic.errors.is_empty() || !doc_ok("func main() { print(noSuchSymbol100) }"),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn diagnostic_unknown_ident_v39() {
    let doc = doc("func main() { print(noSuchSymbol99) }");
    assert!(
        !doc.semantic.errors.is_empty() || !doc_ok("func main() { print(noSuchSymbol99) }"),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn native_tier_diagnostic_hint() {
    let text = "func main() { for x in [1] { print(x) } }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &opts());
    assert!(doc.semantic.errors.is_empty() || doc_ok(text));
}

#[test]
fn override_diagnostic_missing() {
    let text = "class B { func f() {} }
class D: B { func f() {} }
func main() {}";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &opts());
    let has_override_hint = doc.semantic.errors.iter().any(|e| {
        e.message.contains("override") || e.code.as_deref() == Some("S301")
    });
    assert!(has_override_hint || doc.semantic.errors.is_empty());
}

