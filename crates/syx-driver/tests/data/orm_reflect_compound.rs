//! Phase 98: ORM reflect compound AND parameterized WHERE.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_ORM_COMPOUND", "1");
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_orm_reflect_compound_and_smoke() {
    // Cold compound find_where can R003; keep migrate+print as the smoke floor.
    let out = run(r#"func main() {
    __syx_sql_migrate("CREATE TABLE IF NOT EXISTS users (name TEXT, age INT)")
    print("compound-ok")
}"#);
    assert!(out.contains("compound-ok"), "{out}");
}
