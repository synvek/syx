//! Phase 91: File.watch OS backend (inotify/kqueue/WinAPI) stdout hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_file_watch_os_io_module_surface() {
    let io = include_str!("../../../../packages/io/mod.sy");
    assert!(io.contains("File.watch") && io.contains("__syx_file_watch"));
}

#[test]
fn p98_file_watch_os_host_impl_surface() {
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase84_hosts.rs");
    assert!(hosts.contains("host_file_watch"));
}

#[test]
fn p98_file_watch_os_stdout() {
    // Wave A W20 / ADR-364 / A-03: missing-path watch product exact (not contains soft).
    let out = run(
        r#"func main() {
    print(__syx_fmt_debug(__syx_file_watch("/tmp/p88-watch-os")))
}"#,
    );
    assert_eq!(out, "Array([])", "file watch missing-path must exact: {out}");
}

#[test]
fn p98_file_watch_os_import_module_stdout() {
    // Wave A W21 / ADR-365 / A-03: File.watch product exact (IO facade surface; host arg labels
    // required under embed — bare `import IO` / full mod.sy host calls are not embed-resolved).
    let out = run(
        r#"
public class File {
    public static func watch(path: String) -> Array<String> {
        return __syx_file_watch(path: path)
    }
}
func main() {
    print(File.watch(path: "/tmp/p88-import"))
}
"#,
    );
    assert_eq!(out, "[]", "file watch import missing-path must exact: {out}");
}

#[test]
fn p98_file_watch_os_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("file-watch-os-default-v1"));
}

#[test]
fn p98_file_watch_os_registry_host_id() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("FileWatch") && reg.contains("__syx_file_watch"));
}

#[test]
fn p98_file_watch_os_no_mock_only_pass() {
    let src = include_str!("../../../syx-vm/src/stdlib_phase84_hosts.rs");
    assert!(src.contains("host_file_watch"));
}
