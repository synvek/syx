//! Todo 5 slice 19: Set storage is a linear Vec deduped by value_equals.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn set_literal_dedup_and_insert_keeps_count() {
    let out = run(
        r#"
func main() {
    var s: Set<Int> = [1, 1, 2]
    print(s.count)
    print(s.insert(2))
    print(s.count)
    print(s.insert(3))
    print(s.count)
}
"#,
    );
    assert_eq!(out, "2\nfalse\n2\ntrue\n3");
}
