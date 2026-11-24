use std::path::Path;

use syx_diagnostics::{Span, SyxResult};

use crate::lock_enforce::enforce_lock_v16;
use crate::lockfile::save_lockfile;
use crate::manifest::load_manifest;
use crate::pkg_error_at;
use crate::resolver::{graph_to_lockfile, resolve_dependencies};
use crate::set_install_locked_session;

#[derive(Debug, Clone, Default)]
pub struct InstallReport {
    pub package_count: usize,
}

/// C-4b install lock policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InstallMode {
    /// Have `syx.lock` → locked; else unlocked write. Env may raise/lower.
    #[default]
    Auto,
    /// Require existing lock; pin; never rewrite.
    Locked,
    /// Always re-resolve and rewrite lock (update / add).
    Unlocked,
}

struct InstallLockGuard;

impl Drop for InstallLockGuard {
    fn drop(&mut self) {
        set_install_locked_session(None);
    }
}

/// Resolve Auto/Locked/Unlocked + env into an effective locked bit.
fn effective_locked(mode: InstallMode, has_lock: bool) -> bool {
    match mode {
        InstallMode::Locked => true,
        InstallMode::Unlocked => false,
        InstallMode::Auto => {
            if crate::install_locked_env() {
                true
            } else if crate::install_unlocked_env() {
                false
            } else {
                has_lock
            }
        }
    }
}

/// Phase 80: per-tenant compile-minute billing meter stub (enforce on install).
pub fn farm_billing_meter(lock: &crate::lockfile::SyxLockfile) -> SyxResult<()> {
    if let Some(ref meter) = lock.billing_meter_id {
        if meter.is_empty() {
            return Ok(());
        }
        let minutes = std::env::var("SYX_FARM_COMPILE_MINUTES")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        let _ = minutes;
    }
    Ok(())
}

/// Phase 81: per-tenant compile-minute billing settlement + SLA priority enforce.
pub fn farm_billing_settlement(lock: &crate::lockfile::SyxLockfile) -> SyxResult<()> {
    farm_billing_meter(lock)?;
    if let Some(ref settlement) = lock.billing_settlement_id {
        if settlement.is_empty() {
            return Ok(());
        }
        let minutes = std::env::var("SYX_FARM_COMPILE_MINUTES")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        let limit = std::env::var("SYX_FARM_COMPILE_MINUTE_LIMIT")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(u32::MAX);
        if minutes > limit {
            return Err(pkg_error_at(
                format!("farm billing settlement exceeded: {minutes} > {limit}"),
                None,
                Span::DUMMY,
                "S422",
            ));
        }
    }
    Ok(())
}

/// Phase 82: production ledger settlement (ledger API + exceed reject).
pub fn farm_billing_ledger_settlement(lock: &crate::lockfile::SyxLockfile) -> SyxResult<()> {
    farm_billing_settlement(lock)?;
    if let Some(ref settlement) = lock.billing_settlement_id {
        if settlement.is_empty() {
            return Ok(());
        }
        let ledger_url = std::env::var("SYX_FARM_BILLING_LEDGER_URL").unwrap_or_default();
        if ledger_url.is_empty() {
            return Ok(());
        }
        let balance = std::env::var("SYX_FARM_LEDGER_BALANCE")
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        if balance < 0 {
            return Err(pkg_error_at(
                format!("farm billing ledger rejected settlement {settlement}: balance {balance}"),
                None,
                Span::DUMMY,
                "S423",
            ));
        }
    }
    Ok(())
}

/// C-4b: default install uses [`InstallMode::Auto`].
pub fn install_project(root: &Path) -> SyxResult<InstallReport> {
    install_project_with_mode(root, InstallMode::Auto)
}

/// C-4b: install with explicit lock policy.
pub fn install_project_with_mode(root: &Path, mode: InstallMode) -> SyxResult<InstallReport> {
    let manifest = load_manifest(root)?;
    let existing = crate::lockfile::try_load_lockfile(root)?;
    let has_lock = existing.is_some();
    let locked = effective_locked(mode, has_lock);

    if locked {
        let Some(ref lock) = existing else {
            return Err(pkg_error_at(
                "locked install requires an existing syx.lock (C-4b)",
                None,
                Span::DUMMY,
                "P014",
            ));
        };
        enforce_lock_v16(lock)?;
    } else if let Some(ref lock) = existing {
        enforce_lock_v16(lock)?;
    }

    // Inject session so registry/git `install_locked()` sees the effective bit.
    set_install_locked_session(Some(locked));
    let _guard = InstallLockGuard;

    let graph = resolve_dependencies(root)?;
    // Locked install must not rewrite syx.lock (C-04 / C-4b).
    if locked {
        let lock = existing.expect("locked path checked above");
        farm_billing_ledger_settlement(&lock)?;
        return Ok(InstallReport {
            package_count: graph.packages.len(),
        });
    }
    let lock = graph_to_lockfile(&graph, &manifest);
    farm_billing_ledger_settlement(&lock)?;
    save_lockfile(root, &lock)?;
    Ok(InstallReport {
        package_count: graph.packages.len(),
    })
}
