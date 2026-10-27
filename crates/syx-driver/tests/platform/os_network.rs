//! Os.primaryIpv4() MVP (`stdlib-os-v4:network-mvp`).

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
fn os_network_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("stdlib-os-v4:network-mvp"));
    assert!(lib.contains("heap_v110_phase138_track_d_os_network_fingerprint"));
    let os = include_str!("../../../../packages/os/mod.sy");
    assert!(os.contains("primaryIpv4"));
    assert!(os.contains("__syx_os_primary_ipv4"));
    let hosts = include_str!("../../../syx-vm/src/os_hosts.rs");
    assert!(hosts.contains("host_os_primary_ipv4"));
    assert!(hosts.contains("primary_ipv4"));
    let registry = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(registry.contains("OsPrimaryIpv4"));
}

#[test]
fn os_network_primary_ipv4_runs_without_crash() {
    let printed = run(&with_os(
        r#"
func main() {
    try {
        print(Os.primaryIpv4())
    } catch (e) {
        print("ipv4-unavailable")
    }
}
"#,
    ))
    .expect("os primaryIpv4 should run");
    let ip = printed[0].trim();
    // ADR-159: success is non-empty dotted IPv4, or catch marker (no empty-string sentinel).
    if ip == "ipv4-unavailable" {
        return;
    }
    assert!(
        !ip.is_empty() && ip.chars().all(|c| c.is_ascii_digit() || c == '.'),
        "primaryIpv4 must be dotted IPv4 or unavailable marker, got {ip:?}"
    );
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let parts: Vec<&str> = ip.split('.').collect();
        assert_eq!(parts.len(), 4, "expected IPv4 dotted quad, got {ip:?}");
        for part in parts {
            let _: u8 = part.parse().expect("ipv4 octet");
        }
        assert_ne!(ip, "127.0.0.1", "primaryIpv4 should prefer non-loopback when available");
    }
}

#[test]
fn os_network_remains_importable_surface() {
    let manifest = include_str!("../../../../packages/internal/manifest.json");
    assert!(
        manifest.contains("\"Os\"") || manifest.contains("Os"),
        "Os must remain in importable_modules"
    );
    let os = include_str!("../../../../packages/os/mod.sy");
    assert!(os.contains("public class Os"));
    assert!(
        os.contains("primaryIpv4")
            && os.contains("diskTotal")
            && os.contains("hostname")
    );
}
