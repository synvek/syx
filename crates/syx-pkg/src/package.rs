use std::path::Path;

use syx_diagnostics::{Span, SyxResult};

use crate::lock_enforce::enforce_lock_v16;
use crate::lockfile::{try_load_lockfile, SyxLockfile};
use crate::pkg_error_at;
use crate::registry::RegistryClient;
use crate::sources::registry::{create_tar_gz, tarball_checksum, validate_package_layout};

/// C-4c: publish requires an existing, enforceable `syx.lock`.
pub fn require_publish_lock(root: &Path) -> SyxResult<SyxLockfile> {
    let Some(lock) = try_load_lockfile(root)? else {
        return Err(pkg_error_at(
            "publish requires an existing syx.lock (C-4c); run syx install",
            None,
            Span::DUMMY,
            "P014",
        ));
    };
    enforce_lock_v16(&lock)?;
    Ok(lock)
}

pub fn publish_package(
    root: &Path,
    registry: &str,
    token: Option<&str>,
    signing_key: Option<&ed25519_dalek::SigningKey>,
) -> SyxResult<()> {
    let manifest = validate_package_layout(root)?;
    if manifest.name.is_empty() || manifest.version.is_empty() {
        return Err(pkg_error_at(
            "Package syx.toml must have name and version for publish",
            None,
            Span::DUMMY,
            "P042",
        ));
    }
    let _lock = require_publish_lock(root)?;
    let registry = registry.to_string();
    let tarball = create_tar_gz(root)?;
    let checksum = tarball_checksum(&tarball);
    let signature = crate::resolve_publish_signature(&checksum, signing_key)?;
    let auth = token
        .map(str::to_string)
        .or_else(|| std::env::var("SYX_REGISTRY_TOKEN").ok());
    let client = RegistryClient::new_with_token(&registry, auth)?;
    client.publish(
        &manifest.name,
        &manifest.version,
        &tarball,
        &checksum,
        signature.as_deref(),
    )?;
    Ok(())
}
