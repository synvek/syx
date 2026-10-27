//! Phase 76: distributed cache remote report for `syx doctor --cache-remote-report`.

#[derive(Debug, Clone, Default)]
pub struct CacheRemoteReport {
    pub last_push: Option<String>,
    pub last_fetch: Option<String>,
    pub push_enabled: bool,
}

pub fn run_cache_remote_report() -> CacheRemoteReport {
    let (last_push, last_fetch) = crate::cache_remote_report();
    CacheRemoteReport {
        last_push,
        last_fetch,
        push_enabled: std::env::var("SYX_CACHE_REMOTE_PUSH")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true),
    }
}

pub fn print_cache_remote_report(report: &CacheRemoteReport) {
    println!("SYX cache remote report (Phase 76)");
    println!(
        "  push enabled: {}",
        if report.push_enabled { "yes" } else { "no" }
    );
    println!(
        "  last push: {}",
        report.last_push.as_deref().unwrap_or("(none)")
    );
    println!(
        "  last fetch: {}",
        report.last_fetch.as_deref().unwrap_or("(none)")
    );
}
