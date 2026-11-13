//! R-41 OrderedSet public façade smoke: new → insert → contains (Sendable items).
//! ≠ OrderedSet<T> GA; host bag runtime unchanged.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn ordered_set_new_insert_contains() {
    syx_host_plugin::register_echo_plugin_in_process();
    // Same surface as Deque smoke: type is importable/prelude-resolvable without
    // `import OrderedSet` (explicit import hits S005 on static members).
    let result = compile_and_run_embedded(
        r#"
func main() {
    var s = OrderedSet.new()
    s = OrderedSet.insert(set: s, item: 1)
    s = OrderedSet.insert(set: s, item: 2)
    s = OrderedSet.insert(set: s, item: 1)
    print(OrderedSet.contains(set: s, item: 1))
    print(OrderedSet.contains(set: s, item: 2))
    print(OrderedSet.contains(set: s, item: 3))
}
"#,
        &opts(),
    )
    .expect("run OrderedSet smoke");
    let lines: Vec<_> = result.printed.iter().map(|l| l.trim()).collect();
    assert_eq!(lines, ["true", "true", "false"], "printed={lines:?}");
}
