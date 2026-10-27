//! Todo 5 slice 4: Array.contains(item: Object) — String + Int paths.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn array_contains_string_elements() {
    let out = run(
        r#"
func main() {
    var a: Array<String> = ["x", "y"]
    print(a.contains("x"))
    print(a.contains("z"))
}
"#,
    );
    assert_eq!(out, "true\nfalse");
}

#[test]
fn array_contains_int_still_works() {
    let out = run(
        r#"
func main() {
    var a: Array<Int> = [1, 2]
    print(a.contains(2))
    print(a.contains(3))
}
"#,
    );
    assert_eq!(out, "true\nfalse");
}
