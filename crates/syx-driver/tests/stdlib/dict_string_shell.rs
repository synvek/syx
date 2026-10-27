//! Todo 5 slice 13: Dictionary String-shell honesty.
//! Int keys use `\x1fsyx:` slots internally; keys/count expose logical keys only.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn int_key_dict_insert_lookup_remove() {
    let out = run(
        r#"
func main() {
    var d: Dictionary<Int, Int> = {10: 100, 20: 200}
    print(d[10])
    print(d[20])
    print(d.containsKey(10))
    print(d.remove(10))
    print(d.containsKey(10))
    print(d.count)
    print(d[20])
}
"#,
    );
    assert_eq!(out, "100\n200\ntrue\ntrue\nfalse\n1\n200");
}

#[test]
fn int_key_keys_and_count_no_encoded_slots() {
    let out = run(
        r#"
func main() {
    var d: Dictionary<Int, Int> = {10: 100, 20: 200, 30: 300}
    print(d.count)
    print(d.keys())
    for k in d.keys() {
        print(k)
    }
}
"#,
    );
    assert!(
        !out.contains("syx:") && !out.contains('\u{1f}'),
        "keys()/print must not leak String-shell encoding slots: {out:?}"
    );
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "3", "count is logical entries, not encoded slot pairs");
    // keys() prints as an Array of Ints (order not specified)
    let keys_line = lines[1];
    assert!(
        keys_line.contains("10") && keys_line.contains("20") && keys_line.contains("30"),
        "keys() should show logical Int keys: {keys_line:?}"
    );
    let mut ints: Vec<i64> = lines[2..]
        .iter()
        .map(|s| s.parse().expect("key line is Int"))
        .collect();
    ints.sort();
    assert_eq!(ints, vec![10, 20, 30]);
}
