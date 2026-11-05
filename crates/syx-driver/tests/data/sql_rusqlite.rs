//! Phase 91: rusqlite embedded SQL E2E.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_sql_rusqlite_create_insert_select() {
    let out = run(
        r#"func main() {
    __syx_sql_exec("CREATE TABLE users (id INT, name TEXT)")
    __syx_sql_exec("INSERT INTO users VALUES (1, 'alice')")
    let rows = __syx_sql_query("SELECT * FROM users")
    print(rows.len())
}"#,
    );
    assert!(out.contains("1"), "{out}");
}

