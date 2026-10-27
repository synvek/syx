//! Todo 5 slice 20: Set.union / intersection use value_equals like insert/contains.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn set_union_dedups_by_value_equals() {
    let out = run(
        r#"
func main() {
    let a: Set<Int> = [1, 2]
    let b: Set<Int> = [2, 3]
    let u = a.union(b)
    print(u.count)
    print(u.contains(1))
    print(u.contains(2))
    print(u.contains(3))
}
"#,
    );
    assert_eq!(out, "3\ntrue\ntrue\ntrue");
}

#[test]
fn set_intersection_uses_value_equals() {
    let out = run(
        r#"
func main() {
    let a: Set<Int> = [1, 2]
    let b: Set<Int> = [2, 3]
    let i = a.intersection(b)
    print(i.count)
    print(i.contains(1))
    print(i.contains(2))
    print(i.contains(3))
}
"#,
    );
    assert_eq!(out, "1\nfalse\ntrue\nfalse");
}
