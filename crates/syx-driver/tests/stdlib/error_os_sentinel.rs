//! Os.hostname / Os.diskTotal throw OsError (no empty/-1 sentinels).
// was: m2_error_os_sentinel
//! ADR-159 / M2 W15: Os.diskAvailable / Os.primaryIpv4 throw OsError (no -1/empty sentinels).
//! ADR-161 / M2 W17: Os.userName / Os.homeDir throw OsError (no empty sentinels).
//! ADR-170 / M2 W26: Os.memoryTotal throw OsError (SYX_OS_MEMORY_TOTAL_FORCE_ERR inject).
//! ADR-171 / M2 W27: Os.memoryAvailable throw OsError (SYX_OS_MEMORY_AVAILABLE_FORCE_ERR inject).
//! ADR-172 / M2 W28: Os.uptimeMs throw OsError (SYX_OS_UPTIME_MS_FORCE_ERR inject).
//! ADR-173 / M2 W29: Os.loadAverage throw OsError (SYX_OS_LOAD_AVERAGE_FORCE_ERR inject).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W133 / ADR-477: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let os = include_str!("../../../../packages/os/mod.sy");
    format!("{os}\n{body}")
}

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn remove(key: &'static str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: tests run with --test-threads=1; restore in Drop.
        unsafe { std::env::remove_var(key) };
        Self { key, prev }
    }

    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: tests run with --test-threads=1; restore in Drop.
        unsafe { std::env::set_var(key, value) };
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => unsafe { std::env::set_var(self.key, v) },
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}

#[test]
fn m2_error_os_disk_total_invalid_throws() {
    // Wave A W43 / ADR-387 / A-03: Os.diskTotal missing-path product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.diskTotal(path: "/tmp/syx_m2_w14_nosuch_xyz")
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid diskTotal must exact: {out}");
}

#[test]
fn m2_error_os_disk_available_invalid_throws() {
    // Wave A W43 / ADR-387 / A-03: Os.diskAvailable missing-path product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.diskAvailable(path: "/tmp/syx_m2_w15_nosuch_xyz")
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid diskAvailable must exact: {out}");
}

#[test]
fn m2_error_os_hostname_ok_or_throws() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let h = Os.hostname()
    print("ok")
    print(h)
  } catch (err) {
    print("caught")
    print(err.message)
    print(err.code)
  }
}
"#,
    ));
    if out.contains("ok") {
        let lines: Vec<&str> = out.lines().collect();
        let name = lines.get(1).copied().unwrap_or("");
        assert!(
            !name.is_empty() && name != "ok",
            "hostname success must be non-empty: {out}"
        );
    } else {
        assert!(
            out.contains("caught")
                && (out.contains("os hostname failed") || out.contains("1")),
            "hostname must catch code 1 when unavailable: {out}"
        );
    }
}

#[test]
fn m2_error_os_primary_ipv4_ok_or_throws() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let ip = Os.primaryIpv4()
    print("ok")
    print(ip)
  } catch (err) {
    print("caught")
    print(err.message)
    print(err.code)
  }
}
"#,
    ));
    if out.contains("ok") {
        let lines: Vec<&str> = out.lines().collect();
        let ip = lines.get(1).copied().unwrap_or("");
        assert!(
            !ip.is_empty() && ip != "ok",
            "primaryIpv4 success must be non-empty: {out}"
        );
    } else {
        assert!(
            out.contains("caught")
                && (out.contains("os primaryIpv4 failed") || out.contains("4")),
            "primaryIpv4 must catch code 4 when unavailable: {out}"
        );
    }
}

#[test]
fn m2_error_os_user_name_missing_throws() {
    // Wave A W44 / ADR-388 / A-03: Os.userName missing-env product exact.
    let _u = EnvGuard::remove("USER");
    let _n = EnvGuard::remove("USERNAME");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let u = Os.userName()
    print("UNEXPECTED_OK")
    print(u)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "missing userName must exact: {out}");
}

#[test]
fn m2_error_os_home_dir_missing_throws() {
    // Wave A W44 / ADR-388 / A-03: Os.homeDir missing-env product exact.
    let _h = EnvGuard::remove("HOME");
    let _p = EnvGuard::remove("USERPROFILE");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let d = Os.homeDir()
    print("UNEXPECTED_OK")
    print(d)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "missing homeDir must exact: {out}");
}

#[test]
fn m2_error_os_memory_total_ok() {
    // Wave A W45 / ADR-389 / A-03: Os.memoryTotal ok-path product exact (drop print(n)).
    let _force = EnvGuard::remove("SYX_OS_MEMORY_TOTAL_FORCE_ERR");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.memoryTotal()
    print("ok")
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "memoryTotal ok-path must exact: {out}");
}

#[test]
fn m2_error_os_memory_total_force_err_throws() {
    // Wave A W41 / ADR-385 / A-03: Os.memoryTotal force-err product exact.
    let _force = EnvGuard::set("SYX_OS_MEMORY_TOTAL_FORCE_ERR", "1");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.memoryTotal()
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "force-err memoryTotal must exact: {out}");
}

#[test]
fn m2_error_os_memory_available_ok() {
    // Wave A W45 / ADR-389 / A-03: Os.memoryAvailable ok-path product exact (drop print(n)).
    let _force = EnvGuard::remove("SYX_OS_MEMORY_AVAILABLE_FORCE_ERR");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.memoryAvailable()
    print("ok")
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "memoryAvailable ok-path must exact: {out}");
}

#[test]
fn m2_error_os_memory_available_force_err_throws() {
    // Wave A W41 / ADR-385 / A-03: Os.memoryAvailable force-err product exact.
    let _force = EnvGuard::set("SYX_OS_MEMORY_AVAILABLE_FORCE_ERR", "1");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.memoryAvailable()
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(
        out, "caught",
        "force-err memoryAvailable must exact: {out}"
    );
}

#[test]
fn m2_error_os_memory_available_force_err_display_throws() {
    // Wave A W134 / ADR-478 / A-03: Os.memoryAvailable force-err Display product exact.
    let _force = EnvGuard::set("SYX_OS_MEMORY_AVAILABLE_FORCE_ERR", "1");
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Os.memoryAvailable()
}
"#,
    ));
    assert_eq!(
        err, "[R007] os memoryAvailable failed",
        "force-err memoryAvailable Display must exact: {err}"
    );
}

#[test]
fn m2_error_os_uptime_ms_ok() {
    // Wave A W46 / ADR-390 / A-03: Os.uptimeMs ok-path product exact (drop print(n)).
    let _force = EnvGuard::remove("SYX_OS_UPTIME_MS_FORCE_ERR");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.uptimeMs()
    print("ok")
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "uptimeMs ok-path must exact: {out}");
}

#[test]
fn m2_error_os_uptime_ms_force_err_throws() {
    // Wave A W42 / ADR-386 / A-03: Os.uptimeMs force-err product exact.
    let _force = EnvGuard::set("SYX_OS_UPTIME_MS_FORCE_ERR", "1");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.uptimeMs()
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "force-err uptimeMs must exact: {out}");
}

#[test]
fn m2_error_os_uptime_ms_force_err_display_throws() {
    // Wave A W133 / ADR-477 / A-03: Os.uptimeMs force-err Display product exact.
    let _force = EnvGuard::set("SYX_OS_UPTIME_MS_FORCE_ERR", "1");
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Os.uptimeMs()
}
"#,
    ));
    assert_eq!(
        err, "[R007] os uptimeMs failed",
        "force-err uptimeMs Display must exact: {err}"
    );
}

#[test]
fn m2_error_os_memory_total_force_err_display_throws() {
    // Wave A W133 / ADR-477 / A-03: Os.memoryTotal force-err Display product exact.
    let _force = EnvGuard::set("SYX_OS_MEMORY_TOTAL_FORCE_ERR", "1");
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Os.memoryTotal()
}
"#,
    ));
    assert_eq!(
        err, "[R007] os memoryTotal failed",
        "force-err memoryTotal Display must exact: {err}"
    );
}

#[test]
fn m2_error_os_load_average_ok() {
    // Wave A W46 / ADR-390 / A-03: Os.loadAverage ok-path product exact (drop print(n)).
    let _force = EnvGuard::remove("SYX_OS_LOAD_AVERAGE_FORCE_ERR");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.loadAverage()
    print("ok")
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "loadAverage ok-path must exact: {out}");
}

#[test]
fn m2_error_os_load_average_force_err_throws() {
    // Wave A W42 / ADR-386 / A-03: Os.loadAverage force-err product exact.
    let _force = EnvGuard::set("SYX_OS_LOAD_AVERAGE_FORCE_ERR", "1");
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Os.loadAverage()
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "force-err loadAverage must exact: {out}");
}

#[test]
fn m2_error_os_load_average_force_err_display_throws() {
    // Wave A W134 / ADR-478 / A-03: Os.loadAverage force-err Display product exact.
    let _force = EnvGuard::set("SYX_OS_LOAD_AVERAGE_FORCE_ERR", "1");
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Os.loadAverage()
}
"#,
    ));
    assert_eq!(
        err, "[R007] os loadAverage failed",
        "force-err loadAverage Display must exact: {err}"
    );
}

#[test]
fn m2_error_os_hostname_force_err_display_throws() {
    // Wave A W137 / ADR-481 / A-03: Os.hostname force-err Display product exact.
    let _force = EnvGuard::set("SYX_OS_HOSTNAME_FORCE_ERR", "1");
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Os.hostname()
}
"#,
    ));
    assert_eq!(
        err, "[R007] os hostname failed",
        "force-err hostname Display must exact: {err}"
    );
}

#[test]
fn m2_error_os_primary_ipv4_force_err_display_throws() {
    // Wave A W137 / ADR-481 / A-03: Os.primaryIpv4 force-err Display product exact.
    let _force = EnvGuard::set("SYX_OS_PRIMARY_IPV4_FORCE_ERR", "1");
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Os.primaryIpv4()
}
"#,
    ));
    assert_eq!(
        err, "[R007] os primaryIpv4 failed",
        "force-err primaryIpv4 Display must exact: {err}"
    );
}
