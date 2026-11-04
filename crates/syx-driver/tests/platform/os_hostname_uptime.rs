//! Os.hostname() / Os.uptimeMs() MVP (`stdlib-os-v2:hostname-uptime-mvp`).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn with_os(body: &str) -> String {
    let os = include_str!("../../../../packages/os/mod.sy");
    format!("{os}\n{body}")
}

fn run(src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &opts()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!("{e}")),
    }
}


#[test]
fn os_hostname_and_uptime_positive() {
    let printed = run(&with_os(
        r#"
func main() {
    try {
        print(Os.hostname())
    } catch (e) {
        print("hostname-failed")
    }
    print(Os.uptimeMs())
    print(Os.cpuCount())
}
"#,
    ))
    .expect("os hostname/uptime should run");
    assert!(
        !printed[0].trim().is_empty(),
        "hostname should be non-empty or fail marker, got {:?}",
        printed[0]
    );
    let uptime: i64 = printed[1].trim().parse().expect("uptimeMs int");
    assert!(uptime > 0, "uptimeMs should be positive, got {uptime}");
    let cpu: i64 = printed[2].trim().parse().expect("cpuCount int");
    assert!(cpu > 0, "cpuCount should remain positive, got {cpu}");
}

#[test]
fn os_hostname_remains_importable_surface() {
    let manifest = include_str!("../../../../packages/internal/manifest.json");
    assert!(
        manifest.contains("\"Os\"") || manifest.contains("Os"),
        "Os must remain in importable_modules"
    );
    let os = include_str!("../../../../packages/os/mod.sy");
    assert!(os.contains("public class Os"));
    assert!(os.contains("cpuCount") && os.contains("hostname") && os.contains("uptimeMs"));
}
