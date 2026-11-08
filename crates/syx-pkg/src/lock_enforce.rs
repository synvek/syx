//! Phase 78: lock v12 marketplace + plugin epoch enforcement on install.

use syx_diagnostics::{Span, SyxResult};

use crate::lockfile::SyxLockfile;
use crate::pkg_error_at;

/// Reject install when env plugin epoch is stale vs lockfile.
pub fn enforce_plugin_epoch(lock: &SyxLockfile) -> SyxResult<()> {
    if lock.plugin_unload_epoch == 0 {
        return Ok(());
    }
    let env_epoch: u64 = std::env::var("SYX_PLUGIN_UNLOAD_EPOCH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if env_epoch < lock.plugin_unload_epoch {
        return Err(pkg_error_at(
            format!(
                "plugin unload epoch stale: lock requires {} but env has {}",
                lock.plugin_unload_epoch, env_epoch
            ),
            None,
            Span::DUMMY,
            "S412",
        ));
    }
    Ok(())
}

/// Phase 78: marketplace CDN base must be present when lock records one.
pub fn enforce_marketplace_cdn(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref cdn) = lock.marketplace_cdn_base {
        if cdn.is_empty() {
            return Ok(());
        }
        let env = std::env::var("SYX_MARKETPLACE_CDN").unwrap_or_default();
        if env.is_empty() {
            return Err(pkg_error_at(
                format!("marketplace CDN required: lock specifies {cdn}"),
                None,
                Span::DUMMY,
                "S413",
            ));
        }
    }
    Ok(())
}

pub fn enforce_lock_v12(lock: &SyxLockfile) -> SyxResult<()> {
    enforce_plugin_epoch(lock)?;
    enforce_marketplace_cdn(lock)?;
    Ok(())
}

/// Phase 79: reject install when tenant env mismatches lockfile.
pub fn enforce_tenant(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref tenant) = lock.tenant_id {
        if tenant.is_empty() {
            return Ok(());
        }
        let env = std::env::var("SYX_TENANT_ID").unwrap_or_default();
        if !env.is_empty() && env != *tenant {
            return Err(pkg_error_at(
                format!("tenant mismatch: lock requires {tenant} but env has {env}"),
                None,
                Span::DUMMY,
                "S414",
            ));
        }
    }
    Ok(())
}

/// Phase 79: rotation cron spec must be acknowledged when set.
pub fn enforce_rotation_cron(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref spec) = lock.rotation_cron_spec {
        if spec.is_empty() {
            return Ok(());
        }
        let ack = std::env::var("SYX_ROTATION_CRON_ACK").unwrap_or_default();
        if ack != *spec {
            return Err(pkg_error_at(
                format!("rotation cron stale: lock requires ack {spec}"),
                None,
                Span::DUMMY,
                "S415",
            ));
        }
    }
    Ok(())
}

pub fn enforce_lock_v13(lock: &SyxLockfile) -> SyxResult<()> {
    enforce_lock_v12(lock)?;
    enforce_tenant(lock)?;
    enforce_rotation_cron(lock)?;
    Ok(())
}

/// Phase 80: reject install when HSM key ref env mismatches lockfile.
pub fn enforce_hsm_key_ref(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref hsm) = lock.hsm_key_ref {
        if hsm.is_empty() {
            return Ok(());
        }
        let env = std::env::var("SYX_HSM_KEY_REF").unwrap_or_default();
        if !env.is_empty() && env != *hsm {
            return Err(pkg_error_at(
                format!("HSM key ref stale: lock requires {hsm} but env has {env}"),
                None,
                Span::DUMMY,
                "S416",
            ));
        }
    }
    Ok(())
}

/// Phase 80: billing meter id must be acknowledged when set.
pub fn enforce_billing_meter(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref meter) = lock.billing_meter_id {
        if meter.is_empty() {
            return Ok(());
        }
        let ack = std::env::var("SYX_BILLING_METER_ACK").unwrap_or_default();
        if ack != *meter {
            return Err(pkg_error_at(
                format!("billing meter stale: lock requires ack {meter}"),
                None,
                Span::DUMMY,
                "S417",
            ));
        }
    }
    Ok(())
}

pub fn enforce_lock_v14(lock: &SyxLockfile) -> SyxResult<()> {
    enforce_lock_v13(lock)?;
    enforce_hsm_key_ref(lock)?;
    enforce_billing_meter(lock)?;
    Ok(())
}

/// Phase 81: HSM rotation cron job must be acknowledged when set.
pub fn enforce_hsm_rotation_cron(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref job) = lock.hsm_rotation_cron_job {
        if job.is_empty() {
            return Ok(());
        }
        let ack = std::env::var("SYX_HSM_ROTATION_CRON_ACK").unwrap_or_default();
        if ack != *job {
            return Err(pkg_error_at(
                format!("HSM rotation cron stale: lock requires ack {job}"),
                None,
                Span::DUMMY,
                "S419",
            ));
        }
    }
    Ok(())
}

/// Phase 81: billing settlement id must match env when lock records one.
pub fn enforce_billing_settlement(lock: &SyxLockfile) -> SyxResult<()> {
    if let Some(ref settlement) = lock.billing_settlement_id {
        if settlement.is_empty() {
            return Ok(());
        }
        let env = std::env::var("SYX_BILLING_SETTLEMENT_ACK").unwrap_or_default();
        if env != *settlement {
            return Err(pkg_error_at(
                format!("billing settlement stale: lock requires ack {settlement}"),
                None,
                Span::DUMMY,
                "S420",
            ));
        }
    }
    Ok(())
}

/// Phase 81: marketplace plugin epoch must be current.
pub fn enforce_marketplace_plugin_epoch(lock: &SyxLockfile) -> SyxResult<()> {
    if lock.marketplace_plugin_epoch == 0 {
        return Ok(());
    }
    let env_epoch: u64 = std::env::var("SYX_MARKETPLACE_PLUGIN_EPOCH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if env_epoch < lock.marketplace_plugin_epoch {
        return Err(pkg_error_at(
            format!(
                "marketplace plugin epoch stale: lock requires {} but env has {}",
                lock.marketplace_plugin_epoch, env_epoch
            ),
            None,
            Span::DUMMY,
            "S421",
        ));
    }
    Ok(())
}

pub fn enforce_lock_v15(lock: &SyxLockfile) -> SyxResult<()> {
    enforce_lock_v14(lock)?;
    enforce_hsm_rotation_cron(lock)?;
    enforce_billing_settlement(lock)?;
    enforce_marketplace_plugin_epoch(lock)?;
    Ok(())
}

/// Phase 82: marketplace publish epoch must be current.
pub fn enforce_marketplace_publish_epoch(lock: &SyxLockfile) -> SyxResult<()> {
    if lock.marketplace_publish_epoch == 0 {
        return Ok(());
    }
    let env_epoch: u64 = std::env::var("SYX_MARKETPLACE_PUBLISH_EPOCH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if env_epoch < lock.marketplace_publish_epoch {
        return Err(pkg_error_at(
            format!(
                "marketplace publish epoch stale: lock requires {} but env has {}",
                lock.marketplace_publish_epoch, env_epoch
            ),
            None,
            Span::DUMMY,
            "S422",
        ));
    }
    Ok(())
}

pub fn enforce_lock_v16(lock: &SyxLockfile) -> SyxResult<()> {
    enforce_lock_v15(lock)?;
    enforce_marketplace_publish_epoch(lock)?;
    Ok(())
}
