//! Computed get-only, static var/let, lazy var MVP.

use syx_driver::lang_props_v2::{COMPUTED_GET_ONLY, LAZY_VAR, STATIC_VAR};
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn props_fingerprint_strings_in_driver() {
    let lib = include_str!("../../src/lang_props_v2.rs");
    assert!(lib.contains(COMPUTED_GET_ONLY));
    assert!(lib.contains(STATIC_VAR));
    assert!(lib.contains(LAZY_VAR));
}

#[test]
fn computed_get_only_prints_doubled() {
    let src = r#"
class Counter {
    var x: Int
    var doubled: Int {
        get { return self.x * 2 }
    }
    init(x: Int) { self.x = x }
}
func main() {
    let c = Counter(x: 5)
    print(c.doubled)
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "10"));
}

#[test]
fn static_var_shared_across_instances() {
    let src = r#"
class Counter {
    static var n: Int = 0
    init() {}
}
func main() {
    Counter.n = 7
    print(Counter.n)
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "7"));
}

#[test]
fn static_let_reads_type_level_value() {
    let src = r#"
class Config {
    static let label: String = "ok"
}
func main() {
    print(Config.label)
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "ok"));
}

#[test]
fn lazy_var_runs_initializer_once() {
    let src = r#"
class Box {
    lazy var label: String = "lazy-ok"
    init() {}
}
func main() {
    let b = Box()
    print(b.label)
    print(b.label)
}
"#;
    let printed = run(src);
    assert_eq!(printed.len(), 2);
    assert!(printed.iter().all(|l| l.trim() == "lazy-ok"));
}
