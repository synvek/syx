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
fn m2_error_os_facade_source_has_oserror_and_throws() {
    let src = include_str!("../../../../packages/os/mod.sy");
    assert!(src.contains("OsError"), "OsError type required");
    assert!(
        src.contains("hostname() -> String throws")
            || src.contains("func hostname() -> String throws"),
        "hostname must throws: {src}"
    );
    assert!(
        src.contains("diskTotal(path: String) -> Int throws")
            || src.contains("func diskTotal(path: String) -> Int throws"),
        "diskTotal must throws: {src}"
    );
    assert!(
        src.contains("diskAvailable(path: String) -> Int throws")
            || src.contains("func diskAvailable(path: String) -> Int throws"),
        "diskAvailable must throws: {src}"
    );
    assert!(
        src.contains("primaryIpv4() -> String throws")
            || src.contains("func primaryIpv4() -> String throws"),
        "primaryIpv4 must throws: {src}"
    );
    assert!(
        src.contains("userName() -> String throws")
            || src.contains("func userName() -> String throws"),
        "userName must throws: {src}"
    );
    assert!(
        src.contains("homeDir() -> String throws")
            || src.contains("func homeDir() -> String throws"),
        "homeDir must throws: {src}"
    );
    assert!(
        src.contains("memoryTotal() -> Int throws")
            || src.contains("func memoryTotal() -> Int throws"),
        "memoryTotal must throws: {src}"
    );
    assert!(
        src.contains("code: 7") && src.contains("os memoryTotal failed"),
        "OsError code 7 message required: {src}"
    );
    assert!(
        src.contains("memoryAvailable() -> Int throws")
            || src.contains("func memoryAvailable() -> Int throws"),
        "memoryAvailable must throws: {src}"
    );
    assert!(
        src.contains("code: 8") && src.contains("os memoryAvailable failed"),
        "OsError code 8 message required: {src}"
    );
    assert!(
        src.contains("uptimeMs() -> Int throws")
            || src.contains("func uptimeMs() -> Int throws"),
        "uptimeMs must throws: {src}"
    );
    assert!(
        src.contains("code: 9") && src.contains("os uptimeMs failed"),
        "OsError code 9 message required: {src}"
    );
    assert!(
        src.contains("loadAverage() -> Int throws")
            || src.contains("func loadAverage() -> Int throws"),
        "loadAverage must throws: {src}"
    );
    assert!(
        src.contains("code: 10") && src.contains("os loadAverage failed"),
        "OsError code 10 message required: {src}"
    );

    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    let os_idx = reg
        .find("type_name: \"Os\"")
        .expect("Os TypeMethod block");
    let slice = &reg[os_idx..];
    for method in [
        "hostname",
        "diskTotal",
        "diskAvailable",
        "primaryIpv4",
        "userName",
        "homeDir",
        "memoryTotal",
        "memoryAvailable",
        "uptimeMs",
        "loadAverage",
    ] {
        assert!(
            slice.contains(&format!("method: \"{method}\""))
                && slice
                    .split(&format!("method: \"{method}\""))
                    .nth(1)
                    .is_some_and(|s| s.contains("host: None")),
            "Os.{method} must host: None: {slice}"
        );
    }

    let hosts = include_str!("../../../syx-vm/src/os_hosts.rs");
    let hostname_fn = hosts
        .split("pub fn host_os_hostname")
        .nth(1)
        .expect("host_os_hostname");
    let hostname_body = hostname_fn.split("pub fn ").next().unwrap_or(hostname_fn);
    assert!(
        hostname_body.contains("with_code(\"R003\")")
            && !hostname_body.contains("unwrap_or_default()"),
        "host_os_hostname must Err (no unwrap_or_default): {hostname_body}"
    );

    let disk_fn = hosts
        .split("pub fn host_os_disk_total")
        .nth(1)
        .expect("host_os_disk_total");
    let disk_body = disk_fn.split("pub fn ").next().unwrap_or(disk_fn);
    assert!(
        disk_body.contains("with_code(\"R003\")") && !disk_body.contains("unwrap_or(-1)"),
        "host_os_disk_total must Err (no unwrap_or(-1)): {disk_body}"
    );

    let avail_fn = hosts
        .split("pub fn host_os_disk_available")
        .nth(1)
        .expect("host_os_disk_available");
    let avail_body = avail_fn.split("pub fn ").next().unwrap_or(avail_fn);
    assert!(
        avail_body.contains("with_code(\"R003\")") && !avail_body.contains("unwrap_or(-1)"),
        "host_os_disk_available must Err (no unwrap_or(-1)): {avail_body}"
    );

    let ipv4_fn = hosts
        .split("pub fn host_os_primary_ipv4")
        .nth(1)
        .expect("host_os_primary_ipv4");
    let ipv4_body = ipv4_fn.split("pub fn ").next().unwrap_or(ipv4_fn);
    assert!(
        ipv4_body.contains("with_code(\"R003\")") && !ipv4_body.contains("unwrap_or_default()"),
        "host_os_primary_ipv4 must Err (no unwrap_or_default): {ipv4_body}"
    );

    let user_fn = hosts
        .split("pub fn host_os_user_name")
        .nth(1)
        .expect("host_os_user_name");
    let user_body = user_fn.split("pub fn ").next().unwrap_or(user_fn);
    assert!(
        user_body.contains("with_code(\"R003\")") && !user_body.contains("unwrap_or_default()"),
        "host_os_user_name must Err (no unwrap_or_default): {user_body}"
    );

    let home_fn = hosts
        .split("pub fn host_os_home_dir")
        .nth(1)
        .expect("host_os_home_dir");
    let home_body = home_fn.split("pub fn ").next().unwrap_or(home_fn);
    assert!(
        home_body.contains("with_code(\"R003\")") && !home_body.contains("unwrap_or_default()"),
        "host_os_home_dir must Err (no unwrap_or_default): {home_body}"
    );

    let mem_fn = hosts
        .split("pub fn host_os_memory_total")
        .nth(1)
        .expect("host_os_memory_total");
    let mem_body = mem_fn.split("pub fn ").next().unwrap_or(mem_fn);
    assert!(
        mem_body.contains("SYX_OS_MEMORY_TOTAL_FORCE_ERR")
            && mem_body.contains("with_code(\"R003\")")
            && !mem_body.contains("unwrap_or(-1)"),
        "host_os_memory_total must inject+Err (no unwrap_or(-1)): {mem_body}"
    );

    let mem_avail_fn = hosts
        .split("pub fn host_os_memory_available")
        .nth(1)
        .expect("host_os_memory_available");
    let mem_avail_body = mem_avail_fn.split("pub fn ").next().unwrap_or(mem_avail_fn);
    assert!(
        mem_avail_body.contains("SYX_OS_MEMORY_AVAILABLE_FORCE_ERR")
            && mem_avail_body.contains("with_code(\"R003\")")
            && !mem_avail_body.contains("unwrap_or(-1)"),
        "host_os_memory_available must inject+Err (no unwrap_or(-1)): {mem_avail_body}"
    );

    let uptime_fn = hosts
        .split("pub fn host_os_uptime_ms")
        .nth(1)
        .expect("host_os_uptime_ms");
    let uptime_body = uptime_fn.split("pub fn ").next().unwrap_or(uptime_fn);
    assert!(
        uptime_body.contains("SYX_OS_UPTIME_MS_FORCE_ERR")
            && uptime_body.contains("with_code(\"R003\")")
            && !uptime_body.contains("unwrap_or(-1)"),
        "host_os_uptime_ms must inject+Err (no unwrap_or(-1)): {uptime_body}"
    );

    let load_fn = hosts
        .split("pub fn host_os_load_average")
        .nth(1)
        .expect("host_os_load_average");
    let load_body = load_fn.split("pub fn ").next().unwrap_or(load_fn);
    assert!(
        load_body.contains("SYX_OS_LOAD_AVERAGE_FORCE_ERR")
            && load_body.contains("with_code(\"R003\")")
            && !load_body.contains("unwrap_or(-1)"),
        "host_os_load_average must inject+Err (no unwrap_or(-1)): {load_body}"
    );
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
