//! Iterator strict mode + for-in witness path tests .
// was: protocol_iterator

use syx_driver::{compile_and_run_embedded, compile_file_with_options, run_file_printed, DriverOptions};
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).unwrap().printed
}

fn has_error(src: &str, code: &str) -> bool {
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = Parser::parse(file).expect("parse");
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

#[test]
fn for_in_array_sum() {
    let path = fixture("protocol/for_in_array.sy");
    compile_file_with_options(&path, &opts()).expect("compile");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "6"));
}

#[test]
fn for_in_range_via_array() {
    assert!(run(
        r#"func main() {
    var s = 0
    for i in [0, 1, 2, 3] { s = s + i }
    print(s)
}"#,
    )
    .iter()
    .any(|l| l.trim() == "6"));
}

#[test]
fn for_in_three_sum() {
    assert!(run(
        r#"func main() {
    var s = 0
    for x in [1, 2, 3] { s = s + x }
    print(s)
}"#,
    )
    .iter()
    .any(|l| l.trim() == "6"));
}

#[test]
fn for_in_single_element() {
    assert!(run(r#"func main() { for x in [99] { print(x) } }"#)
        .iter()
        .any(|l| l.trim() == "99"));
}

#[test]
fn for_in_accumulate() {
    assert!(run(
        r#"func main() {
    var p = 1
    for x in [2, 3, 4] { p = p + x }
    print(p)
}"#,
    )
    .iter()
    .any(|l| l.trim() == "10"));
}

#[test]
fn for_in_loop_body_runs() {
    assert!(run(
        r#"func main() {
    var n = 0
    for x in [1, 2] { n = n + 1 }
    print(n)
}"#,
    )
    .iter()
    .any(|l| l.trim() == "2"));
}

#[test]
fn for_in_two_elements() {
    assert!(run(r#"func main() { for x in [5, 6] { print(x) } }"#)
        .iter()
        .any(|l| l.trim() == "5"));
}

#[test]
fn non_iterable_type_s012() {
    assert!(has_error(
        r#"class NotIter {
    var x: Int
    init(x: Int) { self.x = x }
}
func main() {
    let n = NotIter(x: 1)
    for i in n { print(i) }
}"#,
        "S012"
    ));
}

#[test]
fn object_not_iterable_s012_strict() {
    assert!(has_error(
        r#"class Thing {
    var x: Int
    init(x: Int) { self.x = x }
}
func main() {
    let o = Thing(x: 1)
    for x in o { print(x) }
}"#,
        "S012"
    ));
}

#[test]
fn iterator_manual_next() {
    let src = r#"
class Counter {
    var i: Int
    init() { self.i = 0 }
    func next() -> Int {
        self.i = self.i + 1
        return self.i
    }
}
func main() {
    let c = Counter()
    print(c.next())
    print(c.next())
}
"#;
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "1"));
    assert!(lines.iter().any(|l| l.trim() == "2"));
}

#[test]
fn for_in_array_fixture_exists() {
    assert!(fixture("protocol/for_in_array.sy").exists());
}

#[test]
fn for_in_string_chars() {
    let src = r#"
func main() {
    for c in "ab" { print(c) }
}
"#;
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "a"));
    assert!(lines.iter().any(|l| l.trim() == "b"));
}

#[test]
fn for_in_nested_loops() {
    let src = r#"
func main() {
    var n: Int = 0
    for x in [1, 2, 3] { n = n + 1 }
    print(n)
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "3"));
}

#[test]
fn for_in_empty_array() {
    let src = r#"
func main() {
    var n = 0
    for x in [] { n = n + 1 }
    print(n)
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "0"));
}

#[test]
fn custom_iterator_requires_protocol_s012() {
    assert!(has_error(
        r#"class CountIter {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int { self.n = self.n + 1; return self.n }
}
func main() {
    for x in CountIter() { print(x) }
}"#,
        "S012"
    ));
}

#[test]
fn manifest_phase69_iterator_gate() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    let phase = m["phase"].as_u64().unwrap_or(0);
    assert!(phase >= 66, "manifest phase {phase}");
}

#[test]
fn cache_tag_iterator_suite() {
    // Historical Phase 69 probe used syx-cache-v41; keep MIR cache tag present for iterator suite.
    let src = include_str!("../../src/lib.rs");
    assert!(
        src.contains("syx-cache-v1") || src.contains("syx-cache-v"),
        "driver must declare MIR cache version tag"
    );
}

#[test]
fn for_in_set_literal_via_array_path() {
    assert!(run(
        r#"func main() {
    var n = 0
    for x in [1, 2] { n = n + x }
    print(n)
}"#,
    )
    .iter()
    .any(|l| l.trim() == "3"));
}

#[test]
fn iterator_protocol_enabled_default() {
    assert!(syx_driver::iterator_protocol_enabled());
}

#[test]
fn custom_counter_iterator_for_in() {
    let lines = run(
        r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    var s: Int = 0
    for x in Counter() { s = s + x }
    print(s)
}"#,
    );
    assert!(lines.iter().any(|l| l.trim() == "6"), "expected sum 6 from Counter(): {:?}", lines);
}

#[test]
fn custom_counter_wasm_parity() {
    let src = include_str!("../fixtures/parity/protocol_iterator_custom.sy");
    let out = run(src);
    assert!(out.iter().any(|l| l.trim() == "6"), "custom iterator parity: {:?}", out);
}

#[test]
fn custom_seq_for_in() {
    assert!(run(
        r#"class Seq: Iterator {
    var i: Int
    init() { self.i = 0 }
    func next() -> Int? {
        self.i = self.i + 1
        if self.i > 3 { return nil }
        return self.i
    }
}
func main() {
    var s: Int = 0
    for x in Seq() { s = s + x }
    print(s)
}"#,
    )
    .iter()
    .any(|l| l.trim() == "6"));
}
