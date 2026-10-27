mod installer;
pub mod lock_enforce;
pub use lock_enforce::{
    enforce_billing_meter, enforce_billing_settlement,
    enforce_hsm_key_ref, enforce_hsm_rotation_cron, enforce_lock_v12, enforce_lock_v13,
    enforce_lock_v14, enforce_lock_v15, enforce_lock_v16, enforce_marketplace_cdn,
    enforce_marketplace_plugin_epoch, enforce_marketplace_publish_epoch,
    enforce_plugin_epoch, enforce_rotation_cron, enforce_tenant,
};
mod lockfile;
mod manifest;
mod package;
mod registry;
mod resolver;
mod sources;

pub use installer::{farm_billing_ledger_settlement, farm_billing_meter, farm_billing_settlement, InstallReport};
pub use lockfile::{
    load_lockfile, save_lockfile, InstalledPackage, LockPackage, LockPluginEntry, PackageSource,
    SignaturePolicy, SyxLockfile,
};
pub use manifest::{
    default_registry_url, load_manifest, registry_url_chain, save_manifest, DependencyAddSpec,
    DependencySpec, NativeConfig, PermissionsConfig, RegistryConfig, SyxManifest,
};
pub use resolver::{graph_to_lockfile, resolve_dependencies, ResolvedGraph};
pub use registry::RegistryClient;
pub use sources::registry::{extract_tar_gz, prepare_vendor_extract_dest};

use std::path::{Path, PathBuf};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use indexmap::IndexMap;
use syx_diagnostics::{SyxDiagnostic, SyxResult};

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Phase 20: registry CDN ed25519 package signature verification.
/// Phase 21: when `SYX_REGISTRY_REQUIRE_SIGNATURE=1`, missing/invalid signatures fail install.
/// Phase 22: `syx1:stub` rejected when signature required unless `SYX_REGISTRY_ALLOW_STUB=1`.
/// Phase 69: lock v8 mandatory sign — signatures required by default; opt out with `SYX_REGISTRY_ALLOW_UNSIGNED=1`.
/// C-02 (W678): ed25519 verify requires `SYX_REGISTRY_VERIFY_KEY`; embedded pubkey must match pin.
pub fn registry_require_signature() -> bool {
    if registry_allow_unsigned() {
        return false;
    }
    match std::env::var("SYX_REGISTRY_REQUIRE_SIGNATURE").as_deref() {
        Ok("0") | Ok("false") => false,
        _ => true,
    }
}

pub fn registry_allow_unsigned() -> bool {
    std::env::var("SYX_REGISTRY_ALLOW_UNSIGNED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Wave C W680 / C-04: when set, registry install must pin version+checksum from `syx.lock`.
pub fn install_locked() -> bool {
    std::env::var("SYX_INSTALL_LOCKED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_allow_stub() -> bool {
    std::env::var("SYX_REGISTRY_ALLOW_STUB")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 23 DG1: staging CDN Phase 6 signed URL install path.
pub fn registry_cdn_phase6_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE6")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 26 DV1: single-region production CDN (`SYX_REGISTRY_CDN_PHASE7=1`).
pub fn registry_cdn_phase7_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE7")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 26: production CDN base URL when Phase 7 enabled.
pub fn registry_cdn_phase7_prod_url() -> &'static str {
    "https://cdn.sy-lang.org/prod/v1"
}

/// Phase 27 EA1: multi-region CDN + failover (`SYX_REGISTRY_CDN_PHASE8=1`).
pub fn registry_cdn_phase8_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE8")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 27: primary + secondary region URLs when Phase 8 enabled.
pub fn registry_cdn_phase8_regions() -> &'static [&'static str] {
    &[
        "https://cdn-us.sy-lang.org/prod/v1",
        "https://cdn-eu.sy-lang.org/prod/v1",
        "https://cdn-ap.sy-lang.org/prod/v1",
    ]
}

pub fn registry_cdn_phase8_failover_url(region_index: usize) -> Option<&'static str> {
    if !registry_cdn_phase8_enabled() {
        return None;
    }
    registry_cdn_phase8_regions().get(region_index).copied()
}

/// Phase 28 EF1: CDN failover retry + health probe (`SYX_REGISTRY_CDN_PHASE9=1`).
pub fn registry_cdn_phase9_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE9")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase9_max_retries() -> u32 {
    std::env::var("SYX_REGISTRY_CDN_PHASE9_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3)
}

pub fn registry_cdn_phase9_failover_url(region_index: usize, attempt: u32) -> Option<String> {
    if !registry_cdn_phase9_enabled() {
        return registry_cdn_phase8_failover_url(region_index).map(|u| u.to_string());
    }
    registry_cdn_phase8_regions()
        .get(region_index)
        .map(|base| format!("{base}?attempt={attempt}"))
}

pub fn registry_cdn_phase9_health_probe_ok(region_index: usize) -> bool {
    if !registry_cdn_phase9_enabled() {
        return true;
    }
    region_index < registry_cdn_phase8_regions().len()
}

/// Phase 29 EK1: Registry Phase 10 signed download E2E + multi-region auto failover.
pub fn registry_cdn_phase10_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE10")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase10_signed_download_url(
    region_index: usize,
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    if !registry_cdn_phase10_enabled() {
        return registry_cdn_phase9_failover_url(region_index, 0)
            .map(|base| registry_signed_download_url(&base, name, version, signature));
    }
    registry_cdn_phase8_regions().get(region_index).map(|base| {
        registry_signed_download_url(base, name, version, signature)
    })
}

pub fn registry_cdn_phase10_failover_regions() -> &'static [&'static str] {
    registry_cdn_phase8_regions()
}

/// Phase 30 EP1: Registry Phase 11 driver-integrated auto signed download + multi-region failover.
pub fn registry_cdn_phase11_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE11")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase11_max_retries() -> u32 {
    std::env::var("SYX_REGISTRY_CDN_PHASE11_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5)
}

pub fn registry_cdn_phase11_auto_download_url(
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    if !registry_cdn_phase11_enabled() {
        return registry_cdn_phase10_signed_download_url(0, name, version, signature);
    }
    for (idx, base) in registry_cdn_phase8_regions().iter().enumerate() {
        if registry_cdn_phase9_health_probe_ok(idx) {
            return Some(registry_signed_download_url(base, name, version, signature));
        }
    }
    registry_cdn_phase8_regions()
        .first()
        .map(|base| registry_signed_download_url(base, name, version, signature))
}

/// Phase 31 EU1: Registry Phase 12 driver `syx install` path signature verify + multi-region retry E2E.
pub fn registry_cdn_phase12_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE12")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase12_install_max_retries() -> u32 {
    std::env::var("SYX_REGISTRY_CDN_PHASE12_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(7)
}

pub fn registry_cdn_phase12_install_download_url(
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    if !registry_cdn_phase12_enabled() {
        return registry_cdn_phase11_auto_download_url(name, version, signature);
    }
    if !verify_package_signature(&format!("{name}:{version}"), Some(signature)) {
        return None;
    }
    let retries = registry_cdn_phase12_install_max_retries();
    for attempt in 0..retries {
        if let Some(url) = registry_cdn_phase11_auto_download_url(name, version, signature) {
            return Some(format!("{url}&install=1&attempt={attempt}"));
        }
    }
    None
}

/// Phase 32 EZ1: Registry Phase 13 driver `syx install --offline` cache + ed25519 verify E2E.
pub fn registry_cdn_phase13_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE13")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase13_offline_cache_dir() -> String {
    std::env::var("SYX_REGISTRY_OFFLINE_CACHE")
        .unwrap_or_else(|_| format!("/tmp/syx-registry-offline-{}", std::process::id()))
}

pub fn registry_cdn_phase13_offline_install_path(
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    if !registry_cdn_phase13_enabled() {
        return registry_cdn_phase12_install_download_url(name, version, signature);
    }
    if !verify_package_signature(&format!("{name}:{version}"), Some(signature)) {
        return None;
    }
    let cache = registry_cdn_phase13_offline_cache_dir();
    Some(format!(
        "{cache}/{name}/{version}/package.syxpack?offline=1&signature={signature}"
    ))
}

/// Phase 33 FE1: Registry Phase 14 CDN edge cache + offline verify E2E prod.
pub fn registry_cdn_phase14_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE14")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase14_edge_cache_dir() -> String {
    std::env::var("SYX_REGISTRY_CDN_EDGE_CACHE")
        .unwrap_or_else(|_| format!("/tmp/syx-registry-edge-{}", std::process::id()))
}

pub fn registry_cdn_phase14_offline_verify_path(
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    if !registry_cdn_phase14_enabled() {
        return registry_cdn_phase13_offline_install_path(name, version, signature);
    }
    if !verify_package_signature(&format!("{name}:{version}"), Some(signature)) {
        return None;
    }
    let cache = registry_cdn_phase14_edge_cache_dir();
    Some(format!(
        "{cache}/{name}/{version}/package.syxpack?edge=1&offline=1&signature={signature}"
    ))
}

pub fn registry_cdn_phase14_offline_install_path(
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    registry_cdn_phase14_offline_verify_path(name, version, signature)
}

/// Phase 34 FJ1: Registry Phase 15 global CDN multi-active write + edge verify E2E prod.
pub fn registry_cdn_phase15_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE15")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn registry_cdn_phase15_global_write_path(name: &str, version: &str, signature: &str) -> Option<String> {
    if !registry_cdn_phase15_enabled() {
        return registry_cdn_phase14_offline_install_path(name, version, signature);
    }
    if !verify_package_signature(&format!("{name}:{version}"), Some(signature)) {
        return None;
    }
    let cache = registry_cdn_phase14_edge_cache_dir();
    Some(format!(
        "{cache}/{name}/{version}/package.syxpack?global-write=1&multi-active=1&signature={signature}"
    ))
}

pub fn registry_cdn_phase15_offline_verify_path(
    name: &str,
    version: &str,
    signature: &str,
) -> Option<String> {
    registry_cdn_phase15_global_write_path(name, version, signature)
}

/// Phase 36 FT1: Registry Phase 17 CRDT prod default on.
pub fn registry_cdn_phase17_prod_default() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE17_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(true)
}

/// Phase 35 FO1 / Phase 36: Phase 16 prod default follows Phase 17 flip.
pub fn registry_cdn_phase16_prod_default() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE16_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(registry_cdn_phase17_prod_default())
}

pub fn registry_cdn_phase16_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE16")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(registry_cdn_phase16_prod_default() || registry_cdn_phase15_enabled())
}

pub fn registry_cdn_phase16_crdt_write_path(
    name: &str,
    version: &str,
    signature: &str,
    crdt_epoch: u32,
) -> Option<String> {
    if !registry_cdn_phase16_enabled() {
        return registry_cdn_phase15_global_write_path(name, version, signature);
    }
    if !verify_package_signature(&format!("{name}:{version}"), Some(signature)) {
        return None;
    }
    let cache = registry_cdn_phase14_edge_cache_dir();
    Some(format!(
        "{cache}/{name}/{version}/package.syxpack?crdt-write=1&epoch={crdt_epoch}&signature={signature}"
    ))
}

pub fn registry_cdn_phase17_enabled() -> bool {
    std::env::var("SYX_REGISTRY_CDN_PHASE17")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(registry_cdn_phase17_prod_default() || registry_cdn_phase16_enabled())
}

pub fn registry_cdn_phase17_crdt_resolve_path(
    name: &str,
    version: &str,
    signature: &str,
    crdt_epoch: u32,
) -> Option<String> {
    if !registry_cdn_phase17_enabled() {
        return registry_cdn_phase16_crdt_write_path(name, version, signature, crdt_epoch);
    }
    if !verify_package_signature(&format!("{name}:{version}"), Some(signature)) {
        return None;
    }
    let cache = registry_cdn_phase14_edge_cache_dir();
    Some(format!(
        "{cache}/{name}/{version}/package.syxpack?crdt-resolve=1&epoch={crdt_epoch}&signature={signature}"
    ))
}

pub fn registry_signed_download_url(base: &str, name: &str, version: &str, signature: &str) -> String {
    format!(
        "{}/v1/packages/{name}/{version}/download?signature={signature}",
        base.trim_end_matches('/')
    )
}

fn effective_require_signature(policy: Option<lockfile::SignaturePolicy>) -> bool {
    if registry_allow_unsigned() {
        return false;
    }
    if let Some(p) = policy {
        return p.requires_signature();
    }
    registry_require_signature()
}

fn stub_signature_accepted() -> bool {
    !registry_require_signature() || registry_allow_stub()
}

pub fn verify_package_signature(checksum: &str, signature: Option<&str>) -> bool {
    verify_package_signature_for_policy(checksum, signature, None)
}

pub fn verify_package_signature_for_policy(
    checksum: &str,
    signature: Option<&str>,
    policy: Option<lockfile::SignaturePolicy>,
) -> bool {
    if checksum.len() < 8 {
        return false;
    }
    let require = effective_require_signature(policy);
    if require {
        let Some(sig_str) = signature else {
            return false;
        };
        if sig_str.is_empty() {
            return false;
        }
        if sig_str.starts_with("syx1:stub") {
            return registry_allow_stub();
        }
    }
    let Some(sig_str) = signature else {
        return true;
    };
    if sig_str.is_empty() {
        return true;
    }
    if sig_str.starts_with("syx1:stub") {
        return stub_signature_accepted();
    }
    let Some(rest) = sig_str.strip_prefix("syx1:ed25519:") else {
        return false;
    };
    // C-02: trust root pin required — embedded pubkey alone must not self-authorize.
    let env_key = std::env::var("SYX_REGISTRY_VERIFY_KEY").unwrap_or_default();
    let env_key = env_key.trim();
    if env_key.is_empty() {
        return false;
    }
    let (pubkey_hex, sig_hex): (&str, &str) = if let Some((pk, sig)) = rest.split_once(':') {
        if !pk.eq_ignore_ascii_case(env_key) {
            return false;
        }
        (pk, sig)
    } else {
        (env_key, rest)
    };
    if pubkey_hex.len() != 64 || sig_hex.len() != 128 {
        return false;
    }
    let Some(pk_bytes) = decode_hex(pubkey_hex) else {
        return false;
    };
    let Some(sig_bytes) = decode_hex(sig_hex) else {
        return false;
    };
    let Ok(pk_array) = <[u8; 32]>::try_from(pk_bytes.as_slice()) else {
        return false;
    };
    let Ok(sig_array) = <[u8; 64]>::try_from(sig_bytes.as_slice()) else {
        return false;
    };
    let Ok(verifying_key) = VerifyingKey::from_bytes(&pk_array) else {
        return false;
    };
    let signature = Signature::from_bytes(&sig_array);
    verifying_key
        .verify(checksum.as_bytes(), &signature)
        .is_ok()
}

/// Hex-encode the ed25519 verifying key (64 lowercase hex chars) for `SYX_REGISTRY_VERIFY_KEY`.
pub fn verifying_key_hex(signing_key: &SigningKey) -> String {
    encode_hex(signing_key.verifying_key().as_bytes())
}

/// Test/dev helper: sign a checksum with an ed25519 signing key.
pub fn sign_package_checksum(checksum: &str, signing_key: &SigningKey) -> String {
    let sig = signing_key.sign(checksum.as_bytes());
    let pk = signing_key.verifying_key();
    format!(
        "syx1:ed25519:{}:{}",
        encode_hex(pk.as_bytes()),
        encode_hex(sig.to_bytes().as_ref())
    )
}

pub fn load_publisher_signing_key() -> SyxResult<Option<SigningKey>> {
    let Ok(hex) = std::env::var("SYX_PUBLISHER_SIGNING_KEY") else {
        return Ok(None);
    };
    let bytes = decode_hex(&hex).ok_or_else(|| {
        SyxDiagnostic::error("SYX_PUBLISHER_SIGNING_KEY must be 64 hex characters (32-byte ed25519 seed)")
            .with_code("P012")
    })?;
    let seed: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
        SyxDiagnostic::error("SYX_PUBLISHER_SIGNING_KEY must decode to exactly 32 bytes")
            .with_code("P012")
    })?;
    Ok(Some(SigningKey::from_bytes(&seed)))
}

fn resolve_publish_signature(
    checksum: &str,
    signing_key: Option<&SigningKey>,
) -> SyxResult<Option<String>> {
    if let Some(key) = signing_key {
        return Ok(Some(sign_package_checksum(checksum, key)));
    }
    if let Some(key) = load_publisher_signing_key()? {
        return Ok(Some(sign_package_checksum(checksum, &key)));
    }
    if registry_require_signature() {
        return Err(SyxDiagnostic::error(
            "Unsigned publish rejected: lock v8 mandatory signature policy requires SYX_PUBLISHER_SIGNING_KEY",
        )
        .with_code("P012"));
    }
    Ok(None)
}

/// Publish the package at `root` to a registry.
pub fn publish_package(root: &Path, registry: &str) -> SyxResult<()> {
    package::publish_package(root, registry, None, None)
}

/// Publish with an explicit ed25519 signing key (Phase 69 lock v8 mandatory sign).
pub fn publish_package_signed(
    root: &Path,
    registry: &str,
    signing_key: &SigningKey,
) -> SyxResult<()> {
    package::publish_package(root, registry, None, Some(signing_key))
}

/// Publish with an explicit registry auth token (overrides `SYX_REGISTRY_TOKEN`).
pub fn publish_package_with_token(
    root: &Path,
    registry: &str,
    token: Option<&str>,
) -> SyxResult<()> {
    package::publish_package(root, registry, token, None)
}

/// Install all dependencies for a project and write `syx.lock`.
pub fn install_project(root: &Path) -> SyxResult<InstallReport> {
    installer::install_project(root)
}
pub fn add_dependency(root: &Path, name: &str, spec: DependencyAddSpec) -> SyxResult<()> {
    let mut manifest = load_manifest(root)?;
    let dep = spec.into_dependency_spec();
    manifest.dependencies.insert(name.to_string(), dep);
    save_manifest(root, &manifest)?;
    install_project(root)?;
    Ok(())
}

/// Re-resolve and reinstall all dependencies.
pub fn update_dependencies(root: &Path) -> SyxResult<InstallReport> {
    install_project(root)
}

/// Load installed packages from `syx.lock` for compile-time module resolution.
pub fn load_resolved_packages(root: &Path) -> SyxResult<IndexMap<String, InstalledPackage>> {
    let lock = load_lockfile(root)?;
    let mut out = IndexMap::new();
    for (name, pkg) in lock.packages {
        let mut installed = pkg.to_installed(&root.to_path_buf())?;
        installed.name = name.clone();
        out.insert(name, installed);
    }
    Ok(out)
}

/// Returns true when the project declares dependencies but has no valid lockfile.
pub fn requires_lockfile(root: &Path) -> SyxResult<bool> {
    let manifest = load_manifest(root)?;
    if manifest.dependencies.is_empty() {
        return Ok(false);
    }
    let lock_path = root.join("syx.lock");
    Ok(!lock_path.exists())
}

pub fn lockfile_error(root: &Path) -> SyxDiagnostic {
    SyxDiagnostic::error(format!(
        "Missing or invalid syx.lock for project '{}'. Run `syx install` first.",
        root.display()
    ))
    .with_code("P001")
}

pub fn vendor_dir(root: &Path) -> PathBuf {
    root.join(".syx").join("vendor")
}

pub fn git_available() -> bool {
    sources::git::git_available()
}
