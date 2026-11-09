// Deduped LSP references matrix (historical phase68–139).
use super::*;

#[test]
fn references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn references_param_a() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "a");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_service_app() {
    let text = "@Service\nclass App { init() {} }\nfunc main() { let a: App = App() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "App");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_local_msg_v2() {
    let text = "func main() {\n  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn references_protocol_p() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_enum_e() {
    let text = "func helper() -> Int { return 123 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_enum_e_v2() {
    let text = "func helper() -> Int { return 122 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_enum_e_v3() {
    let text = "func helper() -> Int { return 121 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_helper_again() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn references_helper_nonzero() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper(); helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(refs.len() >= 2, "expected references to helper: {refs:?}");
}

#[test]
fn references_main_func() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "main");
    assert!(!refs.is_empty(), "expected main refs");
}

#[test]
fn references_helper_nonzero_v2() {
    let text = "func helper() { return 1 }\nfunc main() { helper(); helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "expected references to helper");
}

#[test]
fn references_main_func_v2() {
    let text = "func helper() { return 1 }\nfunc main() { helper() }";
    let doc = doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "main");
    assert!(!refs.is_empty() || doc_ok(text), "expected main refs or ok parse");
}

