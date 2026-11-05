//! Phase 91: ORM findWhere/update/delete roundtrip.

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn p98_orm_find_where_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    __syx_orm_save("users", { "name": "alice", "role": "admin" })
    let rows = __syx_orm_find_where("users", "name", "alice")
    print(rows.count)
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains('1')), "{out:?}");
}

#[test]
fn p98_orm_update_where_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    __syx_orm_save("items", { "k": "a", "v": "1" })
    let n = __syx_orm_update_where("items", "k", "a", "v", "2")
    print(n)
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains('1')), "{out:?}");
}

#[test]
fn p98_orm_delete_where_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    __syx_orm_save("del", { "id": "x" })
    let n = __syx_orm_delete_where("del", "id", "x")
    print(n)
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains('1')), "{out:?}");
}

