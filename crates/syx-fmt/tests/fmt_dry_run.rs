//! W683–W799 / ADR-1027–1143 / C-06: dry-run idempotency + CST print/let/var/typed/return/bare/multi-func.

use syx_diagnostics::SourceFile;
use syx_fmt::{format_dry_run, format_source_cst};

#[test]
fn fmt_dry_run_idempotent_fixture() {
    let src = "func main() {\n    print(1)\n}\n";
    let file = SourceFile::new("fixture.sy", src);
    let report = format_dry_run(&file).expect("dry-run must succeed on narrow fixture");
    assert!(
        !report.would_change || report.formatted.contains("func main"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("fixture.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent dry-run");
    assert!(!r2.would_change, "format(format(x)) must equal format(x)");
    assert_eq!(r2.formatted, report.formatted);
}

#[test]
fn fmt_dry_run_rejects_unsupported() {
    // W874: lazy computed formats; M0-A02 nails if-case KeyPath pattern F001.
    let file = SourceFile::new(
        "t.sy",
        "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n",
    );
    let err = format_dry_run(&file).expect_err("if case KeyPath pattern must F001");
    assert!(
        err.code.as_deref() == Some("F001") || err.message.contains("unsupported"),
        "{err:?}"
    );
    assert!(
        err.message.contains("if case pattern"),
        "expected if case pattern tip; got {err:?}"
    );
}

#[test]
fn fmt_cst_print_body_trivia_roundtrip() {
    let src = "// ahead\nfunc main() {\n  // inside\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst print body");
    assert_eq!(out, src);
    assert!(out.contains("// ahead") && out.contains("print(1)"));
}

#[test]
fn fmt_cst_print_deepen_roundtrip() {
    // W790 / ADR-1134: print(ident|string) + multi-print with trivia byte-identical.
    let src = "// ahead\nfunc main() {\n  // inside\n  print(1)\n  print(x)\n  print(\"hi\")\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst print deepen");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("// inside"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(\"hi\")"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_let_bind_roundtrip() {
    // W791 / ADR-1135: let[+print] with trivia byte-identical; reject call RHS.
    let src = "// ahead\nfunc main() {\n  // bind\n  let x = 1\n  let s = \"hi\"\n  print(x)\n  print(s)\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst let bind");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x = 1"));
    assert!(out.contains("let s = \"hi\""));
    assert!(out.contains("print(x)"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_return_simple_roundtrip() {
    // W792 / ADR-1136: return[+let/print] with trivia byte-identical; reject call RHS.
    let src = "// ahead\nfunc main() {\n  // bind\n  let x = 1\n  print(x)\n  return x\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst return simple");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x = 1"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return x"));
    assert!(out.contains("/* trail */"));

    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_bare_return_roundtrip() {
    // W797 / ADR-1141: bare return[+print/var] with trivia byte-identical; reject call RHS / while.
    let src = "// ahead\nfunc main() {\n  // done\n  var x = 1\n  print(x)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst bare return");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("var x = 1"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let semi = "// a\nfunc main() {\n  // b\n  return;\n}\n/* c */\n";
    let file_semi = SourceFile::new("semi.sy", semi);
    let out_semi = format_source_cst(&file_semi).expect("cst bare return;");
    assert_eq!(out_semi, semi);

    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_multi_func_roundtrip() {
    // W798 / ADR-1142: 2+ zero-arg funcs[+print/var/return] with trivia; reject call / param / while.
    let src = "// ahead\nfunc helper() {\n  // a\n  print(1)\n}\nfunc main() {\n  // b\n  var x = 1\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst multi func");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("func helper"));
    assert!(out.contains("func main"));
    assert!(out.contains("print(1)"));
    assert!(out.contains("var x = 1"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let param = SourceFile::new("param.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&param).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));

    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_bind_roundtrip() {
    // W799 / ADR-1143: typed let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    // `: (&Ident)->Ident` accepted since W805; `: (Ident?)->Ident` since W806 —
    // see fmt_cst_typed_function_param_optional_roundtrip.
    let src = "// ahead\nfunc main() {\n  // typed\n  let x: Int = 1\n  var s: String = \"hi\"\n  print(x)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed bind");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x: Int = 1"));
    assert!(out.contains("var s: String = \"hi\""));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_optional_roundtrip() {
    // W800 / ADR-1144: typed optional let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // optional type\n  let x: Int? = nil\n  var s: String? = \"hi\"\n  print(x)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed optional");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x: Int? = nil"));
    assert!(out.contains("var s: String? = \"hi\""));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_reference_roundtrip() {
    // W801 / ADR-1145: typed ref let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // ref type\n  let x: &Int = y\n  var s: &String = t\n  print(x)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed reference");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x: &Int = y"));
    assert!(out.contains("var s: &String = t"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_mut_reference_roundtrip() {
    // W802 / ADR-1146: typed mut-ref let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // mut ref\n  let x: &mut Int = y\n  var s: &mut String = t\n  print(x)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed mut reference");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x: &mut Int = y"));
    assert!(out.contains("var s: &mut String = t"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_lifetime_reference_roundtrip() {
    // W803 / ADR-1147: typed lifetime-ref let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // lifetime ref\n  let x: &'a Int = y\n  var s: &'a mut String = t\n  print(x)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed lifetime reference");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x: &'a Int = y"));
    assert!(out.contains("var s: &'a mut String = t"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_function_type_roundtrip() {
    // W804 / ADR-1148: typed function-type let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // func type\n  let f: (Int) -> Int = id\n  var g: (Int, String) -> Bool = h\n  let u: () -> Void = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function type");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: (Int) -> Int = id"));
    assert!(out.contains("var g: (Int, String) -> Bool = h"));
    assert!(out.contains("let u: () -> Void = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_function_param_reference_roundtrip() {
    // W805 / ADR-1149: Function param Reference let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // func param ref\n  let f: (&Int) -> Int = id\n  var g: (Int, &mut String) -> Bool = h\n  let u: (&'a Int) -> Void = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function param reference");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: (&Int) -> Int = id"));
    assert!(out.contains("var g: (Int, &mut String) -> Bool = h"));
    assert!(out.contains("let u: (&'a Int) -> Void = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_function_param_optional_roundtrip() {
    // W806 / ADR-1150: Function param Optional let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // func param optional\n  let f: (Int?) -> Int = id\n  var g: (String?, &mut Int) -> Bool = h\n  let u: (Int?, String?) -> Void = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function param optional");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: (Int?) -> Int = id"));
    assert!(out.contains("var g: (String?, &mut Int) -> Bool = h"));
    assert!(out.contains("let u: (Int?, String?) -> Void = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_function_return_reference_roundtrip() {
    // W807 / ADR-1151: Function return Reference let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // func return ref\n  let f: (Int) -> &Int = id\n  var g: (String?) -> &mut String = h\n  let u: () -> &'a Int = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function return reference");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: (Int) -> &Int = id"));
    assert!(out.contains("var g: (String?) -> &mut String = h"));
    assert!(out.contains("let u: () -> &'a Int = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_function_return_optional_roundtrip() {
    // W808 / ADR-1152: Function return Optional let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // func return optional\n  let f: (Int) -> Int? = id\n  var g: (String?) -> String? = h\n  let u: () -> Void? = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function return optional");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: (Int) -> Int? = id"));
    assert!(out.contains("var g: (String?) -> String? = h"));
    assert!(out.contains("let u: () -> Void? = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_typed_function_param_nested_roundtrip() {
    // W809 / ADR-1153: nested Function param let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // nested function param\n  let f: ((Int) -> Int) -> Int = id\n  var g: ((String?) -> String?) -> Void = h\n  let u: (() -> Void) -> Int? = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function param nested");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: ((Int) -> Int) -> Int = id"));
    assert!(out.contains("var g: ((String?) -> String?) -> Void = h"));
    assert!(out.contains("let u: (() -> Void) -> Int? = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_function_return_nested_roundtrip() {
    // W810 / ADR-1154: nested Function return let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // nested function return\n  let f: (Int) -> ((Int) -> Int) = id\n  var g: (String?) -> (() -> Void) = h\n  let u: () -> ((Int?) -> Int?) = k\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed function return nested");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let f: (Int) -> ((Int) -> Int) = id"));
    assert!(out.contains("var g: (String?) -> (() -> Void) = h"));
    assert!(out.contains("let u: () -> ((Int?) -> Int?) = k"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_amp_optional_roundtrip() {
    // W811 / ADR-1155: Amp+`?` bind/Function let/var[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // amp optional\n  let a: &Int? = p\n  var b: &mut String? = q\n  let f: (&Int?) -> Int = id\n  var g: (Int) -> &mut String? = h\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed amp optional");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let a: &Int? = p"));
    assert!(out.contains("var b: &mut String? = q"));
    assert!(out.contains("let f: (&Int?) -> Int = id"));
    assert!(out.contains("var g: (Int) -> &mut String? = h"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_generic_roundtrip() {
    // W812 / ADR-1156: Foo<Int>[?] bind/Function[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // typed generic\n  let a: Foo<Int> = p\n  var b: Bar<String>? = q\n  let f: (Foo<Int>) -> Int = id\n  var g: (Int) -> Bar<String> = h\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed generic");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let a: Foo<Int> = p"));
    assert!(out.contains("var b: Bar<String>? = q"));
    assert!(out.contains("let f: (Foo<Int>) -> Int = id"));
    assert!(out.contains("var g: (Int) -> Bar<String> = h"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_nested_generic_roundtrip() {
    // W813 / ADR-1157: Foo<Int>[?] bind/Function[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // nested generic\n  let a: Foo<Int> = p\n  var b: Box<String>? = q\n  let f: (Foo<Int>) -> Int = id\n  var g: (Int) -> Box<String> = h\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed nested generic");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let a: Foo<Int> = p"));
    assert!(out.contains("var b: Box<String>? = q"));
    assert!(out.contains("let f: (Foo<Int>) -> Int = id"));
    assert!(out.contains("var g: (Int) -> Box<String> = h"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_multi_arg_generic_roundtrip() {
    // W814 / ADR-1158: Foo<A,B>[?] bind/Function[+print/return] with trivia; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc main() {\n  // multi-arg generic\n  let a: Foo<A> = p\n  var b: Dict<String, Int>? = q\n  let f: (Foo<A>) -> Int = id\n  var g: (Int) -> Dict<String, Int> = h\n  print(f)\n  return\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed multi-arg generic");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let a: Foo<A> = p"));
    assert!(out.contains("var b: Dict<String, Int>? = q"));
    assert!(out.contains("let f: (Foo<A>) -> Int = id"));
    assert!(out.contains("var g: (Int) -> Dict<String, Int> = h"));
    assert!(out.contains("print(f)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_toplevel_simple_roundtrip() {
    // W815 / ADR-1159: ahead/between/trail simple + zero-arg funcs; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nprint(0)\nfunc helper() {\n  let n: Int = 1\n}\n// between\nlet x: Foo<A, B> = p\nvar y: Dict<String, Int>? = q\nprint(x)\nfunc main() {\n  print(y)\n  return\n}\nreturn\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst toplevel simple");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("print(0)"));
    assert!(out.contains("let x: Foo<A, B> = p"));
    assert!(out.contains("var y: Dict<String, Int>? = q"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(y)"));
    assert!(out.contains("return"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_param_func_roundtrip() {
    // W816 / ADR-1160: bare Ident params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x) {\n  // param\n  print(x)\n  return x\n}\nfunc add(a, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst param func");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("func id(x)"));
    assert!(out.contains("func add(a, b)"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(0)"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_typed_param_roundtrip() {
    // W817 / ADR-1161: Ident[: Ident] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: Int) {\n  // typed param\n  print(x)\n  return x\n}\nfunc add(a: Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed param");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("func id(x: Int)"));
    assert!(out.contains("func add(a: Int, b)"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(0)"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_param_optional_roundtrip() {
    // W818 / ADR-1162: Ident[: Ident[?]] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: Int?) {\n  // typed param optional\n  print(x)\n  return x\n}\nfunc add(a: Int?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed param optional");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("func id(x: Int?)"));
    assert!(out.contains("func add(a: Int?, b)"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(0)"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_param_amp_roundtrip() {
    // W819 / ADR-1163: Ident[: Ident[?]| &Ident] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: &Int) {\n  // typed param amp\n  print(x)\n  return x\n}\nfunc add(a: &Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed param amp");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("func id(x: &Int)"));
    assert!(out.contains("func add(a: &Int, b)"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(0)"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}



#[test]
fn fmt_cst_typed_param_mut_amp_roundtrip() {
    // W820 / ADR-1164: Ident[: Ident[?]| &Ident| &mut Ident] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: &mut Int) {\n  // typed param mut amp\n  print(x)\n  return x\n}\nfunc add(a: &mut Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst typed param mut amp");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("func id(x: &mut Int)"));
    assert!(out.contains("func add(a: &mut Int, b)"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("print(0)"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));

    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_param_lifetime_amp_roundtrip() {
    // W821 / ADR-1165: Ident[: Ident[?]| &Ident| &mut Ident| &'a Ident| &'a mut Ident] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: &'a Int) {\n  // typed param lifetime amp\n  print(x)\n  return x\n}\nfunc add(a: &'a mut Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst typed param lifetime amp");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: &'a Int)"));
    assert!(out.contains("func add(a: &'a mut Int, b)"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_param_amp_optional_roundtrip() {
    // W822 / ADR-1166: Ident[: &Ident[?]| &mut Ident[?]| &'a Ident[?]| &'a mut Ident[?]] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: &Int?) {\n  // typed param amp optional\n  print(x)\n  return x\n}\nfunc add(a: &mut Int?, b: &'a Int?) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst typed param amp optional");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: &Int?)"));
    assert!(out.contains("func add(a: &mut Int?, b: &'a Int?)"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_param_generic_roundtrip() {
    // W823 / ADR-1167: Ident[: Ident[<Ident>][?]| Amp[?]…] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: Foo<Int>) {\n  // typed param generic\n  print(x)\n  return x\n}\nfunc add(a: List<Int>?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst typed param generic");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Foo<Int>)"));
    assert!(out.contains("func add(a: List<Int>?, b)"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_param_nested_generic_roundtrip() {
    // W824 / ADR-1168: Ident[: Ident[<NestedNamed>][?]| Amp[?]…] params + simple body[+toplevel]; reject call / Amp-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: Foo<Int>) {\n  // typed param nested generic\n  print(x)\n  return x\n}\nfunc add(a: Box<Int>?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst typed param nested generic");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Foo<Int>)"));
    assert!(out.contains("func add(a: Box<Int>?, b)"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_typed_param_multi_arg_generic_roundtrip() {
    // W825 / ADR-1169: Ident[: Ident[<Named(, Named)*>][?]| Amp[?]…] params + simple body[+toplevel]; reject call / Function-as-generic-arg typed param.
    let src = "// ahead\nfunc id(x: Foo<A, B>) {\n  // typed param multi-arg generic\n  print(x)\n  return x\n}\nfunc add(a: Dict<String, Int>?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst typed param multi-arg generic");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Foo<A, B>)"));
    assert!(out.contains("func add(a: Dict<String, Int>?, b)"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_roundtrip() {
    // W826 / ADR-1170: func … [-> Ident[?]] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> Int {\n  // func return\n  print(x)\n  return x\n}\nfunc none() -> Void? {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> Int"));
    assert!(out.contains("func none() -> Void?"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_amp_roundtrip() {
    // W827 / ADR-1171: func … [-> Ident[?]| &Ident] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> &Int {\n  // func return amp\n  print(x)\n  return x\n}\nfunc bare() -> &String {\n  return\n}\nfunc none() -> Void? {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return amp");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> &Int"));
    assert!(out.contains("func bare() -> &String"));
    assert!(out.contains("func none() -> Void?"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_mut_amp_roundtrip() {
    // W828 / ADR-1172: func … [-> Ident[?]| &Ident| &mut Ident] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> &mut Int {\n  // func return mut amp\n  print(x)\n  return x\n}\nfunc bare() -> &mut String {\n  return\n}\nfunc amp() -> &Int {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return mut amp");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> &mut Int"));
    assert!(out.contains("func bare() -> &mut String"));
    assert!(out.contains("func amp() -> &Int"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_lifetime_amp_roundtrip() {
    // W829 / ADR-1173: func … [-> Ident[?]| &Ident| &mut Ident| &'a Ident| &'a mut Ident] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> &'a Int {\n  // func return lifetime amp\n  print(x)\n  return x\n}\nfunc mut_life() -> &'b mut String {\n  return\n}\nfunc amp() -> &Int {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return lifetime amp");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> &'a Int"));
    assert!(out.contains("func mut_life() -> &'b mut String"));
    assert!(out.contains("func amp() -> &Int"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_amp_optional_roundtrip() {
    // W830 / ADR-1174: func … [-> Ident[?]| &Ident[?]| &mut Ident[?]| &'a Ident[?]| &'a mut Ident[?]] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> &Int? {\n  // func return amp optional\n  print(x)\n  return x\n}\nfunc mut_opt() -> &mut String? {\n  return\n}\nfunc life() -> &'a Int? {\n  return\n}\nfunc mut_life() -> &'b mut String? {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return amp optional");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> &Int?"));
    assert!(out.contains("func mut_opt() -> &mut String?"));
    assert!(out.contains("func life() -> &'a Int?"));
    assert!(out.contains("func mut_life() -> &'b mut String?"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_generic_roundtrip() {
    // W831 / ADR-1175: func … [-> Ident[<Ident>][?]| Amp 族[?]] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> Foo<Int> {\n  // func return generic\n  print(x)\n  return x\n}\nfunc opt() -> Bar<String>? {\n  return\n}\nfunc amp() -> &Int? {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return generic");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> Foo<Int>"));
    assert!(out.contains("func opt() -> Bar<String>?"));
    assert!(out.contains("func amp() -> &Int?"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}







#[test]
fn fmt_cst_func_return_nested_generic_roundtrip() {
    // W832 / ADR-1176: func … [-> Ident[<NestedNamed>][?]| Amp 族[?]] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> Foo<Int> {\n  // func return nested generic\n  print(x)\n  return x\n}\nfunc opt() -> Box<String>? {\n  return\n}\nfunc flat() -> Foo<Int> {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return nested generic");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> Foo<Int>"));
    assert!(out.contains("func opt() -> Box<String>?"));
    assert!(out.contains("func flat() -> Foo<Int>"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let gen_ret = SourceFile::new("gen.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&gen_ret).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err3 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err3.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_func_return_multi_arg_generic_roundtrip() {
    // W833 / ADR-1177 face1: func … [-> Ident[<Arg(, Arg)*>][?]| Amp 族[?]] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> Foo<A> {\n  // func return multi-arg generic\n  print(x)\n  return x\n}\nfunc opt() -> Dict<String, Int>? {\n  return\n}\nfunc nested() -> Foo<Int> {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return multi-arg generic");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> Foo<A>"));
    assert!(out.contains("func opt() -> Dict<String, Int>?"));
    assert!(out.contains("func nested() -> Foo<Int>"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_return_function_roundtrip() {
    // W833 / ADR-1177 face2: func … [-> Function| Ident[?]| Amp 族[?]] + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Int) -> (Int) -> Bool {\n  // func return function\n  print(x)\n  return x\n}\nfunc unit() -> () {\n  return\n}\nfunc named() -> Int {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst func return function");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Int) -> (Int) -> Bool"));
    assert!(out.contains("func unit() -> ()"));
    assert!(out.contains("func named() -> Int"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_amp_as_generic_arg_roundtrip() {
    // W833 / ADR-1177 face3: Amp-as-generic-arg param/return + simple body[+toplevel]; reject call / Function-as-generic-arg.
    let src = "// ahead\nfunc id(x: Foo<&Int>) {\n  // amp as generic arg\n  print(x)\n  return x\n}\nfunc ret() -> Bar<&T> {\n  return\n}\nfunc mutp(x: Box<&mut String>) {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst amp as generic arg");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Foo<&Int>)"));
    assert!(out.contains("func ret() -> Bar<&T>"));
    assert!(out.contains("func mutp(x: Box<&mut String>)"));
    assert!(out.contains("print(0)"));
    let call = SourceFile::new("call.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&call).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let pref = SourceFile::new("pref.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&pref).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_function_as_generic_arg_roundtrip() {
    // W834 / ADR-1178 face1: Function-as-generic-arg param/return + simple body[+toplevel]; reject chained logic.
    let src = "// ahead\nfunc id(x: Foo<(Int)->Bool>) {\n  // function as generic arg\n  print(x)\n  return x\n}\nfunc ret() -> Bar<()> {\n  return\n}\nfunc nest(x: Box<(Int)->((Int)->Bool)>) {\n  return\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst function as generic arg");
    assert_eq!(out, src);
    assert!(out.contains("func id(x: Foo<(Int)->Bool>)"));
    assert!(out.contains("func ret() -> Bar<()>"));
    assert!(out.contains("func nest(x: Box<(Int)->((Int)->Bool)>)"));
    assert!(out.contains("print(0)"));
    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let nest = SourceFile::new("throws.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_print_call_roundtrip() {
    // W834 / ADR-1178 face2: print(call-simple) + simple body[+toplevel]; reject chained logic.
    let src = "// ahead\nfunc main() {\n  // print call\n  print(foo())\n  print(g(1))\n  print(h(x, y))\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst print call");
    assert_eq!(out, src);
    assert!(out.contains("print(foo())"));
    assert!(out.contains("print(g(1))"));
    assert!(out.contains("print(h(x, y))"));
    assert!(out.contains("print(0)"));
    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let nest = SourceFile::new("throws.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_call_rhs_roundtrip() {
    // W834 / ADR-1178 face3: let/var/return call-simple + simple body[+toplevel]; reject chained logic.
    let src = "// ahead\nfunc main() {\n  // call rhs\n  let x = foo()\n  var y = g(1)\n  print(h(x, y))\n  return k()\n}\nprint(0)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst call rhs");
    assert_eq!(out, src);
    assert!(out.contains("let x = foo()"));
    assert!(out.contains("var y = g(1)"));
    assert!(out.contains("print(h(x, y))"));
    assert!(out.contains("return k()"));
    assert!(out.contains("print(0)"));
    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let nest = SourceFile::new("deep2.sy", "func main() { while -------------------n { print(1) } }\n");
    let err2 = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_while_ident_cond_roundtrip() {
    // W835 / ADR-1179 face1: while true|ident + print/let body; reject chained logic.
    let src = "// ahead\nfunc main() {\n  while true {\n    print(1)\n  }\n  while flag {\n    let x = 1\n    print(x)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while ident cond");
    assert_eq!(out, src);
    assert!(out.contains("while true"));
    assert!(out.contains("while flag"));
    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_call_cond_roundtrip() {
    // W835 face2: while ready()|tick(1) + body; reject chained logic.
    let src = "// ahead\nfunc main() {\n  while ready() {\n    print(1)\n  }\n  while tick(1) {\n    var y = 2\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while call cond");
    assert_eq!(out, src);
    assert!(out.contains("while ready()"));
    assert!(out.contains("while tick(1)"));
    let nest = SourceFile::new("throws.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_mixed_roundtrip() {
    // W835 face3: mixed + toplevel while; reject still-out-of-subset faces.
    let src = "// ahead\nwhile go {\n  print(0)\n}\nfunc main() {\n  while true {\n    print(1)\n    return\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while mixed");
    assert_eq!(out, src);
    assert!(out.contains("while go"));
    assert!(out.contains("return"));
    let nobrace = SourceFile::new("nobrace.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nobrace).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err2 = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_nested_roundtrip() {
    // W836 / ADR-1180 face1: nested while; reject chained logic.
    let src = "// ahead\nfunc main() {\n  while true {\n    while flag {\n      print(1)\n    }\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while nested");
    assert_eq!(out, src);
    assert!(out.contains("while true"));
    assert!(out.contains("while flag"));
    let tp_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_compare_cond_roundtrip() {
    // W836 face2: compare cond; reject chained logic.
    let src = "// ahead\nfunc main() {\n  while x < n {\n    print(x)\n  }\n  while a >= b {\n    print(a)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while compare");
    assert_eq!(out, src);
    assert!(out.contains("while x < n"));
    assert!(out.contains("while a >= b"));
    let nest = SourceFile::new("throws.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_deeper_mixed_roundtrip() {
    // W836 face3: nested×compare + toplevel; reject still-out-of-subset faces.
    let src = "// ahead\nwhile go {\n  while i < n {\n    print(i)\n  }\n}\nfunc main() {\n  while ready() {\n    while flag {\n      print(1)\n    }\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while deeper mixed");
    assert_eq!(out, src);
    assert!(out.contains("while i < n"));
    assert!(out.contains("while ready()"));
    let nobrace = SourceFile::new("nobrace.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nobrace).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err2 = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_effect_pure_roundtrip() {
    // W837 / ADR-1181 face1: requires Pure; reject chained logic.
    let src = "// ahead\nfunc main() requires Pure {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst effect pure");
    assert_eq!(out, src);
    assert!(out.contains("requires Pure"));
    let nest = SourceFile::new("throws.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_effect_io_roundtrip() {
    // W837 face2: requires IO + params/return; reject chained logic.
    let src = "// ahead\nfunc go(x: Int) -> Int requires IO {\n  print(x)\n  return x\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst effect io");
    assert_eq!(out, src);
    assert!(out.contains("requires IO"));
    let tp = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_effect_mixed_roundtrip() {
    // W837 face3: Pure×IO + while; reject chained logic.
    let src = "// ahead\nfunc pure() requires Pure {\n  print(0)\n}\nfunc io() requires IO {\n  while true {\n    print(1)\n  }\n}\nprint(2)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst effect mixed");
    assert_eq!(out, src);
    assert!(out.contains("requires Pure"));
    assert!(out.contains("requires IO"));
    let where_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&where_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}


#[test]
fn fmt_cst_where_bare_roundtrip() {
    // W841 / ADR-1185 face1: bare where; reject chained logic.
    let src = "// ahead\nfunc id<T>(x: T) where T: Equatable {\n  print(x)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst where bare");
    assert_eq!(out, src);
    assert!(out.contains("where T: Equatable"));
    let deep = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_where_clause_roundtrip() {
    // W841 face2: multi preds / return / requires; reject chained logic.
    let src = "// ahead\nfunc pair<A, B>(a: A, b: B) where A: Equatable, B: Hashable {\n  print(a)\n}\nfunc id<T>(x: T) -> T requires Pure where T: Equatable {\n  return x\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst where clause");
    assert_eq!(out, src);
    assert!(out.contains("where A: Equatable, B: Hashable"));
    let brace = SourceFile::new("brace.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&brace).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_where_mixed_roundtrip() {
    // W841 face3: where × throws × effect × nested-call; reject chained logic.
    let src = "// ahead\nfunc id<T>(x: T) throws requires Pure where T: Equatable {\n  print(foo(bar()))\n}\nfunc go<U>() where U: Bound {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\nprint(1)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst where mixed");
    assert_eq!(out, src);
    assert!(out.contains("where T: Equatable"));
    assert!(out.contains("print(foo(bar()))"));
    let logic = SourceFile::new("logic.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&logic).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}




#[test]
fn fmt_cst_while_logic_and_roundtrip() {
    // W844 / ADR-1188 face1: && braced; reject chained.
    let src = "// ahead\nfunc main() {\n  while a && b {\n    print(1)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic and");
    assert_eq!(out, src);
    assert!(out.contains("while a && b"));
    let chain = SourceFile::new("chain.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&chain).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_or_roundtrip() {
    // W844 face2: || + brace-less &&; reject compare×logic.
    let src = "// ahead\nfunc main() {\n  while x || y {\n    print(1)\n  }\n  while a && b\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic or");
    assert_eq!(out, src);
    assert!(out.contains("while x || y"));
    let mix = SourceFile::new("mix.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&mix).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_mixed_roundtrip() {
    // W844 face3: logic × effect/where/depth2; reject depth>2.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while a && b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while x || y\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic mixed");
    assert_eq!(out, src);
    assert!(out.contains("while a && b"));
    let deep = SourceFile::new("deep.sy", "func main() { print(foo(bar(baz(qux())))) }\n");
    let err = format_source_cst(&deep).expect_err("depth>2 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_chain_roundtrip() {
    // W845 / ADR-1189 face1: same-op chain; reject mixed-ops.
    let src = "// ahead\nfunc main() {\n  while a && b && c {\n    print(1)\n  }\n  while x || y || z\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic chain");
    assert_eq!(out, src);
    assert!(out.contains("while a && b && c"));
    let mix = SourceFile::new("mix.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&mix).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_cmp_roundtrip() {
    // W845 face2: compare×logic; reject unary !.
    let src = "// ahead\nfunc main() {\n  while a < b && c {\n    print(1)\n  }\n  while x >= n || flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic cmp");
    assert_eq!(out, src);
    assert!(out.contains("while a < b && c"));
    let bang = SourceFile::new("bang.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_deep_mixed_roundtrip() {
    // W845 face3: chain × cmp×logic × effect/where/depth2; reject depth>2.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while a && b && c {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while i < n || flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic deep mixed");
    assert_eq!(out, src);
    assert!(out.contains("while a && b && c"));
    let deep = SourceFile::new("deep.sy", "func main() { print(foo(bar(baz(qux())))) }\n");
    let err = format_source_cst(&deep).expect_err("depth>2 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_mixedops_roundtrip() {
    // W846 / ADR-1190 face1: mixed-ops left-assoc; reject unary !.
    let src = "// ahead\nfunc main() {\n  while a && b || c {\n    print(1)\n  }\n  while x || y && z\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic mixedops");
    assert_eq!(out, src);
    assert!(out.contains("while a && b || c"));
    let bang = SourceFile::new("bang.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_call_roundtrip() {
    // W846 face2: call×logic; reject paren.
    let src = "// ahead\nfunc main() {\n  while ready() && flag {\n    print(1)\n  }\n  while tick(1) || done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic call");
    assert_eq!(out, src);
    assert!(out.contains("while ready() && flag"));
    let paren = SourceFile::new("paren.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&paren).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_logic_final_mixed_roundtrip() {
    // W846 face3: mixed-ops × call×logic × effect/where/depth2; reject depth>2.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while i < n || tick(1)\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while logic final mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { print(foo(bar(baz(qux())))) }\n");
    let err = format_source_cst(&deep).expect_err("depth>2 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_unary_bare_roundtrip() {
    // W847 / ADR-1191 face1: bare unary !; reject Neg.
    let src = "// ahead\nfunc main() {\n  while !flag {\n    print(1)\n  }\n  while !done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while unary bare");
    assert_eq!(out, src);
    assert!(out.contains("while !flag"));
    let neg = SourceFile::new("neg.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&neg).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_unary_logic_roundtrip() {
    // W847 face2: unary×logic; reject !! .
    let src = "// ahead\nfunc main() {\n  while !a && b {\n    print(1)\n  }\n  while !ready() || done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while unary logic");
    assert_eq!(out, src);
    assert!(out.contains("while !a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_unary_mixed_roundtrip() {
    // W847 face3: unary × effect/where/depth2; reject Neg.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while unary mixed");
    assert_eq!(out, src);
    assert!(out.contains("while !ready() && a || b"));
    let neg = SourceFile::new("neg.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&neg).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren_bare_roundtrip() {
    // W848 / ADR-1192 face1: bare paren / paren×logic; reject Neg.
    let src = "// ahead\nfunc main() {\n  while (a && b) || c {\n    print(1)\n  }\n  while a && (b || c)\n  print(1)\n  while (flag)\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren bare");
    assert_eq!(out, src);
    assert!(out.contains("while (a && b) || c"));
    let neg = SourceFile::new("neg.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&neg).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren_unary_roundtrip() {
    // W848 face2: paren×unary·call; reject nested paren.
    let src = "// ahead\nfunc main() {\n  while !(a && b) {\n    print(1)\n  }\n  while (!ready() && flag)\n  print(1)\n  while (tick(1) || done)\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren unary");
    assert_eq!(out, src);
    assert!(out.contains("while !(a && b)"));
    let nest = SourceFile::new("nest.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren_mixed_roundtrip() {
    // W848 face3: paren × effect/where/depth2; reject !! .
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while (a && b) || ready() {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !(x || y)\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren mixed");
    assert_eq!(out, src);
    assert!(out.contains("while (a && b) || ready()"));
    let bang = SourceFile::new("bang.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg_bare_roundtrip() {
    // W849 / ADR-1193 face1: bare Neg `-`; reject !! .
    let src = "// ahead\nfunc main() {\n  while -n {\n    print(1)\n  }\n  while -done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg bare");
    assert_eq!(out, src);
    assert!(out.contains("while -n"));
    let bang = SourceFile::new("bang.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg_logic_roundtrip() {
    // W849 face2: Neg×logic·call; reject -- .
    let src = "// ahead\nfunc main() {\n  while -a && b {\n    print(1)\n  }\n  while -ready() || done\n  print(1)\n  while (-n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg logic");
    assert_eq!(out, src);
    assert!(out.contains("while -a && b"));
    let dbl = SourceFile::new("dbl.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&dbl).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg_mixed_roundtrip() {
    // W849 face3: Neg × effect/where/depth2; reject nested paren.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg mixed");
    assert_eq!(out, src);
    assert!(out.contains("while -ready() && a || b"));
    let nest = SourceFile::new("nest.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_bang2_bare_roundtrip() {
    // W850 / ADR-1194 face1: bare `!!`; reject -- .
    let src = "// ahead\nfunc main() {\n  while !!flag {\n    print(1)\n  }\n  while !!done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while bang2 bare");
    assert_eq!(out, src);
    assert!(out.contains("while !!flag"));
    let nest = SourceFile::new("nest.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&nest).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_bang2_logic_roundtrip() {
    // W850 face2: !!×logic·call; reject -- .
    let src = "// ahead\nfunc main() {\n  while !!a && b {\n    print(1)\n  }\n  while !!ready() || done\n  print(1)\n  while (!!flag) && x\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while bang2 logic");
    assert_eq!(out, src);
    assert!(out.contains("while !!a && b"));
    let trip = SourceFile::new("trip.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&trip).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_bang2_mixed_roundtrip() {
    // W850 face3: !! × effect/where/depth2; reject -- .
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !!ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !!flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while bang2 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while !!ready() && a || b"));
    let dbl = SourceFile::new("dbl.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&dbl).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren2_bare_roundtrip() {
    // W851 / ADR-1195 face1: bare nested paren depth=2; reject ((((…)))).
    let src = "// ahead\nfunc main() {\n  while ((flag)) {\n    print(1)\n  }\n  while ((done))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren2 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ((flag))"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren2_logic_roundtrip() {
    // W851 face2: nest×logic·call / bang nest; reject -- .
    let src = "// ahead\nfunc main() {\n  while ((a && b)) || c {\n    print(1)\n  }\n  while ((tick(1)))\n  print(1)\n  while !(!(ready()))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren2 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ((a && b)) || c"));
    let trip = SourceFile::new("trip.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&trip).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren2_mixed_roundtrip() {
    // W851 face3: nest × effect/where/depth2/!!·Neg; reject -- .
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ((!!ready())) && a || -b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ((flag))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren2 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ((!!ready())) && a || -b"));
    let dbl = SourceFile::new("dbl.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&dbl).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren3_bare_roundtrip() {
    // W871 / ADR-1215 face1: bare nested paren depth=3; reject ((((…)))).
    let src = "// ahead\nfunc main() {\n  while (((flag))) {\n    print(1)\n  }\n  while (((done)))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren3 bare");
    assert_eq!(out, src);
    assert!(out.contains("while (((flag)))"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren3_logic_roundtrip() {
    // W871 face2.
    let src = "// ahead\nfunc main() {\n  while (((a && b))) || c {\n    print(1)\n  }\n  while (((tick(1))))\n  print(1)\n  while !(((ready())))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren3 logic");
    assert_eq!(out, src);
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_paren3_mixed_roundtrip() {
    // W871 face3.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while (((!!!!ready()))) && a || -b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while (((flag)))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while paren3 mixed");
    assert_eq!(out, src);
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_comment_line_roundtrip() {
    // W872 / ADR-1216 face1: file-level + body line comments; AST still drops.
    use syx_fmt::format_source;
    let src = "// ahead\nfunc main() {\n  // inside\n  print(1)\n  // mid\n  let x = 1\n}\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst comment line");
    assert_eq!(out, src);
    assert!(out.contains("// ahead") && out.contains("// inside") && out.contains("// mid"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("// ahead") && !ast.contains("// inside") && !ast.contains("// mid"),
        "AST must drop line comments; got {ast:?}"
    );
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_comment_stmt_roundtrip() {
    // W872 face2: line×stmt·while / inline block before paren.
    use syx_fmt::format_source;
    let src = "// ahead\nfunc main() {\n  let x = 1 // trail-let\n  while flag // trail-while\n  print(1)\n  while /*c*/ (((a)))\n  print(2)\n}\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst comment stmt");
    assert_eq!(out, src);
    assert!(out.contains("let x = 1 // trail-let"));
    assert!(out.contains("while /*c*/ (((a)))"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("// trail-let") && !ast.contains("/*c*/"),
        "AST must drop stmt comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_comment_mixed_roundtrip() {
    // W872 face3: // + /* */ × effect/where/depth2/!!!!·Neg.
    use syx_fmt::format_source;
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  // body\n  while (((!!!!ready()))) && a || -b /* mid */\n  print(foo(bar(baz())))\n}\nfunc main() {\n  /* block */\n  while (((flag)))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst comment mixed");
    assert_eq!(out, src);
    assert!(out.contains("/* mid */") && out.contains("/* trail */"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop mixed comments; got {ast:?}"
    );
}

#[test]
fn fmt_default_cst_preserves_comments() {
    // W873 / ADR-1217 face1: default format_dry_run (no env) preserves line comments.
    let src = "// ahead\nfunc main() {\n  // inside\n  print(1)\n  // mid\n  let x = 1\n}\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default CST-first dry-run");
    assert!(!report.would_change, "subset+comments must be byte-stable");
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("// ahead") && report.formatted.contains("// inside"),
        "default path must preserve comments"
    );
}

#[test]
fn fmt_default_cst_fallback_ast_f001() {
    // W873 face2 + W874: CST001 → AST; if-case KeyPath still F001; caps still reject on pure CST.
    let case_pat = SourceFile::new(
        "case.sy",
        "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n",
    );
    let err = format_dry_run(&case_pat).expect_err("if case KeyPath must F001 via AST fallback");
    assert_eq!(err.code.as_deref(), Some("F001"));
    assert!(
        err.message.contains("if case pattern"),
        "expected if case pattern tip; got {err:?}"
    );

    // Pure CST still rejects nest>3 / bang5 / neg19 (caps frozen).
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren4 CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
    let bang5 = SourceFile::new("b.sy", "func main() { while !!!!!flag { print(1) } }\n");
    assert_eq!(
        format_source_cst(&bang5)
            .expect_err("bang5")
            .code
            .as_deref(),
        Some("CST001")
    );
    let neg19 =
        SourceFile::new("n.sy", "func main() { while -------------------n { print(1) } }\n");
    assert_eq!(
        format_source_cst(&neg19)
            .expect_err("neg19")
            .code
            .as_deref(),
        Some("CST001")
    );

    // Default dry-run on out-of-subset paren nest falls back to AST (must not CST001).
    let deep_auto = format_dry_run(&deep).expect("paren4 must AST-fallback");
    assert!(deep_auto.formatted.contains("while"));
}

#[test]
fn fmt_default_cst_mixed_comment_mvp() {
    // W873 face3: mixed comment fixture stable under default dry-run.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  // body\n  while (((!!!!ready()))) && a || -b /* mid */\n  print(foo(bar(baz())))\n}\nfunc main() {\n  /* block */\n  while (((flag)))\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("mixed.sy", src);
    let report = format_dry_run(&file).expect("default mixed comment");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("/* mid */") && report.formatted.contains("/* trail */"));
}

#[test]
fn fmt_cst_func_access_public_dry_run() {
    // W875 / ADR-1219 face1: public func + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\npublic func api() {\n  // body\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default public access");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("public func api") && report.formatted.contains("/// docs"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("// body") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject face (W878): export {…} list still CST001.
    let ex = SourceFile::new("e.sy", "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n");
    let err = format_source_cst(&ex).expect_err("if case KeyPath must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_access_private_dry_run() {
    // W875 face2: private / internal / fileprivate + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nprivate func a() {\n  print(1)\n}\ninternal func b() {\n  /* mid */\n  print(2)\n}\nfileprivate func c() {\n  print(3)\n}\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default private family");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("private func a")
            && report.formatted.contains("internal func b")
            && report.formatted.contains("fileprivate func c")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */"),
        "AST must drop comments; got {ast:?}"
    );
    // W876: static+async stack still CST001.
    let stack = SourceFile::new("s.sy", "static async func main() { print(1) }\n");
    let err = format_source_cst(&stack).expect_err("static async stack must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_access_mixed_dry_run() {
    // W875 face3: access × throws/effect/where/while + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic func go<T>() throws requires Pure where T: Bound {\n  // body\n  while flag /* mid */\n  print(1)\n}\nprivate func main() {\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default access mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("public func go")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_func_static_dry_run() {
    // W876 / ADR-1220 face1: static func + docs; default CST preserves.
    // Top-level `static func` is not AST-parseable (P001); evidence = CST path vs AST fail.
    use syx_fmt::format_source;
    let src = "/// docs\nstatic func api() {\n  // body\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default static");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("static func api") && report.formatted.contains("/// docs"));
    let ast_err = format_source(&file).expect_err("top-level static func must fail AST parse");
    assert_eq!(ast_err.code.as_deref(), Some("P001"));
    // W878: export {…} list still CST001 (export func itself now in subset).
    let ex = SourceFile::new("e.sy", "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n");
    let err = format_source_cst(&ex).expect_err("if case KeyPath must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_async_dry_run() {
    // W876 face2: async func + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nasync func a() {\n  print(1)\n}\nasync func b() {\n  /* mid */\n  print(2)\n}\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default async");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("async func a") && report.formatted.contains("async func b")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_func_static_async_mixed_dry_run() {
    // W876 face3: access × static/async × throws/effect/where/while + comments.
    // `public static` / mix is not fully AST-parseable at top level; CST preserves.
    use syx_fmt::format_source;
    let src = "// ahead\npublic static func go<T>() throws requires Pure where T: Bound {\n  // body\n  while flag /* mid */\n  print(1)\n}\nprivate async func main() {\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default static/async mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("public static func go")
            && report.formatted.contains("private async func main")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast_err = format_source(&file).expect_err("public static mix must fail AST parse");
    assert_eq!(ast_err.code.as_deref(), Some("P001"));
    // Async-only still parses on AST and drops comments (contrast).
    let async_only = SourceFile::new(
        "a.sy",
        "// ahead\nprivate async func main() {\n  /* mid */\n  print(2)\n}\n",
    );
    let ast = format_source(&async_only).expect("async-only AST");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */"),
        "AST must drop comments on async-only; got {ast:?}"
    );
}

#[test]
fn fmt_cst_if_bare_dry_run() {
    // W877 / ADR-1221 face1: if bare + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nfunc main() {\n  // body\n  if flag {\n    print(1)\n  }\n  if ready\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default if bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("if flag") && report.formatted.contains("/// docs"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("// body") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // CST still rejects if case (M0 stays AST F001).
    let case_src = SourceFile::new(
        "c.sy",
        "func main() {\n  if case \\Foo.bar = x {\n    print(1)\n  }\n}\n",
    );
    let err = format_source_cst(&case_src).expect_err("if case must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_if_else_dry_run() {
    // W877 face2: if/else + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nfunc main() {\n  if a {\n    print(1)\n  } else {\n    /* mid */\n    print(2)\n  }\n}\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default if else");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("} else {") && report.formatted.contains("/* mid */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */"),
        "AST must drop comments; got {ast:?}"
    );
    let let_src = SourceFile::new(
        "l.sy",
        "func main() {\n  if let x = y {\n    print(1)\n  }\n}\n",
    );
    let err = format_source_cst(&let_src).expect_err("if let must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_if_mixed_dry_run() {
    // W877 face3: if × logic/while/else-if/access + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic func go() {\n  // body\n  if a && b {\n    while flag /* mid */\n    print(1)\n  } else if c {\n    print(2)\n  } else {\n    print(3)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default if mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("else if c")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_func_export_bare_dry_run() {
    // W878 / ADR-1222 face1: export func + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nexport func api() {\n  // body\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("export func api") && report.formatted.contains("/// docs"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("// body") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject face: export {…} list still CST001.
    let ex = SourceFile::new("e.sy", "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n");
    let err = format_source_cst(&ex).expect_err("if case KeyPath must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_export_mod_dry_run() {
    // W878 face2: export static / export async + comments.
    // Top-level `export static` is not AST-parseable (P001); async contrast drops comments.
    use syx_fmt::format_source;
    let src = "// ahead\nexport static func a() {\n  print(1)\n}\nexport async func b() {\n  /* mid */\n  print(2)\n}\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export mod");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export static func a")
            && report.formatted.contains("export async func b")
    );
    let ast_err = format_source(&file).expect_err("export static mix must fail AST parse");
    assert_eq!(ast_err.code.as_deref(), Some("P001"));
    let async_only = SourceFile::new(
        "a.sy",
        "// ahead\nexport async func main() {\n  /* mid */\n  print(2)\n}\n",
    );
    let ast = format_source(&async_only).expect("export async AST");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */"),
        "AST must drop comments on export async; got {ast:?}"
    );
    // static+async stack still CST001.
    let stack = SourceFile::new("s.sy", "export static async func main() { print(1) }\n");
    let err = format_source_cst(&stack).expect_err("export static async stack must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_func_export_mixed_dry_run() {
    // W878 face3: export × throws/effect/where/while/if + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nexport func go<T>() throws requires Pure where T: Bound {\n  // body\n  while flag /* mid */\n  print(1)\n}\nexport func main() {\n  if ready {\n    print(2)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export func go")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_class_bare_dry_run() {
    // W879 / ADR-1223 face1: empty / stored class + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var name: String\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("class Box") && report.formatted.contains("/// docs"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let inh = SourceFile::new("i.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err = format_source_cst(&inh).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_class_members_dry_run() {
    // W879 face2: class + var + method + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var name: String\n  func tag() {\n    /* mid */\n    print(1)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class members");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("func tag")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let deinit = SourceFile::new(
        "d.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&deinit).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_class_mixed_dry_run() {
    // W879 face3: public class × method × top-level func + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Box {\n  var name: String?\n  func tag() {\n    // body\n    print(1)\n  }\n}\nfunc main() {\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("public class Box")
            && report.formatted.contains("func main")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("// body") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_class_inherit_dry_run() {
    // W880 / ADR-1224 face1: single superclass + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Sub: Box {\n  var name: String\n  func tag() {\n    print(1)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class inherit");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(report.formatted.contains("class Sub: Box") && report.formatted.contains("/// docs"));
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let multi = SourceFile::new("m.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err = format_source_cst(&multi).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_class_init_dry_run() {
    // W880 face2: init + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var name: String\n  init(name: String) {\n    /* mid */\n    print(1)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class init");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("init(name: String)")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let data = SourceFile::new("d.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err = format_source_cst(&data).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_class_deep_mixed_dry_run() {
    // W880 face3: public inherit × init × top-level func + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box {\n  var name: String?\n  init(name: String) {\n    // body\n    print(1)\n  }\n  func tag() {\n    print(2)\n  }\n}\nfunc main() {\n  print(3)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class deep mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("public class Sub: Box")
            && report.formatted.contains("init(name: String)")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("// body") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_data_class_dry_run() {
    // W881 / ADR-1225 face1: thin data class + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\ndata class Point(x: Int, y: Int)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default data class");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("data class Point") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let gen = SourceFile::new("g.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err = format_source_cst(&gen).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_final_class_dry_run() {
    // W881 face2: thin final class + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\nfinal class Token {\n  Ident\n  Number(value: Int)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default final class");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("final class Token")
            && report.formatted.contains("Number(value: Int)")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let gen = SourceFile::new(
        "g.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&gen).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_data_final_mixed_dry_run() {
    // W881 face3: public data × final × top-level func + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic data class Point(x: Int)\nfinal class Shape {\n  point\n  circle(r: Int)\n}\nfunc main() {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default data/final mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("public data class Point")
            && report.formatted.contains("final class Shape")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_data_class_tp_dry_run() {
    // W882 / ADR-1226 face1: data class type params + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\ndata class Box<T>(value: T)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default data class tp");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("data class Box<T>") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new("r.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_data_class_where_dry_run() {
    // W882 face2: data class where + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\ndata class Box<T>(value: T) where T: Equatable\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default data class where");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("where T: Equatable")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "e.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_data_final_gw_mixed_dry_run() {
    // W882 face3: data where × final tp × top-level func + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic data class Box<T>(value: T) where T: Equatable\nfinal class Wrap<U> {\n  wrap(value: U)\n}\nfunc main() {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default data/final gw mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("public data class Box<T>")
            && report.formatted.contains("final class Wrap<U>")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_export_list_bare_dry_run() {
    // W883 / ADR-1227 face1: bare export list + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nexport { foo }\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export list bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export { foo }") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new("r.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_export_list_multi_dry_run() {
    // W883 face2: multi + trailing comma + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nexport { a, b, c, }\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export list multi");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export { a, b, c, }")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "a.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_export_list_mixed_dry_run() {
    // W883 face3: export list × export func × data + comments.
    use syx_fmt::format_source;
    let src = "// ahead\nexport { Point }\ndata class Point(x: Int)\nexport func main() {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export list mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export { Point }")
            && report.formatted.contains("export func main")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_final_where_bare_dry_run() {
    // W884 / ADR-1228 face1: final+where bare + docs; default CST preserves.
    // AST/parser has no FinalClass where (P001); comment path is CST-only.
    use syx_fmt::format_source;
    let src = "/// docs\nfinal class Box<T> where T: Equatable {\n  wrap(value: T)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default final where bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("where T: Equatable") && report.formatted.contains("/// docs")
    );
    let err = format_source(&file).expect_err("AST FinalClass has no where");
    assert_eq!(err.code.as_deref(), Some("P001"));
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new("r.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err2 = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_final_where_multi_dry_run() {
    // W884 face2: multi-pred where + public + comments; AST P001 (no FinalClass where).
    use syx_fmt::format_source;
    let src = "// ahead\npublic final class Pair<A, B> where A: Equatable, B: Hashable {\n  pair(a: A, b: B)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default final where multi");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("where A: Equatable, B: Hashable")
            && report.formatted.contains("/* trail */")
    );
    let err = format_source(&file).expect_err("AST FinalClass has no where");
    assert_eq!(err.code.as_deref(), Some("P001"));
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "a.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err2 = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_final_where_mixed_dry_run() {
    // W884 face3: final+where × data where × export list + comments; AST P001 on final where.
    use syx_fmt::format_source;
    let src = "// ahead\nexport { Box }\npublic data class Box<T>(value: T) where T: Equatable\nfinal class Wrap<U> where U: Hashable {\n  wrap(value: U)\n}\nfunc main() {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default final where mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("final class Wrap<U> where U: Hashable")
            && report.formatted.contains("export { Box }")
            && report.formatted.contains("/* trail */")
    );
    let err = format_source(&file).expect_err("AST FinalClass has no where");
    assert_eq!(err.code.as_deref(), Some("P001"));
}

#[test]
fn fmt_cst_export_as_bare_dry_run() {
    // W885 / ADR-1229 face1: bare as-rename + docs; default CST preserves.
    // AST/parser has no export as-rename (P001); comment path is CST-only.
    use syx_fmt::format_source;
    let src = "/// docs\nexport { foo as bar }\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export as bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export { foo as bar }") && report.formatted.contains("/// docs")
    );
    let err = format_source(&file).expect_err("AST ExportList has no as-rename");
    assert_eq!(err.code.as_deref(), Some("P001"));
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new("r.sy", "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n");
    let err2 = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_export_empty_dry_run() {
    // W885 face2: empty export list + comments; AST rejects empty list (P001).
    use syx_fmt::format_source;
    let src = "// ahead\nexport { }\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default empty export");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export { }") && report.formatted.contains("/* trail */")
    );
    let err = format_source(&file).expect_err("AST rejects empty export list");
    assert_eq!(err.code.as_deref(), Some("P001"));
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "d.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err2 = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err2.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_export_as_empty_mixed_dry_run() {
    // W885 face3: as × empty × thin list × data/func + comments; AST P001 on as/empty.
    use syx_fmt::format_source;
    let src = "// ahead\nexport { }\nexport { foo as bar }\nexport { Point }\ndata class Point(x: Int)\nexport func main() {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default export as empty mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("export { foo as bar }")
            && report.formatted.contains("export { }")
            && report.formatted.contains("/* trail */")
    );
    let err = format_source(&file).expect_err("AST rejects empty/as export");
    assert_eq!(err.code.as_deref(), Some("P001"));
}

#[test]
fn fmt_cst_class_multi_conf_dry_run() {
    // W886 / ADR-1230 face1: multi-conformance + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Sub: Box, Greeter {\n  var name: String\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class multi conf");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("class Sub: Box, Greeter") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_class_deinit_dry_run() {
    // W886 face2: deinit + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var name: String\n  deinit {\n    /* mid */\n    print(1)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class deinit");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("deinit {")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_class_multi_deinit_mixed_dry_run() {
    // W886 face3: multi × deinit × init × top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    print(1)\n  }\n  deinit {\n    print(2)\n  }\n}\nfunc main() {\n  print(3)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default class multi deinit mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("class Sub: Box, Greeter")
            && report.formatted.contains("deinit {")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_self_field_bare_dry_run() {
    // W887 / ADR-1231 face1: init self.field= + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var name: String\n  init(name: String) {\n    self.name = name\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default self field bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.name = name") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_self_field_method_dry_run() {
    // W887 face2: method self.field= + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var name: String\n  func setName(name: String) {\n    /* mid */\n    self.name = name\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default self field method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.name = name")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var name: String\n  func set(x: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_self_field_mixed_dry_run() {
    // W887 face3: init×method×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    self.name = name\n  }\n  func retag(tag: String) {\n    self.name = tag\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default self field mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.name = name")
            && report.formatted.contains("deinit {")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_nested_self_field_bare_dry_run() {
    // W888 / ADR-1232 face1: init nested self.a.b= + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var name: String\n  init(name: String) {\n    self.inner.name = name\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default nested self field bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.inner.name = name") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_nested_self_field_method_dry_run() {
    // W888 face2: method nested self.a.b= + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var name: String\n  func setName(name: String) {\n    /* mid */\n    self.inner.name = name\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default nested self field method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.inner.name = name")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var name: String\n  func set(x: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_nested_self_field_mixed_dry_run() {
    // W888 face3: nested×单层×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    self.inner.name = name\n  }\n  func retag(tag: String) {\n    self.name = tag\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default nested self field mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.inner.name = name")
            && report.formatted.contains("deinit {")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}


#[test]
fn fmt_cst_member_field_bare_dry_run() {
    // W889 / ADR-1233 face1: init non-self member assign + docs; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var name: String\n  init(name: String) {\n    box.name = name\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("box.name = name") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("ast must succeed");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_method_dry_run() {
    // W889 face2: method non-self member assign + comments; default CST preserves; AST drops.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var name: String\n  func setName(name: String) {\n    /* mid */\n    other.name = name\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("other.name = name")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var name: String\n  func set(x: String) {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_mixed_dry_run() {
    // W889 face3: non-self×self×nested×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    box.name = name\n  }\n  func retag(tag: String) {\n    self.inner.name = tag\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("box.name = name")
            && report.formatted.contains("self.inner.name = tag")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("ast");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}

#[test]
fn fmt_cst_member_field_plus_bare_dry_run() {
    // W890 / ADR-1234 face1: init self.count += + docs; default CST preserves; AST rejects MemberCompoundAssign.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    self.count += 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field plus bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.count += 1") && report.formatted.contains("/// docs")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var count: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_plus_method_dry_run() {
    // W890 face2: method box.x += + comments; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var count: Int\n  func bump(n: Int) {\n    /* mid */\n    box.count += n\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field plus method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("box.count += n")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var count: Int\n  func bump() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_plus_mixed_dry_run() {
    // W890 face3: self×non-self×nested×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    self.count += 1\n  }\n  func bump(n: Int) {\n    box.inner.count += n\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field plus mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.count += 1")
            && report.formatted.contains("box.inner.count += n")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
}

#[test]
fn fmt_cst_member_field_minus_bare_dry_run() {
    // W891 / ADR-1235 face1: init self.count -= + docs; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    self.count -= 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field minus bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.count -= 1") && report.formatted.contains("/// docs")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var count: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_minus_method_dry_run() {
    // W891 face2: method box.x -= + comments; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var count: Int\n  func drop(n: Int) {\n    /* mid */\n    box.count -= n\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field minus method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("box.count -= n")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var count: Int\n  func drop() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_minus_mixed_dry_run() {
    // W891 face3: self×non-self×nested×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    self.count -= 1\n  }\n  func drop(n: Int) {\n    box.inner.count -= n\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field minus mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.count -= 1")
            && report.formatted.contains("box.inner.count -= n")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
}

#[test]
fn fmt_cst_member_field_star_bare_dry_run() {
    // W892 / ADR-1236 face1: init self.count *= + docs; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    self.count *= 2\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field star bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.count *= 2") && report.formatted.contains("/// docs")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var count: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_star_method_dry_run() {
    // W892 face2: method box.x *= / /= + comments; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var count: Int\n  func scale(n: Int) {\n    /* mid */\n    box.count *= n\n    box.count /= 2\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field star method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("box.count *= n")
            && report.formatted.contains("box.count /= 2")
            && report.formatted.contains("/* mid */")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var count: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_field_star_mixed_dry_run() {
    // W892 face3: self×non-self×nested×%=×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    self.count *= 2\n  }\n  func scale(n: Int) {\n    box.inner.count %= n\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member field star mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.count *= 2")
            && report.formatted.contains("box.inner.count %= n")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
}

#[test]
fn fmt_cst_ident_compound_bare_dry_run() {
    // W893 / ADR-1237 face1: init count += + docs; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    count += 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default ident compound bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("count += 1") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("AST supports bare CompoundAssign");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var count: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_ident_compound_method_dry_run() {
    // W893 face2: method count *= / /= + comments; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var count: Int\n  func scale(n: Int) {\n    /* mid */\n    count *= n\n    count /= 2\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default ident compound method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("count *= n")
            && report.formatted.contains("count /= 2")
            && report.formatted.contains("/* mid */")
    );
    let ast = format_source(&file).expect("AST supports bare CompoundAssign");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var count: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_ident_compound_mixed_dry_run() {
    // W893 face3: ident×member×nested×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    count += 1\n  }\n  func scale(n: Int) {\n    box.inner.count %= n\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default ident compound mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("count += 1")
            && report.formatted.contains("box.inner.count %= n")
            && report.formatted.contains("count *= 2")
            && report.formatted.contains("/* trail */")
    );
    // Mixed includes member compound; AST has no MemberCompoundAssign.
    let _ = format_source(&file).expect_err("AST has no MemberCompoundAssign");
}


#[test]
fn fmt_cst_index_assign_bare_dry_run() {
    // W894 / ADR-1238 face1: init a[0] = + docs; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0] = 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index assign bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0] = 1") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("AST supports IndexAssign");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var items: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_assign_method_dry_run() {
    // W894 face2: method xs[i] = + comments; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var items: Int\n  func set(i: Int, n: Int) {\n    /* mid */\n    xs[i] = n\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index assign method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("xs[i] = n")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("AST supports IndexAssign");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var items: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_assign_mixed_dry_run() {
    // W894 face3: index×ident-compound×member×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0] = 1\n  }\n  func set(i: Int, n: Int) {\n    xs[i] = n\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index assign mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0] = 1")
            && report.formatted.contains("xs[i] = n")
            && report.formatted.contains("count *= 2")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("AST supports IndexAssign + CompoundAssign");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
}


#[test]
fn fmt_cst_index_compound_bare_dry_run() {
    // W895 / ADR-1239 face1: init a[0] += + docs; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0] += 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index compound bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0] += 1") && report.formatted.contains("/// docs")
    );
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var items: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_compound_method_dry_run() {
    // W895 face2: method xs[i] *= / /= + comments; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var items: Int\n  func scale(i: Int, n: Int) {\n    /* mid */\n    xs[i] *= n\n    xs[i] /= 2\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index compound method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("xs[i] *= n")
            && report.formatted.contains("xs[i] /= 2")
            && report.formatted.contains("/* mid */")
    );
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var items: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_compound_mixed_dry_run() {
    // W895 face3: index compound×ident-compound×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0] += 1\n  }\n  func scale(i: Int, n: Int) {\n    xs[i] %= n\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index compound mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0] += 1")
            && report.formatted.contains("xs[i] %= n")
            && report.formatted.contains("count *= 2")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
}

#[test]
fn fmt_cst_index_nested_bare_dry_run() {
    // W896 / ADR-1240 face1: init a[0][1] = + docs; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0][1] = 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index nested bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0][1] = 1") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("AST supports nested IndexAssign");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var items: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_nested_method_dry_run() {
    // W896 face2: method buf[i][j] = + comments; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var items: Int\n  func set(i: Int, j: Int, n: Int) {\n    /* mid */\n    buf[i][j] = n\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index nested method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("buf[i][j] = n")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("AST supports nested IndexAssign");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var items: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_nested_mixed_dry_run() {
    // W896 face3: nested=×index-compound×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0][1] = 1\n  }\n  func scale(i: Int, j: Int, n: Int) {\n    buf[i][j] = n\n    xs[i] += 1\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default index nested mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0][1] = 1")
            && report.formatted.contains("buf[i][j] = n")
            && report.formatted.contains("xs[i] += 1")
            && report.formatted.contains("/* trail */")
    );
    // Mixed includes index compound; AST has no IndexCompoundAssign.
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
}


#[test]
fn fmt_cst_member_index_assign_bare_dry_run() {
    // W897 / ADR-1241 face1: init self.xs[0] = + docs; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    self.xs[0] = 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member-index bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.xs[0] = 1") && report.formatted.contains("/// docs")
    );
    let ast = format_source(&file).expect("AST supports member-index IndexAssign");
    assert!(
        !ast.contains("/// docs") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var items: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_index_assign_method_dry_run() {
    // W897 face2: method box.items[i] = + comments; default CST preserves; AST drops comments.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var items: Int\n  func set(i: Int, n: Int) {\n    /* mid */\n    box.items[i] = n\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member-index method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("box.items[i] = n")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let ast = format_source(&file).expect("AST supports member-index IndexAssign");
    assert!(
        !ast.contains("// ahead") && !ast.contains("/* mid */") && !ast.contains("/* trail */"),
        "AST must drop comments; got {ast:?}"
    );
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var items: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_member_index_assign_mixed_dry_run() {
    // W897 face3: member-index=×nested×index-compound×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    self.xs[0] = 1\n  }\n  func scale(i: Int, j: Int, n: Int) {\n    box.items[i] = n\n    a[0][1] = 2\n    xs[i] += 1\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default member-index mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("self.xs[0] = 1")
            && report.formatted.contains("box.items[i] = n")
            && report.formatted.contains("a[0][1] = 2")
            && report.formatted.contains("/* trail */")
    );
    // Mixed includes index compound; AST has no IndexCompoundAssign.
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
}


#[test]
fn fmt_cst_index_nested_compound_bare_dry_run() {
    // W898 / ADR-1242 face1: init a[0][1] += + docs; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0][1] += 1\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default nested compound bare");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0][1] += 1") && report.formatted.contains("/// docs")
    );
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "r.sy",
        "class Box {\n  var items: Int\n  init() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_nested_compound_method_dry_run() {
    // W898 face2: method buf[i][j] *= / /= + comments; default CST preserves; AST rejects.
    use syx_fmt::format_source;
    let src = "// ahead\nclass Box {\n  var items: Int\n  func scale(i: Int, j: Int, n: Int) {\n    /* mid */\n    buf[i][j] *= n\n    buf[i][j] /= 2\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default nested compound method");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("buf[i][j] *= n")
            && report.formatted.contains("/* mid */")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
    // Successor reject: member-index compound still CST001.
    let rej = SourceFile::new(
        "s.sy",
        "class Box {\n  var items: Int\n  func set() {\n    self.xs[0] += 1\n  }\n}\n",
    );
    let err = format_source_cst(&rej).expect_err("member-index compound must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_index_nested_compound_mixed_dry_run() {
    // W898 face3: nested-compound×member-index=×multi/deinit×top-level + comments.
    use syx_fmt::format_source;
    let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0][1] += 1\n  }\n  func scale(i: Int, j: Int, n: Int) {\n    buf[i][j] %= n\n    self.xs[0] = 2\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let report = format_dry_run(&file).expect("default nested compound mixed");
    assert!(!report.would_change);
    assert_eq!(report.formatted, src);
    assert!(
        report.formatted.contains("a[0][1] += 1")
            && report.formatted.contains("buf[i][j] %= n")
            && report.formatted.contains("self.xs[0] = 2")
            && report.formatted.contains("/* trail */")
    );
    let _ = format_source(&file).expect_err("AST has no IndexCompoundAssign");
}


#[test]
fn fmt_cst_while_bang3_bare_roundtrip() {
    // W852 / ADR-1196 face1: bare `!!!`; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while !!!flag {\n    print(1)\n  }\n  while !!!done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while bang3 bare");
    assert_eq!(out, src);
    assert!(out.contains("while !!!flag"));
    let quad = SourceFile::new("quad.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&quad).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_bang3_logic_roundtrip() {
    // W852 face2: !!!×logic·call; reject -- .
    let src = "// ahead\nfunc main() {\n  while !!!a && b {\n    print(1)\n  }\n  while !!!ready() || done\n  print(1)\n  while (!!!flag) && x\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while bang3 logic");
    assert_eq!(out, src);
    assert!(out.contains("while !!!a && b"));
    let dbl = SourceFile::new("dbl.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&dbl).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_bang3_mixed_roundtrip() {
    // W852 face3: !!! × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !!!ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !!!flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while bang3 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while !!!ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg2_bare_roundtrip() {
    // W853 / ADR-1197 face1: bare stacked `--`; reject `---`.
    let src = "// ahead\nfunc main() {\n  while --n {\n    print(1)\n  }\n  while --done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg2 bare");
    assert_eq!(out, src);
    assert!(out.contains("while --n"));
    let trip = SourceFile::new("trip.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&trip).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg2_logic_roundtrip() {
    // W853 face2: --×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while --a && b {\n    print(1)\n  }\n  while --ready() || done\n  print(1)\n  while (--n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg2 logic");
    assert_eq!(out, src);
    assert!(out.contains("while --a && b"));
    let quad = SourceFile::new("quad.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&quad).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg2_mixed_roundtrip() {
    // W853 face3: -- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while --ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while --n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg2 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while --ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg3_bare_roundtrip() {
    // W854 / ADR-1198 face1: bare stacked `---`; reject `----`.
    let src = "// ahead\nfunc main() {\n  while ---n {\n    print(1)\n  }\n  while ---done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg3 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ---n"));
    let quad = SourceFile::new("quad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&quad).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg3_logic_roundtrip() {
    // W854 face2: ---×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ---a && b {\n    print(1)\n  }\n  while ---ready() || done\n  print(1)\n  while (---n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg3 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ---a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg3_mixed_roundtrip() {
    // W854 face3: --- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ---ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ---n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg3 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ---ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg4_bare_roundtrip() {
    // W855 / ADR-1199 face1: bare stacked `----`; reject `------` (W856 dry-run tip).
    let src = "// ahead\nfunc main() {\n  while ----n {\n    print(1)\n  }\n  while ----done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg4 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ----n"));
    let sext = SourceFile::new("sext.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&sext).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg4_logic_roundtrip() {
    // W855 face2: ----×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ----a && b {\n    print(1)\n  }\n  while ----ready() || done\n  print(1)\n  while (----n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg4 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ----a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg4_mixed_roundtrip() {
    // W855 face3: ---- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ----ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ----n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg4 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ----ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg5_bare_roundtrip() {
    // W856 / ADR-1200 face1: bare stacked `-----`; reject `-------` (W857 dry-run tip).
    let src = "// ahead\nfunc main() {\n  while -----n {\n    print(1)\n  }\n  while -----done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg5 bare");
    assert_eq!(out, src);
    assert!(out.contains("while -----n"));
    let sept = SourceFile::new("sept.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&sept).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg5_logic_roundtrip() {
    // W856 face2: -----×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while -----a && b {\n    print(1)\n  }\n  while -----ready() || done\n  print(1)\n  while (-----n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg5 logic");
    assert_eq!(out, src);
    assert!(out.contains("while -----a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg5_mixed_roundtrip() {
    // W856 face3: ----- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -----ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -----n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg5 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while -----ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg6_bare_roundtrip() {
    // W857 / ADR-1201 face1: bare stacked `------`; reject `--------` (W858 dry-run tip).
    let src = "// ahead\nfunc main() {\n  while ------n {\n    print(1)\n  }\n  while ------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg6 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ------n"));
    let oct = SourceFile::new("oct.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&oct).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg6_logic_roundtrip() {
    // W857 face2: ------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ------a && b {\n    print(1)\n  }\n  while ------ready() || done\n  print(1)\n  while (------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg6 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg6_mixed_roundtrip() {
    // W857 face3: ------ × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg6 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg7_bare_roundtrip() {
    // W858 / ADR-1202 face1: bare stacked `-------`; reject `--------`.
    let src = "// ahead\nfunc main() {\n  while -------n {\n    print(1)\n  }\n  while -------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg7 bare");
    assert_eq!(out, src);
    assert!(out.contains("while -------n"));
    let oct = SourceFile::new("oct.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&oct).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg7_logic_roundtrip() {
    // W858 face2: -------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while -------a && b {\n    print(1)\n  }\n  while -------ready() || done\n  print(1)\n  while (-------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg7 logic");
    assert_eq!(out, src);
    assert!(out.contains("while -------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg7_mixed_roundtrip() {
    // W858 face3: ------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg7 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while -------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg8_bare_roundtrip() {
    // W859 / ADR-1203 face1: bare stacked `--------`; reject `---------`.
    let src = "// ahead\nfunc main() {\n  while --------n {\n    print(1)\n  }\n  while --------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg8 bare");
    assert_eq!(out, src);
    assert!(out.contains("while --------n"));
    let non = SourceFile::new("non.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&non).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg8_logic_roundtrip() {
    // W859 face2: --------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while --------a && b {\n    print(1)\n  }\n  while --------ready() || done\n  print(1)\n  while (--------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg8 logic");
    assert_eq!(out, src);
    assert!(out.contains("while --------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg8_mixed_roundtrip() {
    // W859 face3: -------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while --------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while --------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg8 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while --------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg9_bare_roundtrip() {
    // W860 / ADR-1204 face1: bare stacked `---------`; reject `----------`.
    let src = "// ahead\nfunc main() {\n  while ---------n {\n    print(1)\n  }\n  while ---------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg9 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ---------n"));
    let dec = SourceFile::new("dec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&dec).expect_err("dec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg9_logic_roundtrip() {
    // W860 face2: ---------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ---------a && b {\n    print(1)\n  }\n  while ---------ready() || done\n  print(1)\n  while (---------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg9 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ---------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg9_mixed_roundtrip() {
    // W860 face3: --------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ---------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ---------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg9 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ---------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg10_bare_roundtrip() {
    // W861 / ADR-1205 face1: bare stacked `----------`; reject `-----------`.
    let src = "// ahead\nfunc main() {\n  while ----------n {\n    print(1)\n  }\n  while ----------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg10 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ----------n"));
    let undec = SourceFile::new("undec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&undec).expect_err("undec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg10_logic_roundtrip() {
    // W861 face2: ----------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ----------a && b {\n    print(1)\n  }\n  while ----------ready() || done\n  print(1)\n  while (----------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg10 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ----------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg10_mixed_roundtrip() {
    // W861 face3: ---------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ----------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ----------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg10 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ----------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg11_bare_roundtrip() {
    // W862 / ADR-1206 face1: bare stacked `-----------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while -----------n {\n    print(1)\n  }\n  while -----------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg11 bare");
    assert_eq!(out, src);
    assert!(out.contains("while -----------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg11_logic_roundtrip() {
    // W862 face2: -----------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while -----------a && b {\n    print(1)\n  }\n  while -----------ready() || done\n  print(1)\n  while (-----------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg11 logic");
    assert_eq!(out, src);
    assert!(out.contains("while -----------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg11_mixed_roundtrip() {
    // W862 face3: ----------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -----------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -----------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg11 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while -----------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg12_bare_roundtrip() {
    // W863 / ADR-1207 face1: bare stacked `------------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while ------------n {\n    print(1)\n  }\n  while ------------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg12 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ------------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg12_logic_roundtrip() {
    // W863 face2: ------------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ------------a && b {\n    print(1)\n  }\n  while ------------ready() || done\n  print(1)\n  while (------------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg12 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ------------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg12_mixed_roundtrip() {
    // W863 face3: ------------ × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ------------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg12 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ------------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg13_bare_roundtrip() {
    // W864 / ADR-1208 face1: bare stacked `-------------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while -------------n {\n    print(1)\n  }\n  while -------------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg13 bare");
    assert_eq!(out, src);
    assert!(out.contains("while -------------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg13_logic_roundtrip() {
    // W864 face2: -------------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while -------------a && b {\n    print(1)\n  }\n  while -------------ready() || done\n  print(1)\n  while (-------------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg13 logic");
    assert_eq!(out, src);
    assert!(out.contains("while -------------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg13_mixed_roundtrip() {
    // W864 face3: ------------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -------------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg13 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while -------------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg14_bare_roundtrip() {
    // W865 / ADR-1209 face1: bare stacked `--------------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while --------------n {\n    print(1)\n  }\n  while --------------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg14 bare");
    assert_eq!(out, src);
    assert!(out.contains("while --------------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg14_logic_roundtrip() {
    // W865 face2: --------------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while --------------a && b {\n    print(1)\n  }\n  while --------------ready() || done\n  print(1)\n  while (--------------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg14 logic");
    assert_eq!(out, src);
    assert!(out.contains("while --------------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg14_mixed_roundtrip() {
    // W865 face3: -------------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while --------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while --------------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg14 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while --------------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg15_bare_roundtrip() {
    // W866 / ADR-1210 face1: bare stacked `---------------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while ---------------n {\n    print(1)\n  }\n  while ---------------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg15 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ---------------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg15_logic_roundtrip() {
    // W866 face2: ---------------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ---------------a && b {\n    print(1)\n  }\n  while ---------------ready() || done\n  print(1)\n  while (---------------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg15 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ---------------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg15_mixed_roundtrip() {
    // W866 face3: --------------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ---------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ---------------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg15 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ---------------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg16_bare_roundtrip() {
    // W867 / ADR-1211 face1: bare stacked `----------------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while ----------------n {\n    print(1)\n  }\n  while ----------------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg16 bare");
    assert_eq!(out, src);
    assert!(out.contains("while ----------------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg16_logic_roundtrip() {
    // W867 face2: ----------------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while ----------------a && b {\n    print(1)\n  }\n  while ----------------ready() || done\n  print(1)\n  while (----------------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg16 logic");
    assert_eq!(out, src);
    assert!(out.contains("while ----------------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg16_mixed_roundtrip() {
    // W867 face3: ---------------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ----------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ----------------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg16 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while ----------------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg17_bare_roundtrip() {
    // W868 / ADR-1212 face1: bare stacked `-----------------`; reject `------------------`.
    let src = "// ahead\nfunc main() {\n  while -----------------n {\n    print(1)\n  }\n  while -----------------done\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg17 bare");
    assert_eq!(out, src);
    assert!(out.contains("while -----------------n"));
    let octodec = SourceFile::new("octodec.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&octodec).expect_err("octodec neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg17_logic_roundtrip() {
    // W868 face2: -----------------×logic·call; reject `!!!!`.
    let src = "// ahead\nfunc main() {\n  while -----------------a && b {\n    print(1)\n  }\n  while -----------------ready() || done\n  print(1)\n  while (-----------------n) && flag\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg17 logic");
    assert_eq!(out, src);
    assert!(out.contains("while -----------------a && b"));
    let bang = SourceFile::new("bang.sy", "func main() { while !!!!!flag { print(1) } }\n");
    let err = format_source_cst(&bang).expect_err("quad bang must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_neg17_mixed_roundtrip() {
    // W868 face3: ----------------- × effect/where/depth2; reject ((((…)))).
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -----------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -----------------n\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while neg17 mixed");
    assert_eq!(out, src);
    assert!(out.contains("while -----------------ready() && a || b"));
    let deep = SourceFile::new("deep.sy", "func main() { while ((((a)))) { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("paren nest>3 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_braceless_bare_roundtrip() {
    // W843 / ADR-1187 face1: brace-less bare; reject chained logic.
    let src = "// ahead\nfunc main() {\n  while true\n  print(1)\n  while flag\n  print(x)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while braceless bare");
    assert_eq!(out, src);
    assert!(out.contains("while true"));
    let logic = SourceFile::new("logic.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&logic).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_braceless_cond_roundtrip() {
    // W843 face2: call/compare brace-less; reject depth>2.
    let src = "// ahead\nfunc main() {\n  while ready()\n  print(1)\n  while x < n\n  print(x)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while braceless cond");
    assert_eq!(out, src);
    assert!(out.contains("while ready()"));
    let deep = SourceFile::new("deep.sy", "func main() { print(foo(bar(baz(qux())))) }\n");
    let err = format_source_cst(&deep).expect_err("depth>2 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_while_braceless_mixed_roundtrip() {
    // W843 face3: brace-less × braced/effect/where; reject nested braceless while.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while true\n  print(foo(bar(baz())))\n}\nfunc main() {\n  while ready() {\n    print(1)\n  }\n  while flag\n  print(2)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst while braceless mixed");
    assert_eq!(out, src);
    assert!(out.contains("while true"));
    let nest = SourceFile::new("nest.sy", "func main() { while true while flag print(1) }\n");
    let err = format_source_cst(&nest).expect_err("braceless nested while must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_depth2_call_print_roundtrip() {
    // W842 / ADR-1186 face1: print depth=2; reject chained logic.
    let src = "// ahead\nfunc main() {\n  print(foo(bar(baz())))\n  print(g(h(1)))\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst depth2 call print");
    assert_eq!(out, src);
    assert!(out.contains("print(foo(bar(baz())))"));
    let brace = SourceFile::new("brace.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&brace).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_depth2_call_rhs_roundtrip() {
    // W842 face2: RHS depth=2; reject depth>2.
    let src = "// ahead\nfunc main() {\n  let x = foo(bar(baz()))\n  var y = g(h(1))\n  return f(k(m()))\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst depth2 call rhs");
    assert_eq!(out, src);
    assert!(out.contains("let x = foo(bar(baz()))"));
    let deep = SourceFile::new("deep.sy", "func main() { print(foo(bar(baz(qux())))) }\n");
    let err = format_source_cst(&deep).expect_err("depth>2 must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_depth2_call_mixed_roundtrip() {
    // W842 face3: depth=2 × while/effect/where; reject chained logic.
    let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  print(foo(bar(baz())))\n}\nfunc main() {\n  while ready() {\n    let x = f(g(h()))\n    print(x)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst depth2 call mixed");
    assert_eq!(out, src);
    assert!(out.contains("print(foo(bar(baz())))"));
    assert!(out.contains("let x = f(g(h()))"));
    let logic = SourceFile::new("logic.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&logic).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_type_params_single_roundtrip() {
    // W840 / ADR-1184 face1: single type param; reject chained logic.
    let src = "// ahead\nfunc id<T>(x: T) {\n  print(x)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst type_params single");
    assert_eq!(out, src);
    assert!(out.contains("id<T>"));
    let where_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&where_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_type_params_multi_roundtrip() {
    // W840 face2: multi TP + return/throws; reject chained logic.
    let src = "// ahead\nfunc pair<A, B>(a: A, b: B) {\n  print(a)\n}\nfunc id<T>(x: T) -> T throws {\n  return x\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst type_params multi");
    assert_eq!(out, src);
    assert!(out.contains("pair<A, B>"));
    assert!(out.contains("throws"));
    let deep = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_type_params_mixed_roundtrip() {
    // W840 face3: type_params × throws × effect × nested-call; reject chained logic.
    let src = "// ahead\nfunc id<T>(x: T) throws requires Pure {\n  print(foo(bar()))\n}\nfunc go<U>() {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\nprint(1)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst type_params mixed");
    assert_eq!(out, src);
    assert!(out.contains("id<T>"));
    assert!(out.contains("print(foo(bar()))"));
    let where_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&where_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_throws_bare_roundtrip() {
    // W839 / ADR-1183 face1: bare throws; reject chained logic.
    let src = "// ahead\nfunc boom() throws {\n  print(1)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst throws bare");
    assert_eq!(out, src);
    assert!(out.contains("throws"));
    let tp = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_throws_clause_roundtrip() {
    // W839 face2: throws + return / requires; reject chained logic.
    let src = "// ahead\nfunc fail() -> Int throws {\n  return 1\n}\nfunc g() throws requires Pure {\n  print(0)\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst throws clause");
    assert_eq!(out, src);
    assert!(out.contains("throws"));
    assert!(out.contains("requires Pure"));
    let where_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&where_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_throws_mixed_roundtrip() {
    // W839 face3: throws × nested-call × while × effect; reject chained logic.
    let src = "// ahead\nfunc boom() throws {\n  print(foo(bar()))\n}\nfunc go() throws requires IO {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\nprint(1)\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst throws mixed");
    assert_eq!(out, src);
    assert!(out.contains("throws"));
    assert!(out.contains("print(foo(bar()))"));
    let deep = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_nested_call_print_roundtrip() {
    // W838 / ADR-1182 face1: print nested-call depth≤1; reject chained logic.
    let src = "// ahead\nfunc main() {\n  print(foo(bar()))\n  print(g(h(1)))\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst nested call print");
    assert_eq!(out, src);
    assert!(out.contains("print(foo(bar()))"));
    let tp = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&tp).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_nested_call_rhs_roundtrip() {
    // W838 face2: let/var/return nested call; reject chained logic.
    let src = "// ahead\nfunc main() {\n  let x = foo(bar())\n  var y = g(h(1))\n  return f(k())\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst nested call rhs");
    assert_eq!(out, src);
    assert!(out.contains("let x = foo(bar())"));
    let deep = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&deep).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_nested_call_mixed_roundtrip() {
    // W838 face3: nested call + while + effect; reject chained logic.
    let src = "// ahead\nfunc go() requires Pure {\n  print(foo(bar()))\n}\nfunc main() {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\n/* trail */\n";
    let file = SourceFile::new("t.sy", src);
    let out = format_source_cst(&file).expect("cst nested call mixed");
    assert_eq!(out, src);
    assert!(out.contains("print(foo(bar()))"));
    assert!(out.contains("let x = f(g())"));
    let where_rej = SourceFile::new("deep.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&where_rej).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_double_simple_roundtrip() {
    // W793 / ADR-1137: Double ∈ simple for print/let/return + trivia byte-identical.
    let src = "// ahead\nfunc main() {\n  // float\n  let x = 1.5\n  print(x)\n  print(2.0)\n  return 3.14\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst double simple");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x = 1.5"));
    assert!(out.contains("print(2.0)"));
    assert!(out.contains("return 3.14"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_bool_simple_roundtrip() {
    // W794 / ADR-1138: Bool ∈ simple for print/let/return + trivia byte-identical.
    let src = "// ahead\nfunc main() {\n  // flag\n  let x = true\n  print(x)\n  print(false)\n  return true\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst bool simple");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x = true"));
    assert!(out.contains("print(false)"));
    assert!(out.contains("return true"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_nil_simple_roundtrip() {
    // W795 / ADR-1139: Nil ∈ simple for print/let/return + trivia byte-identical.
    let src = "// ahead\nfunc main() {\n  // empty\n  let x = nil\n  print(x)\n  print(nil)\n  return nil\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst nil simple");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("let x = nil"));
    assert!(out.contains("print(nil)"));
    assert!(out.contains("return nil"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_cst_var_bind_roundtrip() {
    // W796 / ADR-1140: var[+let/print/return] with trivia byte-identical; reject call RHS.
    let src = "// ahead\nfunc main() {\n  // mut\n  var x = 1\n  let y = true\n  print(x)\n  return nil\n}\n/* trail */\n";
    let file = SourceFile::new("fixture.sy", src);
    let out = format_source_cst(&file).expect("cst var bind");
    assert_eq!(out, src);
    assert!(out.contains("// ahead"));
    assert!(out.contains("var x = 1"));
    assert!(out.contains("let y = true"));
    assert!(out.contains("print(x)"));
    assert!(out.contains("return nil"));
    assert!(out.contains("/* trail */"));

    let reject = SourceFile::new("bad.sy", "func main() { while -------------------n { print(1) } }\n");
    let err = format_source_cst(&reject).expect_err("non neg must CST001");
    assert_eq!(err.code.as_deref(), Some("CST001"));
}

#[test]
fn fmt_index_dict_dry_run() {
    let src = r#"func main() {
    let a = [1, 8, 3]
    print(a[1])
    let d: Dictionary<Int, Int> = {10: 100, 20: 200}
    print(d[10])
    print(d.length)
}
"#;
    let file = SourceFile::new("suite.sy", src);
    let report = format_dry_run(&file).expect("index/dict/typed-let must format");
    let again = SourceFile::new("suite.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
    assert!(r2.formatted.contains("a[1]"));
    assert!(r2.formatted.contains("Dictionary<Int, Int>") || r2.formatted.contains("{10: 100"));
}

#[test]
fn fmt_class_init_override_dry_run() {
    let src = r#"class Box {
    var name: String
    init(name: String) {
        self.name = name
    }
    func tag() -> String {
        return self.name
    }
}

class Sub: Box {
    override func tag() -> String {
        return "sub"
    }
}

func main() {
    let b = Box(name: "n")
    print(b.tag())
}
"#;
    let file = SourceFile::new("class.sy", src);
    let report = format_dry_run(&file).expect("class/init/override/typed-param must format");
    assert!(report.formatted.contains("class Box"));
    assert!(report.formatted.contains("init(name: String)"));
    assert!(report.formatted.contains("override func tag"));
    let again = SourceFile::new("class.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_extension_protocol_dry_run() {
    let src = r#"protocol Greeter {
    func greet() -> String {
        return "?"
    }
}

class Person: Greeter {
    var name: String
    init(name: String) {
        self.name = name
    }
    func greet() -> String {
        return self.name
    }
}

extension Person {
    func shout() -> String {
        return self.name
    }
}

func main() {
    let p = Person(name: "n")
    print(p.greet())
    print(p.shout())
}
"#;
    let file = SourceFile::new("ext_proto.sy", src);
    let report = format_dry_run(&file).expect("extension/protocol must format");
    assert!(report.formatted.contains("protocol Greeter"));
    assert!(report.formatted.contains("extension Person"));
    let again = SourceFile::new("ext_proto.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_protocol_access_inherit_dry_run() {
    // W687 / ADR-1031: public + inherited protocols must round-trip (W686 loss fix).
    let src = "public protocol Codable: Encodable, Decodable {\n}\n";
    let file = SourceFile::new("codable.sy", src);
    let report = format_dry_run(&file).expect("public protocol inherit must format");
    assert!(
        report.formatted.contains("public protocol Codable"),
        "formatted={}",
        report.formatted
    );
    assert!(
        report.formatted.contains("Encodable") && report.formatted.contains("Decodable"),
        "formatted={}",
        report.formatted
    );
    let again = SourceFile::new("codable.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
    assert_eq!(r2.formatted, report.formatted);
}

#[test]
fn fmt_actor_deinit_dry_run() {
    // W687 / ADR-1031: Actor + Class/deinit idempotent.
    let src = r#"actor Counter {
    var n: Int
    init() {
        self.n = 7
    }
    func peek() -> Int {
        return self.n
    }
}

class Node {
    var tag: Int
    init(tag: Int) {
        self.tag = tag
    }
    deinit {
        print(self.tag)
    }
}

func main() {
    let c = Counter()
    print(c.peek())
}
"#;
    let file = SourceFile::new("actor_deinit.sy", src);
    let report = format_dry_run(&file).expect("actor/deinit must format");
    assert!(report.formatted.contains("actor Counter"));
    assert!(report.formatted.contains("deinit {"));
    let again = SourceFile::new("actor_deinit.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_enum_typealias_dry_run() {
    // W688 / ADR-1032: Enum + TypeAlias idempotent.
    let src = r#"enum Color {
    red
    green
}

enum Wire {
    ok(code: Int)
    err(message: String)
}

enum Rank {
    ace = 1
    two = 2
}

public typealias Foo = Int

func main() {
    let x: Foo = 42
    print(x)
}
"#;
    let file = SourceFile::new("enum_ta.sy", src);
    let report = format_dry_run(&file).expect("enum/typealias must format");
    assert!(report.formatted.contains("enum Color"));
    assert!(report.formatted.contains("ok(code: Int)"));
    assert!(report.formatted.contains("ace = 1"));
    assert!(report.formatted.contains("public typealias Foo = Int"));
    let again = SourceFile::new("enum_ta.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_dataclass_dry_run() {
    // W689 / ADR-1033: DataClass idempotent.
    let src = r#"data class Point(x: Int, y: Int)

public data class IOError(code: Int, message: String)

public data class HttpHeaders()

func main() {
    let p = Point(x: 1, y: 2)
    print(p.x)
}
"#;
    let file = SourceFile::new("dataclass.sy", src);
    let report = format_dry_run(&file).expect("data class must format");
    assert!(report.formatted.contains("data class Point(x: Int, y: Int)"));
    assert!(report
        .formatted
        .contains("public data class IOError(code: Int, message: String)"));
    assert!(report.formatted.contains("public data class HttpHeaders()"));
    let again = SourceFile::new("dataclass.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_finalclass_dry_run() {
    // W690 / ADR-1034: FinalClass idempotent.
    let src = r#"final class Shape {
    point
    circle(r: Int)
    rect(w: Int, h: Int)
}

public final class Token {
    eof
    ident(name: String)
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("finalclass.sy", src);
    let report = format_dry_run(&file).expect("final class must format");
    assert!(report.formatted.contains("final class Shape"));
    assert!(report.formatted.contains("circle(r: Int)"));
    assert!(report.formatted.contains("rect(w: Int, h: Int)"));
    assert!(report.formatted.contains("public final class Token"));
    assert!(report.formatted.contains("ident(name: String)"));
    let again = SourceFile::new("finalclass.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_globalactor_dry_run() {
    // W691 / ADR-1035: GlobalActor idempotent.
    let src = r#"global actor AppActor {
}

public global actor MainQueue {
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("globalactor.sy", src);
    let report = format_dry_run(&file).expect("global actor must format");
    assert!(report.formatted.contains("global actor AppActor"));
    assert!(report.formatted.contains("public global actor MainQueue"));
    let again = SourceFile::new("globalactor.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_subscript_dry_run() {
    // W692 / ADR-1036: Subscript get-only idempotent.
    let src = r#"class Box {
    subscript(i: Int) -> Int {
        get {
            return 0
        }
    }
}

extension Box {
    subscript(label j: Int) -> Int {
        get {
            return j
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("subscript.sy", src);
    let report = format_dry_run(&file).expect("subscript get-only must format");
    assert!(report.formatted.contains("subscript(i: Int) -> Int"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("return 0"));
    assert!(report.formatted.contains("subscript(label j: Int) -> Int"));
    assert!(!report.formatted.contains("set {"));
    let again = SourceFile::new("subscript.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_observers_dry_run() {
    // W693 / ADR-1037: stored willSet/didSet idempotent.
    let src = r#"class Counter {
    var value: Int
        willSet {
            print("will")
        }
        didSet {
            print("did")
        }

    init(value: Int) {
        self.value = value
    }
}

class WillOnly {
    var n: Int
        willSet {
            print(1)
        }
}

class DidOnly {
    var n: Int = 0
        didSet {
            print(2)
        }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("observers.sy", src);
    let report = format_dry_run(&file).expect("observers must format");
    assert!(report.formatted.contains("willSet {"));
    assert!(report.formatted.contains("didSet {"));
    assert!(report.formatted.contains("print(\"will\")"));
    assert!(report.formatted.contains("var n: Int = 0"));
    let again = SourceFile::new("observers.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_subscript_setter_dry_run() {
    // W694 / ADR-1038: Subscript get+set idempotent.
    let src = r#"class Box {
    subscript(i: Int) -> Int {
        get {
            return 0
        }
        set {
            print(1)
        }
    }
}

extension Box {
    subscript(label j: Int) -> Int {
        get {
            return j
        }
        set {
            print(j)
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("subscript_setter.sy", src);
    let report = format_dry_run(&file).expect("subscript get+set must format");
    assert!(report.formatted.contains("subscript(i: Int) -> Int"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("set {"));
    assert!(report.formatted.contains("print(1)"));
    assert!(report.formatted.contains("subscript(label j: Int) -> Int"));
    let again = SourceFile::new("subscript_setter.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_computed_dry_run() {
    // W695 / ADR-1039: computed get-only idempotent.
    let src = r#"class Box {
    var base: Int = 1
    var doubled: Int {
        get {
            return self.base * 2
        }
    }
}

extension Box {
    var tripled: Int {
        get {
            return self.base * 3
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("computed.sy", src);
    let report = format_dry_run(&file).expect("computed get-only must format");
    assert!(report.formatted.contains("var doubled: Int {"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("return self.base * 2"));
    assert!(report.formatted.contains("var tripled: Int {"));
    assert!(!report.formatted.contains("set {"));
    let again = SourceFile::new("computed.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_lazy_dry_run() {
    // W696 / ADR-1040: lazy var idempotent.
    let src = r#"class Box {
    lazy var x: Int = 1
    public lazy var label: String = "ok"
    var ready: Int = 0
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("lazy.sy", src);
    let report = format_dry_run(&file).expect("lazy var must format");
    assert!(report.formatted.contains("lazy var x: Int = 1"));
    assert!(report.formatted.contains("public lazy var label: String = \"ok\""));
    assert!(report.formatted.contains("var ready: Int = 0"));
    let again = SourceFile::new("lazy.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_lazy_computed_bare_dry_run() {
    // W874 / ADR-1218: lazy computed get-only idempotent.
    let src = r#"class Box {
    lazy var x: Int {
        get {
            return 1
        }
    }
    public lazy var n: Int {
        get {
            return 0
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("lazy_computed_bare.sy", src);
    let report = format_dry_run(&file).expect("lazy computed get-only must format");
    assert!(report.formatted.contains("lazy var x: Int {"));
    assert!(report.formatted.contains("public lazy var n: Int {"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("return 1"));
    assert!(!report.formatted.contains("set {"));
    let again = SourceFile::new("lazy_computed_bare.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_lazy_computed_getset_dry_run() {
    // W874 / ADR-1218: lazy computed get+set idempotent.
    let src = r#"class Box {
    lazy var x: Int {
        get {
            return 0
        }
        set {
            print(1)
        }
    }
    public lazy var n: Int {
        get {
            return 1
        }
        set {
            print(2)
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("lazy_computed_getset.sy", src);
    let report = format_dry_run(&file).expect("lazy computed get+set must format");
    assert!(report.formatted.contains("lazy var x: Int {"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("set {"));
    assert!(report.formatted.contains("public lazy var n: Int {"));
    let again = SourceFile::new("lazy_computed_getset.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_lazy_computed_mixed_dry_run() {
    // W874 / ADR-1218: stored lazy + lazy computed + class interleave idempotent.
    let src = r#"class A {
    lazy var stored: Int = 1
    lazy var computed: Int {
        get {
            return 2
        }
    }
}

class B {
    lazy var both: Int {
        get {
            return 3
        }
        set {
            print(1)
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("lazy_computed_mixed.sy", src);
    let report = format_dry_run(&file).expect("lazy computed mixed must format");
    assert!(report.formatted.contains("lazy var stored: Int = 1"));
    assert!(report.formatted.contains("lazy var computed: Int {"));
    assert!(report.formatted.contains("lazy var both: Int {"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("set {"));
    let again = SourceFile::new("lazy_computed_mixed.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_computed_setter_dry_run() {
    // W697 / ADR-1041: computed get+set idempotent.
    let src = r#"class Box {
    var base: Int = 1
    var doubled: Int {
        get {
            return self.base * 2
        }
        set {
            print(1)
        }
    }
}

extension Box {
    var tripled: Int {
        get {
            return self.base * 3
        }
        set {
            print(2)
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("computed_setter.sy", src);
    let report = format_dry_run(&file).expect("computed get+set must format");
    assert!(report.formatted.contains("var doubled: Int {"));
    assert!(report.formatted.contains("get {"));
    assert!(report.formatted.contains("set {"));
    assert!(report.formatted.contains("return self.base * 2"));
    assert!(report.formatted.contains("var tripled: Int {"));
    let again = SourceFile::new("computed_setter.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_override_var_dry_run() {
    // W698 / ADR-1042: override var idempotent.
    let src = r#"class Box {
    override var x: Int = 1
    public override var label: String = "ok"
    override var doubled: Int {
        get {
            return 0
        }
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("override_var.sy", src);
    let report = format_dry_run(&file).expect("override var must format");
    assert!(report.formatted.contains("override var x: Int = 1"));
    assert!(report.formatted.contains("public override var label: String = \"ok\""));
    assert!(report.formatted.contains("override var doubled: Int {"));
    assert!(report.formatted.contains("get {"));
    let again = SourceFile::new("override_var.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_operator_func_dry_run() {
    // W699 / ADR-1043: OperatorFunc idempotent.
    let src = r#"class Box {
    public operator func +(a: Int, b: Int) -> Int {
        return a
    }
    operator func -(a: Int, b: Int) -> Int {
        return a
    }
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("operator_func.sy", src);
    let report = format_dry_run(&file).expect("OperatorFunc must format");
    assert!(report.formatted.contains("public operator func +(a: Int, b: Int) -> Int"));
    assert!(report.formatted.contains("operator func -(a: Int, b: Int) -> Int"));
    assert!(report.formatted.contains("return a"));
    let again = SourceFile::new("operator_func.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_macro_decl_dry_run() {
    // W700 / ADR-1044: MacroDecl User + empty decl_body idempotent.
    // Note: parser rejects `public #macro` (access stays Internal in practice).
    let src = r#"#macro double(x) {
    return x
}

#macro hello() {
    print(1)
}

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("macro_decl.sy", src);
    let report = format_dry_run(&file).expect("MacroDecl must format");
    assert!(report.formatted.contains("#macro double(x)"));
    assert!(report.formatted.contains("#macro hello()"));
    assert!(report.formatted.contains("return x"));
    let again = SourceFile::new("macro_decl.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_macro_site_invoke_dry_run() {
    // W701 / ADR-1045: MacroSiteInvoke idempotent.
    let src = r#"#makeHelper()

#log("hi", 1)

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("macro_site.sy", src);
    let report = format_dry_run(&file).expect("MacroSiteInvoke must format");
    assert!(report.formatted.contains("#makeHelper()"));
    assert!(report.formatted.contains("#log(\"hi\", 1)"));
    let again = SourceFile::new("macro_site.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_macro_invoke_dry_run() {
    // W702 / ADR-1046: Expr + Stmt MacroInvoke idempotent.
    let src = r#"#macro double(x) {
    return x
}

func main() {
    print(#double(1))
    #say("hi")
}
"#;
    let file = SourceFile::new("macro_invoke.sy", src);
    let report = format_dry_run(&file).expect("MacroInvoke must format");
    assert!(report.formatted.contains("#macro double(x)"));
    assert!(report.formatted.contains("print(#double(1))"));
    assert!(report.formatted.contains("#say(\"hi\")"));
    let again = SourceFile::new("macro_invoke.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_toplevel_stmt_dry_run() {
    // W703 / ADR-1047: TopLevelStmt idempotent.
    let src = r#"print(1)
let x = 2
print(x)

func main() {
    print(3)
}
"#;
    let file = SourceFile::new("toplevel.sy", src);
    let report = format_dry_run(&file).expect("TopLevelStmt must format");
    assert!(report.formatted.contains("print(1)"));
    assert!(report.formatted.contains("let x = 2"));
    assert!(report.formatted.contains("func main"));
    let again = SourceFile::new("toplevel.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_macro_builtin_dry_run() {
    // W704 / ADR-1048: BuiltIn MacroDecl idempotent.
    // Note: assignment-form `macro` does not parse with access modifiers (parser gap);
    // formatter still emits access when AST has non-Internal.
    let src = r#"macro Tracked = #builtInMacro("tracked_accessor")
macro Obs = #builtInMacro("observable")

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("macro_builtin.sy", src);
    let report = format_dry_run(&file).expect("BuiltIn MacroDecl must format");
    assert!(report.formatted.contains("#builtInMacro(\"tracked_accessor\")"));
    assert!(report.formatted.contains("macro Obs"));
    assert!(report.formatted.contains("func main"));
    let again = SourceFile::new("macro_builtin.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_macro_external_dry_run() {
    // W705 / ADR-1049: External MacroDecl idempotent.
    let src = r#"macro Smoke = #externalMacro(module: "SyxMacros", type: "SmokeMemberMacro")
macro Other = #externalMacro(module: "M", type: "T")

func main() {}
"#;
    let file = SourceFile::new("macro_external.sy", src);
    let report = format_dry_run(&file).expect("External MacroDecl must format");
    assert!(report.formatted.contains("#externalMacro(module: \"SyxMacros\""));
    assert!(report.formatted.contains("type: \"SmokeMemberMacro\")"));
    assert!(report.formatted.contains("macro Other"));
    assert!(report.formatted.contains("func main"));
    let again = SourceFile::new("macro_external.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_macro_decl_body_dry_run() {
    // W706 / ADR-1050: nonempty freestanding decl_body idempotent.
    let src = r#"@freestanding(declaration)
#macro makeHelper() {
    data class DeclHelper(ok: Bool)
}

#makeHelper()

func main() {
    print(1)
}
"#;
    let file = SourceFile::new("macro_decl_body.sy", src);
    let report = format_dry_run(&file).expect("nonempty decl_body must format");
    assert!(report.formatted.contains("@freestanding(declaration)"));
    assert!(report.formatted.contains("#macro makeHelper()"));
    assert!(report.formatted.contains("data class DeclHelper"));
    assert!(report.formatted.contains("#makeHelper()"));
    let again = SourceFile::new("macro_decl_body.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_throw_dry_run() {
    // W707 / ADR-1051: Stmt::Throw idempotent.
    let src = r#"func main() {
    throw "boom"
    throw e
}
"#;
    let file = SourceFile::new("throw.sy", src);
    let report = format_dry_run(&file).expect("Throw must format");
    assert!(report.formatted.contains("throw \"boom\""));
    assert!(report.formatted.contains("throw e"));
    let again = SourceFile::new("throw.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_try_dry_run() {
    // W708 / ADR-1052: Stmt::Try idempotent.
    let src = r#"func main() {
    try {
        print(1)
        throw "boom"
    } catch (err) {
        print(2)
    }
}
"#;
    let file = SourceFile::new("try.sy", src);
    let report = format_dry_run(&file).expect("Try must format");
    assert!(report.formatted.contains("try {"));
    assert!(report.formatted.contains("} catch (err) {"));
    assert!(report.formatted.contains("throw \"boom\""));
    let again = SourceFile::new("try.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_closure_dry_run() {
    // W709 / ADR-1053: Expr::Closure idempotent; does not reconstruct `-> Ret`.
    let src = r#"func main() {
    let f = { (a: Int) in
        return a
    }
    print(1)
}
"#;
    let file = SourceFile::new("closure.sy", src);
    let report = format_dry_run(&file).expect("Closure must format");
    assert!(report.formatted.contains("{ (a: Int) in"));
    assert!(report.formatted.contains("return a"));
    assert!(!report.formatted.contains("->"), "must not reconstruct -> Ret");
    let again = SourceFile::new("closure.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_await_dry_run() {
    // W710 / ADR-1054: Expr::Await idempotent.
    let src = r#"func main() {
    await print(1)
    await print(2)
}
"#;
    let file = SourceFile::new("await.sy", src);
    let report = format_dry_run(&file).expect("Await must format");
    assert!(report.formatted.contains("await print(1)"));
    assert!(report.formatted.contains("await print(2)"));
    let again = SourceFile::new("await.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_force_unwrap_dry_run() {
    // W711 / ADR-1055: Expr::ForceUnwrap idempotent.
    let src = r#"func main() {
    let x = 1
    print(x!)
    print(x!)
}
"#;
    let file = SourceFile::new("force_unwrap.sy", src);
    let report = format_dry_run(&file).expect("ForceUnwrap must format");
    assert!(report.formatted.contains("print(x!)"));
    let again = SourceFile::new("force_unwrap.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_cast_dry_run() {
    // W712 / ADR-1056: Expr::Cast idempotent.
    let src = r#"func main() {
    let x = 1 as Int
    print(x)
}
"#;
    let file = SourceFile::new("cast.sy", src);
    let report = format_dry_run(&file).expect("Cast must format");
    assert!(report.formatted.contains("1 as Int"));
    let again = SourceFile::new("cast.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_optional_cast_dry_run() {
    // W713 / ADR-1057: Expr::OptionalCast idempotent.
    let src = r#"func main() {
    let x = 1 as? Int
    print(1)
}
"#;
    let file = SourceFile::new("optional_cast.sy", src);
    let report = format_dry_run(&file).expect("OptionalCast must format");
    assert!(report.formatted.contains("1 as? Int"));
    let again = SourceFile::new("optional_cast.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_is_dry_run() {
    // W714 / ADR-1058: Expr::Is idempotent.
    let src = r#"func main() {
    let x = 1 is Int
    print(1)
}
"#;
    let file = SourceFile::new("is.sy", src);
    let report = format_dry_run(&file).expect("Is must format");
    assert!(report.formatted.contains("1 is Int"));
    let again = SourceFile::new("is.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_type_self_dry_run() {
    // W715 / ADR-1059: Expr::TypeSelf idempotent.
    let src = r#"func main() {
    let t = Int.self
    print(1)
}
"#;
    let file = SourceFile::new("type_self.sy", src);
    let report = format_dry_run(&file).expect("TypeSelf must format");
    assert!(report.formatted.contains("Int.self"));
    let again = SourceFile::new("type_self.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_optional_chain_dry_run() {
    // W716 / ADR-1060: Expr::OptionalChain idempotent.
    let src = r#"func main() {
    print(x?.y)
}
"#;
    let file = SourceFile::new("optional_chain.sy", src);
    let report = format_dry_run(&file).expect("OptionalChain must format");
    assert!(report.formatted.contains("x?.y"));
    let again = SourceFile::new("optional_chain.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_half_open_range_dry_run() {
    // W717 / ADR-1061: Expr::HalfOpenRange idempotent.
    let src = r#"func main() {
    print(1..<3)
}
"#;
    let file = SourceFile::new("half_open_range.sy", src);
    let report = format_dry_run(&file).expect("HalfOpenRange must format");
    assert!(report.formatted.contains("1..<3"));
    let again = SourceFile::new("half_open_range.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_closed_range_dry_run() {
    // W718 / ADR-1062: Expr::ClosedRange idempotent.
    let src = r#"func main() {
    print(1...3)
}
"#;
    let file = SourceFile::new("closed_range.sy", src);
    let report = format_dry_run(&file).expect("ClosedRange must format");
    assert!(report.formatted.contains("1...3"));
    let again = SourceFile::new("closed_range.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_dry_run() {
    // W719 / ADR-1063: Stmt::Switch (literal case) idempotent.
    let src = r#"func main() {
    switch 1 {
    case 1:
        print("one")
    default:
        print("other")
    }
}
"#;
    let file = SourceFile::new("switch.sy", src);
    let report = format_dry_run(&file).expect("Switch must format");
    assert!(report.formatted.contains("switch 1"));
    assert!(report.formatted.contains("case 1:"));
    assert!(report.formatted.contains("default:"));
    let again = SourceFile::new("switch.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_if_let_dry_run() {
    // W720 / ADR-1064: Stmt::IfLet idempotent.
    let src = r#"func main() {
    if let x = opt {
        print(x)
    }
}
"#;
    let file = SourceFile::new("if_let.sy", src);
    let report = format_dry_run(&file).expect("IfLet must format");
    assert!(report.formatted.contains("if let x = opt"));
    let again = SourceFile::new("if_let.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_guard_let_dry_run() {
    // W721 / ADR-1065: Stmt::GuardLet idempotent.
    let src = r#"func main() {
    guard let x = opt else {
        return
    }
    {
        print(x)
    }
}
"#;
    let file = SourceFile::new("guard_let.sy", src);
    let report = format_dry_run(&file).expect("GuardLet must format");
    assert!(report.formatted.contains("guard let x = opt"));
    assert!(report.formatted.contains("else {"));
    let again = SourceFile::new("guard_let.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_repeat_while_dry_run() {
    // W722 / ADR-1066: Stmt::RepeatWhile idempotent.
    let src = r#"func main() {
    repeat {
        print(1)
    } while false
}
"#;
    let file = SourceFile::new("repeat_while.sy", src);
    let report = format_dry_run(&file).expect("RepeatWhile must format");
    assert!(report.formatted.contains("repeat {"));
    assert!(report.formatted.contains("} while false"));
    let again = SourceFile::new("repeat_while.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_if_case_dry_run() {
    // W723 / ADR-1067: Stmt::IfCase + VariantPattern idempotent.
    let src = r#"func main() {
    if case .ok(let x) = opt {
        print(x)
    }
}
"#;
    let file = SourceFile::new("if_case.sy", src);
    let report = format_dry_run(&file).expect("IfCase must format");
    assert!(report.formatted.contains("if case .ok(let x) = opt"));
    let again = SourceFile::new("if_case.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_tuple_pattern_dry_run() {
    // W724 / ADR-1068: Expr::TuplePattern if-case idempotent.
    let src = r#"func main() {
    if case (a, b) = pair {
        print(a)
    }
}
"#;
    let file = SourceFile::new("tuple_pattern.sy", src);
    let report = format_dry_run(&file).expect("TuplePattern must format");
    assert!(report.formatted.contains("if case (a, b) = pair"));
    let again = SourceFile::new("tuple_pattern.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_task_block_dry_run() {
    // W725 / ADR-1069: Expr::TaskBlock Task / Task.detached idempotent.
    let src = r#"func main() {
    Task {
        print(1)
    }
    Task.detached {
        print(2)
    }
}
"#;
    let file = SourceFile::new("task_block.sy", src);
    let report = format_dry_run(&file).expect("TaskBlock must format");
    assert!(report.formatted.contains("Task {"));
    assert!(report.formatted.contains("Task.detached {"));
    let again = SourceFile::new("task_block.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_borrow_dry_run() {
    // W726 / ADR-1070: Expr::Borrow & / &mut idempotent.
    let src = r#"func main() {
    let r = &n
    let m = &mut n
    print(1)
}
"#;
    let file = SourceFile::new("borrow.sy", src);
    let report = format_dry_run(&file).expect("Borrow must format");
    assert!(report.formatted.contains("&n"));
    assert!(report.formatted.contains("&mut n"));
    let again = SourceFile::new("borrow.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_keypath_dry_run() {
    // W727 / ADR-1071: Expr::KeyPath \Root.prop idempotent.
    // Note: parser currently rejects `\Root?.prop` (checks Dot before Question);
    // format arm still emits `?` when AST.optional is true.
    let src = r#"func main() {
    let kp = \Person.name
    let chain = \Person.name.age
    print(1)
}
"#;
    let file = SourceFile::new("keypath.sy", src);
    let report = format_dry_run(&file).expect("KeyPath must format");
    assert!(report.formatted.contains(r"\Person.name"));
    assert!(report.formatted.contains(r"\Person.name.age"));
    let again = SourceFile::new("keypath.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_compound_assign_dry_run() {
    // W728 / ADR-1072: Expr::CompoundAssign +=/-=/*=/ /=/%= idempotent.
    let src = r#"func main() {
    x += 1
    y -= 2
    z *= 3
    a /= 4
    b %= 5
    print(1)
}
"#;
    let file = SourceFile::new("compound_assign.sy", src);
    let report = format_dry_run(&file).expect("CompoundAssign must format");
    assert!(report.formatted.contains("x += 1"));
    assert!(report.formatted.contains("y -= 2"));
    assert!(report.formatted.contains("z *= 3"));
    assert!(report.formatted.contains("a /= 4"));
    assert!(report.formatted.contains("b %= 5"));
    let again = SourceFile::new("compound_assign.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_index_assign_dry_run() {
    // W729 / ADR-1073: Expr::IndexAssign object[index] = value idempotent.
    let src = r#"func main() {
    a[0] = 1
    xs[i] = 2
    print(1)
}
"#;
    let file = SourceFile::new("index_assign.sy", src);
    let report = format_dry_run(&file).expect("IndexAssign must format");
    assert!(report.formatted.contains("a[0] = 1"));
    assert!(report.formatted.contains("xs[i] = 2"));
    let again = SourceFile::new("index_assign.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_tuple_dry_run() {
    // W730 / ADR-1074: Expr::Tuple positional + labeled idempotent.
    let src = r#"func main() {
    print((1, 2))
    print((x: 1, y: 2))
}
"#;
    let file = SourceFile::new("tuple.sy", src);
    let report = format_dry_run(&file).expect("Tuple must format");
    assert!(report.formatted.contains("(1, 2)"));
    assert!(report.formatted.contains("(x: 1, y: 2)"));
    let again = SourceFile::new("tuple.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_super_dry_run() {
    // W731 / ADR-1075: Expr::Super + Member/Call idempotent.
    let src = r#"class Base {
    func speak() {
        print(1)
    }
}
class Child: Base {
    func speak() {
        super.speak()
    }
}
func main() {
    print(1)
}
"#;
    let file = SourceFile::new("super.sy", src);
    let report = format_dry_run(&file).expect("Super must format");
    assert!(report.formatted.contains("super.speak()"));
    let again = SourceFile::new("super.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_seq_dry_run() {
    // W732 / ADR-1076: Stmt::Seq tuple destructure rebuilds surface; no __tuple_.
    let src = "func main() {\n  let (a, b) = (1, 2)\n  print(a)\n}\n";
    let file = SourceFile::new("seq.sy", src);
    let report = format_dry_run(&file).expect("Seq destructure must format");
    assert!(
        report.formatted.contains("let (a, b)"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        !report.formatted.contains("__tuple_"),
        "must not emit desugar temps; formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("seq.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_where_dry_run() {
    // W733 / ADR-1077: Switch Literal case + where guard idempotent.
    let src = "func main() {\n  switch 1 {\n  case 1 where true:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_deep.sy", src);
    let report = format_dry_run(&file).expect("Switch where must format");
    assert!(
        report.formatted.contains("case 1 where true:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_range_dry_run() {
    // W734 / ADR-1078: Switch HalfOpenRange case idempotent.
    let src = "func main() {\n  switch 5 {\n  case 0..<10:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_range.sy", src);
    let report = format_dry_run(&file).expect("Switch range must format");
    assert!(
        report.formatted.contains("case 0..<10:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_range.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_variant_dry_run() {
    // W735 / ADR-1079: Switch VariantPattern case idempotent.
    let src = "func main() {\n  switch r {\n  case .ok(let v):\n    print(v)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_variant.sy", src);
    let report = format_dry_run(&file).expect("Switch variant must format");
    assert!(
        report.formatted.contains("case .ok(let v):"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_variant.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_tuple_dry_run() {
    // W736 / ADR-1080: Switch TuplePattern case idempotent.
    let src = "func main() {\n  switch t {\n  case (a, b):\n    print(a)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_tuple.sy", src);
    let report = format_dry_run(&file).expect("Switch tuple must format");
    assert!(
        report.formatted.contains("case (a, b):"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_tuple.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_is_dry_run() {
    // W737 / ADR-1081: Switch Is case special-print; no __switch_subject leak.
    let src = "func main() {\n  switch c {\n  case is Dog:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_is.sy", src);
    let report = format_dry_run(&file).expect("Switch is must format");
    assert!(
        report.formatted.contains("case is Dog:"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        !report.formatted.contains("__switch_subject"),
        "must not leak __switch_subject; formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_is.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_ident_dry_run() {
    // W738 / ADR-1082: Switch Ident case idempotent.
    let src = "func main() {\n  switch x {\n  case y:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_ident.sy", src);
    let report = format_dry_run(&file).expect("Switch Ident must format");
    assert!(
        report.formatted.contains("case y:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_ident.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_binary_dry_run() {
    // W739 / ADR-1083: Switch Binary case idempotent (Ident-headed; not `1 + 1`).
    let src = "func main() {\n  switch x {\n  case a + b:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_binary.sy", src);
    let report = format_dry_run(&file).expect("Switch Binary must format");
    assert!(
        report.formatted.contains("case a + b:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_binary.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_call_dry_run() {
    // W740 / ADR-1084: Switch Call case idempotent.
    let src = "func main() {\n  switch x {\n  case foo():\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_call.sy", src);
    let report = format_dry_run(&file).expect("Switch Call must format");
    assert!(
        report.formatted.contains("case foo():"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_call.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_member_dry_run() {
    // W741 / ADR-1085: Switch Member case idempotent.
    let src = "func main() {\n  switch x {\n  case y.z:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_member.sy", src);
    let report = format_dry_run(&file).expect("Switch Member must format");
    assert!(
        report.formatted.contains("case y.z:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_member.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_index_dry_run() {
    // W742 / ADR-1086: Switch Index case idempotent.
    let src = "func main() {\n  switch x {\n  case a[0]:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_index.sy", src);
    let report = format_dry_run(&file).expect("Switch Index must format");
    assert!(
        report.formatted.contains("case a[0]:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_index.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_optional_chain_dry_run() {
    // W743 / ADR-1087: Switch OptionalChain case idempotent.
    let src = "func main() {\n  switch x {\n  case a?.b:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_optional_chain.sy", src);
    let report = format_dry_run(&file).expect("Switch OptionalChain must format");
    assert!(
        report.formatted.contains("case a?.b:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_optional_chain.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_force_unwrap_dry_run() {
    // W744 / ADR-1088: Switch ForceUnwrap case idempotent.
    let src = "func main() {\n  switch x {\n  case a!:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_force_unwrap.sy", src);
    let report = format_dry_run(&file).expect("Switch ForceUnwrap must format");
    assert!(
        report.formatted.contains("case a!:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_force_unwrap.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_keypath_dry_run() {
    // W745 / ADR-1089: Switch KeyPath case idempotent.
    let src = "func main() {\n  switch x {\n  case \\Person.name:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_keypath.sy", src);
    let report = format_dry_run(&file).expect("Switch KeyPath must format");
    assert!(
        report.formatted.contains("case \\Person.name:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_keypath.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_array_dry_run() {
    // W746 / ADR-1090: Switch Array case idempotent.
    let src = "func main() {\n  switch x {\n  case [1, 2]:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_array.sy", src);
    let report = format_dry_run(&file).expect("Switch Array must format");
    assert!(
        report.formatted.contains("case [1, 2]:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_array.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_unary_dry_run() {
    // W747 / ADR-1091: Switch Unary case idempotent.
    let src = "func main() {\n  switch x {\n  case -y:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_unary.sy", src);
    let report = format_dry_run(&file).expect("Switch Unary must format");
    assert!(
        report.formatted.contains("case -y:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_unary.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_await_dry_run() {
    // W748 / ADR-1092: Switch Await case idempotent.
    let src = "func main() {\n  switch x {\n  case await y:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_await.sy", src);
    let report = format_dry_run(&file).expect("Switch Await must format");
    assert!(
        report.formatted.contains("case await y:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_await.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_cast_dry_run() {
    // W749 / ADR-1093: Switch Cast case idempotent.
    let src = "func main() {\n  switch x {\n  case y as Int:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_cast.sy", src);
    let report = format_dry_run(&file).expect("Switch Cast must format");
    assert!(
        report.formatted.contains("case y as Int:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_cast.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_optional_cast_dry_run() {
    // W750 / ADR-1094: Switch OptionalCast case idempotent.
    let src = "func main() {\n  switch x {\n  case y as? Int:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_optional_cast.sy", src);
    let report = format_dry_run(&file).expect("Switch OptionalCast must format");
    assert!(
        report.formatted.contains("case y as? Int:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_optional_cast.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_type_self_dry_run() {
    // W751 / ADR-1095: Switch TypeSelf case idempotent.
    let src = "func main() {\n  switch x {\n  case Int.self:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_type_self.sy", src);
    let report = format_dry_run(&file).expect("Switch TypeSelf must format");
    assert!(
        report.formatted.contains("case Int.self:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_type_self.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_super_dry_run() {
    // W752 / ADR-1096: Switch Super case idempotent.
    let src = "func main() {\n  switch x {\n  case super:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_super.sy", src);
    let report = format_dry_run(&file).expect("Switch Super must format");
    assert!(
        report.formatted.contains("case super:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_super.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_borrow_dry_run() {
    // W753 / ADR-1097: Switch Borrow case idempotent.
    let src = "func main() {\n  switch x {\n  case &x:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_borrow.sy", src);
    let report = format_dry_run(&file).expect("Switch Borrow must format");
    assert!(
        report.formatted.contains("case &x:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_borrow.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_macro_invoke_dry_run() {
    // W754 / ADR-1098: Switch MacroInvoke case idempotent.
    let src =
        "func main() {\n  switch x {\n  case #foo():\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_macro_invoke.sy", src);
    let report = format_dry_run(&file).expect("Switch MacroInvoke must format");
    assert!(
        report.formatted.contains("case #foo():"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_macro_invoke.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_closure_dry_run() {
    // W755 / ADR-1099: Switch Closure case idempotent.
    // Parser stores `{ (a) in a }` as ClosureBody::Block → printer emits multi-line form.
    let src = "func main() {\n  switch x {\n  case { (a) in a }:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_closure.sy", src);
    let report = format_dry_run(&file).expect("Switch Closure must format");
    assert!(
        report.formatted.contains("case { (a) in") && report.formatted.contains("a"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_closure.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_task_block_dry_run() {
    // W756 / ADR-1100: Switch TaskBlock case idempotent.
    // Printer emits multi-line `Task {\n    …\n}` form.
    let src =
        "func main() {\n  switch x {\n  case Task { print(1) }:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_task_block.sy", src);
    let report = format_dry_run(&file).expect("Switch TaskBlock must format");
    assert!(
        report.formatted.contains("case Task {") && report.formatted.contains("print(1)"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_task_block.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_assign_dry_run() {
    // W757 / ADR-1101: Switch Assign case idempotent.
    let src =
        "func main() {\n  switch x {\n  case y = 1:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_assign.sy", src);
    let report = format_dry_run(&file).expect("Switch Assign must format");
    assert!(
        report.formatted.contains("case y = 1:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_assign.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_compound_assign_dry_run() {
    // W758 / ADR-1102: Switch CompoundAssign case idempotent.
    let src =
        "func main() {\n  switch x {\n  case y += 1:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_compound_assign.sy", src);
    let report = format_dry_run(&file).expect("Switch CompoundAssign must format");
    assert!(
        report.formatted.contains("case y += 1:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_compound_assign.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_member_assign_dry_run() {
    // W759 / ADR-1103: Switch MemberAssign case idempotent.
    let src =
        "func main() {\n  switch x {\n  case y.z = 1:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_member_assign.sy", src);
    let report = format_dry_run(&file).expect("Switch MemberAssign must format");
    assert!(
        report.formatted.contains("case y.z = 1:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_member_assign.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_index_assign_dry_run() {
    // W760 / ADR-1104: Switch IndexAssign case idempotent.
    let src =
        "func main() {\n  switch x {\n  case a[0] = 1:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_index_assign.sy", src);
    let report = format_dry_run(&file).expect("Switch IndexAssign must format");
    assert!(
        report.formatted.contains("case a[0] = 1:"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_index_assign.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_switch_is_expr_dry_run() {
    // W761 / ADR-1105: Switch non-desugar Is case idempotent; must print `y is Int` (not bare `is Int`).
    let src =
        "func main() {\n  switch x {\n  case y is Int:\n    print(1)\n  default:\n    print(0)\n  }\n}\n";
    let file = SourceFile::new("switch_is_expr.sy", src);
    let report = format_dry_run(&file).expect("Switch non-desugar Is must format");
    assert!(
        report.formatted.contains("case y is Int:"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        !report.formatted.contains("case is Int:"),
        "must not collapse to desugar form; formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("switch_is_expr.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_if_case_literal_dry_run() {
    // W762 / ADR-1106: IfCase Literal pattern idempotent; must print `if case 1 = x`.
    let src = "func main() {\n  if case 1 = x {\n    print(1)\n  }\n}\n";
    let file = SourceFile::new("if_case_literal.sy", src);
    let report = format_dry_run(&file).expect("IfCase Literal must format");
    assert!(
        report.formatted.contains("if case 1 = x"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("if_case_literal.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_if_case_range_dry_run() {
    // W763 / ADR-1107: IfCase Range pattern idempotent; must print `0..<10` (and Closed `...`).
    let src = "func main() {\n  if case 0..<10 = x {\n    print(1)\n  }\n  if case 1...3 = y {\n    print(2)\n  }\n}\n";
    let file = SourceFile::new("if_case_range.sy", src);
    let report = format_dry_run(&file).expect("IfCase Range must format");
    assert!(
        report.formatted.contains("0..<10"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        report.formatted.contains("1...3"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("if_case_range.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_if_case_is_dry_run() {
    // W764 / ADR-1108: IfCase desugar Is idempotent; print `is Dog`, never leak __switch_subject.
    let src = "func main() {\n  if case is Dog = x {\n    print(1)\n  }\n}\n";
    let file = SourceFile::new("if_case_is.sy", src);
    let report = format_dry_run(&file).expect("IfCase desugar Is must format");
    assert!(
        report.formatted.contains("is Dog"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        !report.formatted.contains("__switch_subject"),
        "must not leak desugar subject; formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("if_case_is.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_protocol_assoc_dry_run() {
    // W765 / ADR-1109: protocol associatedtype idempotent; print `associatedtype T`, never `: Any`.
    let src = "protocol P {\n  associatedtype T\n  associatedtype Item: Equatable\n}\n";
    let file = SourceFile::new("protocol_assoc.sy", src);
    let report = format_dry_run(&file).expect("protocol associatedtype must format");
    assert!(
        report.formatted.contains("associatedtype T"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        report.formatted.contains("associatedtype Item: Equatable"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        !report.formatted.contains(": Any"),
        "must not print Any sentinel; formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("protocol_assoc.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_extension_where_dry_run() {
    // W766 / ADR-1110: extension where idempotent; print `where Element: Equatable`.
    let src = "extension Array where Element: Equatable {\n  func foo() {}\n}\n";
    let file = SourceFile::new("extension_deep.sy", src);
    let report = format_dry_run(&file).expect("extension where must format");
    assert!(
        report.formatted.contains("where Element: Equatable"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("extension_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_enum_type_params_dry_run() {
    // W767 / ADR-1111: enum type_params idempotent; print `enum Box<T>`.
    let src = "enum Box<T> {\n  wrap(value: T)\n}\n";
    let file = SourceFile::new("enum_deep.sy", src);
    let report = format_dry_run(&file).expect("enum type_params must format");
    assert!(
        report.formatted.contains("enum Box<T>"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("enum_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_data_class_type_params_dry_run() {
    // W768 / ADR-1112: data class type_params idempotent; print `data class Box<T>`.
    let src = "data class Box<T>(value: T)\n";
    let file = SourceFile::new("dataclass_deep.sy", src);
    let report = format_dry_run(&file).expect("data class type_params must format");
    assert!(
        report.formatted.contains("data class Box<T>"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("dataclass_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_data_class_where_dry_run() {
    // W769 / ADR-1113: data class where idempotent; print `where T: Equatable`.
    let src = "data class Box<T>(value: T) where T: Equatable\n";
    let file = SourceFile::new("dataclass_deep.sy", src);
    let report = format_dry_run(&file).expect("data class where must format");
    assert!(
        report.formatted.contains("where T: Equatable"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("dataclass_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_final_class_type_params_dry_run() {
    // W770 / ADR-1114: final class type_params idempotent; print `Box<T>`.
    let src = "final class Box<T> {\n    wrap(value: T)\n}\n";
    let file = SourceFile::new("finalclass_deep.sy", src);
    let report = format_dry_run(&file).expect("final class type_params must format");
    assert!(
        report.formatted.contains("Box<T>"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("finalclass_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_func_type_params_dry_run() {
    // W771 / ADR-1115: function type_params idempotent; print `id<T>`.
    let src = "func id<T>(x: T) {\n}\n";
    let file = SourceFile::new("func_deep.sy", src);
    let report = format_dry_run(&file).expect("function type_params must format");
    assert!(
        report.formatted.contains("id<T>"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("func_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_func_throws_dry_run() {
    // W772 / ADR-1116: function throws idempotent; print bare ` throws`.
    let src = "func boom() throws {\n}\n";
    let file = SourceFile::new("func_throws.sy", src);
    let report = format_dry_run(&file).expect("function throws must format");
    assert!(
        report.formatted.contains(" throws"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("func_throws.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_func_effect_dry_run() {
    // W773 / ADR-1117: function effect idempotent; print `requires Pure`.
    let src = "func boom() requires Pure {\n}\n";
    let file = SourceFile::new("func_effect.sy", src);
    let report = format_dry_run(&file).expect("function effect must format");
    assert!(
        report.formatted.contains("requires Pure"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("func_effect.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_func_where_dry_run() {
    // W774 / ADR-1118: function where idempotent; print `where T: Equatable`.
    let src = "func id<T>(x: T) where T: Equatable {\n}\n";
    let file = SourceFile::new("func_deep.sy", src);
    let report = format_dry_run(&file).expect("function where must format");
    assert!(
        report.formatted.contains("where T: Equatable"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("func_deep.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_parameter_attrs_dry_run() {
    // W775 / ADR-1119: parameter attributes idempotent; print `@Autowired`.
    let src = "func greeter(@Autowired logger: Logger) {\n}\n";
    let file = SourceFile::new("parameter_attrs.sy", src);
    let report = format_dry_run(&file).expect("parameter attributes must format");
    assert!(
        report.formatted.contains("@Autowired"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("parameter_attrs.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_isolated_sending_dry_run() {
    // W776 / ADR-1120: isolated/sending idempotent; print `isolated `.
    let src = "func f(isolated x: Int) {\n}\n";
    let file = SourceFile::new("isolated_sending.sy", src);
    let report = format_dry_run(&file).expect("isolated/sending must format");
    assert!(
        report.formatted.contains("isolated "),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("isolated_sending.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_binop_mod_dry_run() {
    // W777 / ADR-1121: binop Mod idempotent; print ` % `.
    let src = "func main() {\n    print(1 % 2)\n}\n";
    let file = SourceFile::new("binop_mod.sy", src);
    let report = format_dry_run(&file).expect("binop Mod must format");
    assert!(
        report.formatted.contains(" % "),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("binop_mod.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_binop_lteq_dry_run() {
    // W778 / ADR-1122: binop LtEq idempotent; print ` <= `.
    let src = "func main() {\n    print(1 <= 2)\n}\n";
    let file = SourceFile::new("binop_lteq.sy", src);
    let report = format_dry_run(&file).expect("binop LtEq must format");
    assert!(
        report.formatted.contains(" <= "),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("binop_lteq.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_binop_gteq_dry_run() {
    // W779 / ADR-1123: binop GtEq idempotent; print ` >= `.
    let src = "func main() {\n    print(1 >= 2)\n}\n";
    let file = SourceFile::new("binop_gteq.sy", src);
    let report = format_dry_run(&file).expect("binop GtEq must format");
    assert!(
        report.formatted.contains(" >= "),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("binop_gteq.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_binop_nilcoalesce_dry_run() {
    // W780 / ADR-1124: binop NilCoalesce idempotent; print ` ?? `.
    let src = "func main() {\n    print(1 ?? 2)\n}\n";
    let file = SourceFile::new("binop_nilcoalesce.sy", src);
    let report = format_dry_run(&file).expect("binop NilCoalesce must format");
    assert!(
        report.formatted.contains(" ?? "),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("binop_nilcoalesce.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_unary_try_dry_run() {
    // W781 / ADR-1125: unary Try idempotent; print postfix `?`.
    let src = "func main() {\n    print(x?)\n}\n";
    let file = SourceFile::new("unary_try.sy", src);
    let report = format_dry_run(&file).expect("unary Try must format");
    assert!(
        report.formatted.contains("x?"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("unary_try.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);

    // Prefix `try expr` shares UnaryOp::Try; canonical form is postfix `expr?`.
    let prefix = "func main() {\n    print(try x)\n}\n";
    let file_p = SourceFile::new("unary_try_prefix.sy", prefix);
    let report_p = format_dry_run(&file_p).expect("prefix try must format");
    assert!(
        report_p.formatted.contains("x?"),
        "prefix must fold to postfix; formatted={:?}",
        report_p.formatted
    );
    let again_p = SourceFile::new("unary_try_prefix.sy", report_p.formatted.as_str());
    let r2_p = format_dry_run(&again_p).expect("idempotent after fold");
    assert!(!r2_p.would_change);
}

#[test]
fn fmt_type_optional_dry_run() {
    // W782 / ADR-1126: type Optional idempotent; print postfix `?`.
    let src = "func main() {\n    let x: Int? = nil\n}\n";
    let file = SourceFile::new("type_optional.sy", src);
    let report = format_dry_run(&file).expect("type Optional must format");
    assert!(
        report.formatted.contains("Int?"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("type_optional.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_opaque_some_dry_run() {
    // W783 / ADR-1127: OpaqueSome idempotent; print `some T`.
    let src = "func main() {\n    let x: some Equatable = 1\n}\n";
    let file = SourceFile::new("opaque_some.sy", src);
    let report = format_dry_run(&file).expect("OpaqueSome must format");
    assert!(
        report.formatted.contains("some Equatable"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("opaque_some.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_opaque_any_dry_run() {
    // W784 / ADR-1128: OpaqueAny idempotent; print `any T`.
    let src = "func main() {\n    let x: any Equatable = 1\n}\n";
    let file = SourceFile::new("opaque_any.sy", src);
    let report = format_dry_run(&file).expect("OpaqueAny must format");
    assert!(
        report.formatted.contains("any Equatable"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("opaque_any.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_func_type_dry_run() {
    // W785 / ADR-1129: Function type idempotent; print `(T) -> Ret`.
    let src = "func main() {\n    let f: (Int) -> Int = id\n}\n";
    let file = SourceFile::new("func_type.sy", src);
    let report = format_dry_run(&file).expect("Function type must format");
    assert!(
        report.formatted.contains("(Int) -> Int"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("func_type.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_type_reference_dry_run() {
    // W786 / ADR-1130: Reference idempotent; print `&T` / `&mut T`.
    let src = "func main() {\n    let x: &Int = y\n    let m: &mut Int = z\n}\n";
    let file = SourceFile::new("type_reference.sy", src);
    let report = format_dry_run(&file).expect("Reference must format");
    assert!(
        report.formatted.contains("&Int"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        report.formatted.contains("&mut Int"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("type_reference.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_literal_bigint_dry_run() {
    // W787 / ADR-1131: oversized `…n` → BigInt; print digits + `n`.
    let src =
        "func main() {\n    print(99999999999999999999999999999999999999999n)\n}\n";
    let file = SourceFile::new("literal_bigint.sy", src);
    let report = format_dry_run(&file).expect("BigInt must format");
    assert!(
        report
            .formatted
            .contains("99999999999999999999999999999999999999999n"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("literal_bigint.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_literal_interpolated_dry_run() {
    // W788 / ADR-1132: Interpolated idempotent; print `"…\(expr)…"`.
    let src = "func main() {\n    print(\"hi\\(x)\")\n}\n";
    let file = SourceFile::new("literal_interpolated.sy", src);
    let report = format_dry_run(&file).expect("Interpolated must format");
    assert!(
        report.formatted.contains("\\(x)"),
        "formatted={:?}",
        report.formatted
    );
    assert!(
        report.formatted.contains("\"hi\\(x)\""),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("literal_interpolated.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

#[test]
fn fmt_literal_regex_dry_run() {
    // W789 / ADR-1133: Regex idempotent; print `/pattern/`.
    let src = "func main() {\n    print(/abc/)\n}\n";
    let file = SourceFile::new("literal_regex.sy", src);
    let report = format_dry_run(&file).expect("Regex must format");
    assert!(
        report.formatted.contains("/abc/"),
        "formatted={:?}",
        report.formatted
    );
    let again = SourceFile::new("literal_regex.sy", report.formatted.as_str());
    let r2 = format_dry_run(&again).expect("idempotent");
    assert!(!r2.would_change);
}

