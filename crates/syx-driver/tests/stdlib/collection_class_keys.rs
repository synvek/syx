//! Hashable class instances as Set elements / Dictionary keys (identity).
//! Slice 3: Object signatures — class-key containsKey / remove.
//! Slice 12: user hash(into:) does not drive Dict/Set; explicit Call still works.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_parser::Parser;
use syx_diagnostics::SourceFile;
use syx_semantic::SemanticAnalyzer;

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn hashable_class_dict_keys_do_not_collide() {
    let out = run(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
}
func main() {
    let a = Key(id: 1)
    let b = Key(id: 2)
    var d: Dictionary<Key, Int> = {a: 10, b: 20}
    print(d.count)
    print(d[a])
    print(d[b])
    d[a] = 99
    print(d[a])
    print(d.count)
}
"#,
    );
    assert_eq!(out, "2\n10\n20\n99\n2");
}

#[test]
fn hashable_class_set_identity_dedup() {
    let out = run(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
}
func main() {
    let a = Key(id: 1)
    let b = Key(id: 1)
    var s: Set<Key> = []
    print(s.insert(a))
    print(s.insert(a))
    print(s.insert(b))
    print(s.count)
    print(s.contains(a))
    print(s.contains(b))
}
"#,
    );
    assert_eq!(out, "true\nfalse\ntrue\n2\ntrue\ntrue");
}

#[test]
fn hashable_class_dict_contains_key_and_remove() {
    let out = run(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
}
func main() {
    let a = Key(id: 1)
    let b = Key(id: 2)
    var d: Dictionary<Key, Int> = {a: 10, b: 20}
    print(d.containsKey(a))
    print(d.containsKey(b))
    print(d.remove(a))
    print(d.containsKey(a))
    print(d.count)
    print(d[b])
}
"#,
    );
    assert_eq!(out, "true\ntrue\ntrue\nfalse\n1\n20");
}

#[test]
fn non_hashable_class_rejected_as_set_element() {
    let src = r#"
class Plain {
    var id: Int
    init(id: Int) { self.id = id }
}
func main() {
    var s: Set<Plain> = []
    print(s.count)
}
"#;
    let module = Parser::parse(SourceFile::new("t.sy", src)).unwrap();
    let sema = SemanticAnalyzer::analyze_collecting(&module);
    assert!(
        sema.errors.iter().any(|e| e.message.contains("Hashable")
            || e.code.as_deref() == Some("S015")),
        "expected S015 Hashable error, got {:?}",
        sema.errors
    );
}

#[test]
fn non_hashable_class_rejected_as_dict_key() {
    let src = r#"
class Plain {
    var id: Int
    init(id: Int) { self.id = id }
}
func main() {
    let p = Plain(id: 1)
    var d: Dictionary<Plain, Int> = {p: 1}
    print(d.count)
}
"#;
    let module = Parser::parse(SourceFile::new("t.sy", src)).unwrap();
    let sema = SemanticAnalyzer::analyze_collecting(&module);
    assert!(
        sema.errors.iter().any(|e| e.message.contains("Hashable")
            || e.code.as_deref() == Some("S042")),
        "expected S042 Hashable error, got {:?}",
        sema.errors
    );
}

#[test]
fn custom_hash_into_does_not_collapse_dict_keys() {
    // Equal fields + constant hash(into:) still two Dict keys (identity).
    let out = run(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
    func hash(into hasher: Hasher) {
        hasher.combine(42)
    }
}
func main() {
    let a = Key(id: 1)
    let b = Key(id: 1)
    var d: Dictionary<Key, Int> = {a: 10, b: 20}
    print(d.count)
    print(d[a])
    print(d[b])
}
"#,
    );
    assert_eq!(out, "2\n10\n20");
}

#[test]
fn custom_hash_into_does_not_collapse_set_elements() {
    let out = run(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
    func hash(into hasher: Hasher) {
        hasher.combine(42)
    }
}
func main() {
    let a = Key(id: 1)
    let b = Key(id: 1)
    var s: Set<Key> = []
    print(s.insert(a))
    print(s.insert(b))
    print(s.count)
}
"#,
    );
    assert_eq!(out, "true\ntrue\n2");
}

#[test]
fn explicit_user_hash_into_updates_hasher() {
    let with_custom = run(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
    func hash(into hasher: Hasher) {
        __syx_hasher_combine(hasher, 999)
    }
}
func main() {
    let a = Key(id: 1)
    var h = __syx_hasher_new()
    a.hash(into: h)
    print(__syx_hasher_finalize(h))
}
"#,
    );
    let baseline = run(
        r#"
func main() {
    var h = __syx_hasher_new()
    __syx_hasher_combine(h, 999)
    print(__syx_hasher_finalize(h))
}
"#,
    );
    assert_eq!(
        with_custom.trim(),
        baseline.trim(),
        "user hash(into:) Call should match direct combine; custom={with_custom:?} baseline={baseline:?}"
    );
}

#[test]
fn missing_hash_into_on_class_is_c020() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
class Key: Hashable {
    var id: Int
    init(id: Int) { self.id = id }
}
func main() {
    let a = Key(id: 1)
    var h = __syx_hasher_new()
    a.hash(into: h)
    print(__syx_hasher_finalize(h))
}
"#,
        &DriverOptions::default(),
    )
    .expect_err("missing hash(into:)");
    assert_eq!(err.code.as_deref(), Some("C020"));
    assert!(
        err.message.contains("hash(into:)") || err.message.contains("Key"),
        "message={}",
        err.message
    );
}
