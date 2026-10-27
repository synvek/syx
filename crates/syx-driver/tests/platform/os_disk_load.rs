//! Os.diskTotal / Os.diskAvailable / Os.loadAverage MVP.

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
fn os_disk_load_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("stdlib-os-v3:disk-load-mvp"));
    assert!(lib.contains("heap_v109_phase137_track_d_os_disk_load_fingerprint"));
    let os = include_str!("../../../../packages/os/mod.sy");
    assert!(os.contains("diskTotal") && os.contains("diskAvailable") && os.contains("loadAverage"));
    assert!(
        os.contains("__syx_os_disk_total")
            && os.contains("__syx_os_disk_available")
            && os.contains("__syx_os_load_average")
    );
    let hosts = include_str!("../../../syx-vm/src/os_hosts.rs");
    assert!(
        hosts.contains("host_os_disk_total")
            && hosts.contains("host_os_disk_available")
            && hosts.contains("host_os_load_average")
    );
    let registry = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        registry.contains("OsDiskTotal")
            && registry.contains("OsDiskAvailable")
            && registry.contains("OsLoadAverage")
    );
}

#[test]
fn os_disk_and_load_positive() {
    let printed = run(&with_os(
        r#"
func main() {
    try {
        print(Os.diskTotal(path: "/"))
    } catch (e) {
        print("-1")
    }
    try {
        print(Os.diskAvailable(path: "/"))
    } catch (e) {
        print("-1")
    }
    print(Os.loadAverage())
}
"#,
    ))
    .expect("os disk/load should run");
    let total: i64 = printed[0].trim().parse().expect("diskTotal int");
    assert!(total > 0, "diskTotal should be positive, got {total}");
    let avail: i64 = printed[1].trim().parse().expect("diskAvailable int");
    assert!(avail >= 0, "diskAvailable should be non-negative, got {avail}");
    assert!(avail <= total, "diskAvailable should not exceed diskTotal");
    let load: i64 = printed[2].trim().parse().expect("loadAverage int");
    assert!(load >= 0, "loadAverage should be non-negative, got {load}");
}

#[test]
fn os_disk_load_remains_importable_surface() {
    let manifest = include_str!("../../../../packages/internal/manifest.json");
    assert!(
        manifest.contains("\"Os\"") || manifest.contains("Os"),
        "Os must remain in importable_modules"
    );
    let os = include_str!("../../../../packages/os/mod.sy");
    assert!(os.contains("public class Os"));
    assert!(
        os.contains("diskTotal")
            && os.contains("diskAvailable")
            && os.contains("loadAverage")
            && os.contains("hostname")
    );
}
