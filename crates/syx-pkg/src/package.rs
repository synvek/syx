use std::path::Path;

use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::registry::RegistryClient;
use crate::sources::registry::{create_tar_gz, tarball_checksum, validate_package_layout};

pub fn publish_package(
    root: &Path,
    registry: &str,
    token: Option<&str>,
    signing_key: Option<&ed25519_dalek::SigningKey>,
) -> SyxResult<()> {
    let manifest = validate_package_layout(root)?;
    if manifest.name.is_empty() || manifest.version.is_empty() {
        return Err(SyxDiagnostic::error(
            "Package syx.toml must have name and version for publish",
        ));
    }
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
