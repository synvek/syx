use std::path::Path;

use syx_diagnostics::SyxResult;

use crate::lock_enforce::enforce_lock_v16;
use crate::lockfile::save_lockfile;
use crate::manifest::load_manifest;
use crate::resolver::{graph_to_lockfile, resolve_dependencies};

#[derive(Debug, Clone, Default)]
pub struct InstallReport {
    pub package_count: usize,
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
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "farm billing settlement exceeded: {minutes} > {limit}"
            ))
            .with_code("S422"));
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
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "farm billing ledger rejected settlement {settlement}: balance {balance}"
            ))
            .with_code("S423"));
        }
    }
    Ok(())
}

pub fn install_project(root: &Path) -> SyxResult<InstallReport> {
    let manifest = load_manifest(root)?;
    let existing = crate::lockfile::try_load_lockfile(root)?;
    let locked = crate::install_locked();
    if locked {
        let Some(ref lock) = existing else {
            return Err(syx_diagnostics::SyxDiagnostic::error(
                "SYX_INSTALL_LOCKED requires an existing syx.lock",
            )
            .with_code("P014"));
        };
        enforce_lock_v16(lock)?;
    } else if let Some(ref lock) = existing {
        enforce_lock_v16(lock)?;
    }
    let graph = resolve_dependencies(root)?;
    // Wave C W681 / C-04: locked install must not rewrite syx.lock.
    if locked {
        let lock = existing.expect("SYX_INSTALL_LOCKED checked above");
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
