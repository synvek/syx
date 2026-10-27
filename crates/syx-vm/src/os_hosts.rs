//! Phase 133 Track E: `Os` runtime queries (cpu / memory / user).
//! Phase 136 Track D: hostname / uptimeMs (`stdlib-os-v2:hostname-uptime-mvp`).
//! Phase 137 Track D: disk / load (`stdlib-os-v3:disk-load-mvp`).
//! Phase 138 Track D: primary IPv4 (`stdlib-os-v4:network-mvp`).
//! M2 W26 / ADR-170: memoryTotal fail-closed (inject via SYX_OS_MEMORY_TOTAL_FORCE_ERR).
//! M2 W27 / ADR-171: memoryAvailable fail-closed (inject via SYX_OS_MEMORY_AVAILABLE_FORCE_ERR).
//! M2 W28 / ADR-172: uptimeMs fail-closed (inject via SYX_OS_UPTIME_MS_FORCE_ERR).
//! M2 W29 / ADR-173: loadAverage fail-closed (inject via SYX_OS_LOAD_AVERAGE_FORCE_ERR).
//! Wave A W137 / ADR-481: hostname/primaryIpv4 fail-closed inject (SYX_OS_HOSTNAME_FORCE_ERR / SYX_OS_PRIMARY_IPV4_FORCE_ERR).

use syx_diagnostics::SyxResult;

use crate::{platform_int_value, Value};

pub fn host_os_cpu_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = std::thread::available_parallelism()
        .map(|n| n.get() as i64)
        .unwrap_or(1);
    stack.push(platform_int_value(n));
    Ok(())
}

pub fn host_os_memory_total(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-170: fail-closed Err (no -1 success sentinel). Test inject: SYX_OS_MEMORY_TOTAL_FORCE_ERR.
    if std::env::var("SYX_OS_MEMORY_TOTAL_FORCE_ERR")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os memoryTotal forced unavailable")
                .with_code("R003"),
        );
    }
    let bytes = memory_total_bytes().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error("os memoryTotal unavailable").with_code("R003")
    })?;
    stack.push(platform_int_value(bytes));
    Ok(())
}

pub fn host_os_memory_available(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-171: fail-closed Err (no -1 success sentinel). Test inject: SYX_OS_MEMORY_AVAILABLE_FORCE_ERR.
    if std::env::var("SYX_OS_MEMORY_AVAILABLE_FORCE_ERR")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os memoryAvailable forced unavailable")
                .with_code("R003"),
        );
    }
    let bytes = memory_available_bytes().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error("os memoryAvailable unavailable").with_code("R003")
    })?;
    stack.push(platform_int_value(bytes));
    Ok(())
}

pub fn host_os_user_name(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-161: fail-closed Err (no empty-string success sentinel).
    let name = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error("os userName unavailable").with_code("R003")
        })?;
    stack.push(Value::Str(name));
    Ok(())
}

pub fn host_os_home_dir(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-161: fail-closed Err (no empty-string success sentinel).
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error("os homeDir unavailable").with_code("R003")
        })?;
    stack.push(Value::Str(home));
    Ok(())
}

pub fn host_os_hostname(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-158: fail-closed Err (no empty-string success sentinel).
    // Wave A W137 / ADR-481: test inject SYX_OS_HOSTNAME_FORCE_ERR.
    if std::env::var("SYX_OS_HOSTNAME_FORCE_ERR")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os hostname forced unavailable")
                .with_code("R003"),
        );
    }
    let name = hostname().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error("os hostname unavailable").with_code("R003")
    })?;
    if name.is_empty() {
        return Err(syx_diagnostics::SyxDiagnostic::error("os hostname empty").with_code("R003"));
    }
    stack.push(Value::Str(name));
    Ok(())
}

pub fn host_os_uptime_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-172: fail-closed Err (no -1 success sentinel). Test inject: SYX_OS_UPTIME_MS_FORCE_ERR.
    if std::env::var("SYX_OS_UPTIME_MS_FORCE_ERR")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os uptimeMs forced unavailable")
                .with_code("R003"),
        );
    }
    let ms = uptime_ms().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error("os uptimeMs unavailable").with_code("R003")
    })?;
    stack.push(platform_int_value(ms));
    Ok(())
}

pub fn host_os_disk_total(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    // ADR-158: fail-closed Err (no -1 success sentinel).
    let bytes = disk_total_bytes(&path).ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error(format!("os diskTotal failed for '{path}'"))
            .with_code("R003")
    })?;
    stack.push(platform_int_value(bytes));
    Ok(())
}

pub fn host_os_disk_available(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    // ADR-159: fail-closed Err (no -1 success sentinel).
    let bytes = disk_available_bytes(&path).ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error(format!("os diskAvailable failed for '{path}'"))
            .with_code("R003")
    })?;
    stack.push(platform_int_value(bytes));
    Ok(())
}

pub fn host_os_load_average(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-173: fail-closed Err (no -1 success sentinel). Test inject: SYX_OS_LOAD_AVERAGE_FORCE_ERR.
    if std::env::var("SYX_OS_LOAD_AVERAGE_FORCE_ERR")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os loadAverage forced unavailable")
                .with_code("R003"),
        );
    }
    let scaled = load_average_scaled().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error("os loadAverage unavailable").with_code("R003")
    })?;
    stack.push(platform_int_value(scaled));
    Ok(())
}

pub fn host_os_primary_ipv4(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-159: fail-closed Err (no empty-string success sentinel).
    // Wave A W137 / ADR-481: test inject SYX_OS_PRIMARY_IPV4_FORCE_ERR.
    if std::env::var("SYX_OS_PRIMARY_IPV4_FORCE_ERR")
        .ok()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os primaryIpv4 forced unavailable")
                .with_code("R003"),
        );
    }
    let ip = primary_ipv4().ok_or_else(|| {
        syx_diagnostics::SyxDiagnostic::error("os primaryIpv4 unavailable").with_code("R003")
    })?;
    if ip.is_empty() {
        return Err(
            syx_diagnostics::SyxDiagnostic::error("os primaryIpv4 empty").with_code("R003"),
        );
    }
    stack.push(Value::Str(ip));
    Ok(())
}

fn hostname() -> Option<String> {
    if let Ok(h) = std::env::var("HOSTNAME") {
        let t = h.trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    if let Ok(h) = std::env::var("COMPUTERNAME") {
        let t = h.trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    let out = std::process::Command::new("hostname").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn uptime_ms() -> Option<i64> {
    #[cfg(target_os = "linux")]
    {
        let text = std::fs::read_to_string("/proc/uptime").ok()?;
        let secs: f64 = text.split_whitespace().next()?.parse().ok()?;
        return Some((secs * 1000.0) as i64);
    }
    #[cfg(target_os = "macos")]
    {
        // kern.boottime → seconds since boot via sysctl; convert to uptime ms.
        let out = std::process::Command::new("sysctl")
            .args(["-n", "kern.boottime"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let s = String::from_utf8_lossy(&out.stdout);
        // Typical: "{ sec = 1710000000, usec = 123456 } ..."
        let sec = s
            .split("sec =")
            .nth(1)?
            .split(',')
            .next()?
            .trim()
            .parse::<i64>()
            .ok()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs() as i64;
        let up_secs = now.saturating_sub(sec);
        return Some(up_secs.saturating_mul(1000));
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

fn disk_total_bytes(path: &str) -> Option<i64> {
    let path = if path.is_empty() { "." } else { path };
    statvfs(path).map(|st| st.total_bytes())
}

fn disk_available_bytes(path: &str) -> Option<i64> {
    let path = if path.is_empty() { "." } else { path };
    statvfs(path).map(|st| st.avail_bytes())
}

fn primary_ipv4() -> Option<String> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        use std::net::{IpAddr, UdpSocket};
        let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
        sock.connect("8.8.8.8:80").ok()?;
        let local = sock.local_addr().ok()?;
        match local.ip() {
            IpAddr::V4(v4) if !v4.is_loopback() && !v4.is_unspecified() => Some(v4.to_string()),
            _ => None,
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

fn load_average_scaled() -> Option<i64> {
    #[cfg(unix)]
    {
        extern "C" {
            fn getloadavg(loadavg: *mut f64, nelem: i32) -> i32;
        }
        let mut load = [0.0f64; 1];
        let n = unsafe { getloadavg(load.as_mut_ptr(), 1) };
        if n != 1 {
            return None;
        }
        return Some((load[0] * 100.0).round() as i64);
    }
    #[cfg(not(unix))]
    {
        None
    }
}

#[repr(C)]
#[cfg(target_os = "macos")]
struct StatVfs {
    f_bsize: u64,
    f_frsize: u64,
    f_blocks: u32,
    f_bfree: u32,
    f_bavail: u32,
}

#[repr(C)]
#[cfg(not(target_os = "macos"))]
struct StatVfs {
    f_bsize: u64,
    f_frsize: u64,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
}

impl StatVfs {
    fn total_bytes(&self) -> i64 {
        (self.blocks() as i64).saturating_mul(self.f_frsize as i64)
    }

    fn avail_bytes(&self) -> i64 {
        (self.bavail() as i64).saturating_mul(self.f_frsize as i64)
    }

    #[cfg(target_os = "macos")]
    fn blocks(&self) -> u64 {
        self.f_blocks as u64
    }

    #[cfg(not(target_os = "macos"))]
    fn blocks(&self) -> u64 {
        self.f_blocks
    }

    #[cfg(target_os = "macos")]
    fn bavail(&self) -> u64 {
        self.f_bavail as u64
    }

    #[cfg(not(target_os = "macos"))]
    fn bavail(&self) -> u64 {
        self.f_bavail
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn statvfs(path: &str) -> Option<StatVfs> {
    use std::ffi::CString;
    use std::mem::MaybeUninit;
    extern "C" {
        fn statvfs(path: *const i8, buf: *mut StatVfs) -> i32;
    }
    let c = CString::new(path).ok()?;
    let mut st = MaybeUninit::<StatVfs>::uninit();
    if unsafe { statvfs(c.as_ptr(), st.as_mut_ptr()) } != 0 {
        return None;
    }
    Some(unsafe { st.assume_init() })
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn statvfs(_path: &str) -> Option<StatVfs> {
    None
}

fn memory_total_bytes() -> Option<i64> {
    #[cfg(target_os = "linux")]
    {
        return parse_meminfo_kb("MemTotal:").map(|kb| kb * 1024);
    }
    #[cfg(target_os = "macos")]
    {
        return sysctl_i64("hw.memsize");
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

fn memory_available_bytes() -> Option<i64> {
    #[cfg(target_os = "linux")]
    {
        return parse_meminfo_kb("MemAvailable:")
            .or_else(|| parse_meminfo_kb("MemFree:"))
            .map(|kb| kb * 1024);
    }
    #[cfg(target_os = "macos")]
    {
        // page_size * free page count — best-effort; unavailable → None → -1.
        let page = sysctl_i64("hw.pagesize")?;
        let free = sysctl_i64("vm.page_free_count")?;
        return Some(page.saturating_mul(free));
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
fn parse_meminfo_kb(key: &str) -> Option<i64> {
    let text = std::fs::read_to_string("/proc/meminfo").ok()?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix(key) {
            let num: String = rest.chars().filter(|c| c.is_ascii_digit()).collect();
            return num.parse().ok();
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn sysctl_i64(name: &str) -> Option<i64> {
    let out = std::process::Command::new("sysctl")
        .args(["-n", name])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    s.trim().parse().ok()
}
