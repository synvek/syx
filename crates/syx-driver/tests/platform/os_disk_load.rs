//! ProcessInfo.diskTotal / ProcessInfo.diskAvailable / ProcessInfo.loadAverage MVP.

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
fn os_disk_and_load_positive() {
    let printed = run(&with_os(
        r#"
func main() {
    do {
        print(ProcessInfo.diskTotal(path: "/"))
    } catch (e) {
        print("-1")
    }
    do {
        print(ProcessInfo.diskAvailable(path: "/"))
    } catch (e) {
        print("-1")
    }
    print(ProcessInfo.loadAverage())
}
"#,
    ))
    .expect("os disk/load should run");
    let total: i64 = printed[0].trim().parse().expect("diskTotal int");
    assert!(total > 0, "diskTotal should be positive, got {total}");
    let avail: i64 = printed[1].trim().parse().expect("diskAvailable int");
    assert!(
        avail >= 0,
        "diskAvailable should be non-negative, got {avail}"
    );
    assert!(avail <= total, "diskAvailable should not exceed diskTotal");
    let load: i64 = printed[2].trim().parse().expect("loadAverage int");
    assert!(load >= 0, "loadAverage should be non-negative, got {load}");
}
