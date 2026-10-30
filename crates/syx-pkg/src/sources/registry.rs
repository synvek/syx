use std::path::{Component, Path};

use flate2::read::GzDecoder;
use semver::Version;
use sha2::{Digest, Sha256};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use tar::Archive;

use crate::lockfile::{try_load_lockfile, LockPackage, PackageSource, SyxLockfile};
use crate::manifest::{check_version_req, load_manifest, registry_url_chain, DependencySpec, SyxManifest};
use crate::registry::{download_package, resolve_registry_version, RegistryClient};
use crate::{install_locked, verify_package_signature_for_policy};
use crate::sources::path::path_relative_to_project;
use crate::vendor_dir;

pub fn resolve_registry_dependency(
    project_root: &Path,
    dep_name: &str,
    spec: &DependencySpec,
    manifest: &SyxManifest,
) -> SyxResult<LockPackage> {
    let req = spec.version_req()?.ok_or_else(|| {
        SyxDiagnostic::error(format!("Registry dependency '{dep_name}' needs version constraint"))
    })?;
    let lock = try_load_lockfile(project_root).ok().flatten();
    if let Some(url) = spec.resolve_registry() {
        return resolve_from_registry_url(project_root, dep_name, &req, url, lock.as_ref());
    }
    let mut last_err = None;
    for registry_url in registry_url_chain_with_lock(manifest, lock.as_ref()) {
        match resolve_from_registry_url(project_root, dep_name, &req, &registry_url, lock.as_ref()) {
            Ok(pkg) => return Ok(pkg),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        SyxDiagnostic::error("No registry URL available for package resolution")
    }))
}

fn registry_url_chain_with_lock(manifest: &SyxManifest, lock: Option<&SyxLockfile>) -> Vec<String> {
    let mut urls = registry_url_chain(manifest);
    if let Some(lock) = lock {
        for mirror in &lock.registry_mirrors {
            if !urls.iter().any(|url| url == mirror) {
                urls.push(mirror.clone());
            }
        }
    }
    urls
}

fn effective_artifact_cdn_base(project_root: &Path, lock: Option<&SyxLockfile>) -> Option<String> {
    lock.and_then(|l| l.artifact_cdn_base.clone()).or_else(|| {
        std::env::var("SYX_REGISTRY_CDN")
            .ok()
            .filter(|value| !value.is_empty())
    }).or_else(|| {
        // Prefer lock on disk when env is unset during reinstall.
        try_load_lockfile(project_root)
            .ok()
            .flatten()
            .and_then(|l| l.artifact_cdn_base)
    })
}

fn resolve_from_registry_url(
    project_root: &Path,
    dep_name: &str,
    req: &semver::VersionReq,
    registry_url: &str,
    lock: Option<&SyxLockfile>,
) -> SyxResult<LockPackage> {
    let client = RegistryClient::new(registry_url)?;
    let (version, checksum) = if install_locked() {
        let lock = lock.ok_or_else(|| {
            SyxDiagnostic::error("SYX_INSTALL_LOCKED requires an existing syx.lock")
                .with_code("P014")
        })?;
        let locked = lock.packages.get(dep_name).ok_or_else(|| {
            SyxDiagnostic::error(format!(
                "SYX_INSTALL_LOCKED: package '{dep_name}' missing from syx.lock"
            ))
            .with_code("P014")
        })?;
        match &locked.source {
            PackageSource::Registry {
                version, checksum, ..
            } => {
                let parsed = Version::parse(version).map_err(|e| {
                    SyxDiagnostic::error(format!(
                        "Invalid locked registry version '{version}' for '{dep_name}': {e}"
                    ))
                    .with_code("P014")
                })?;
                if !req.matches(&parsed) {
                    return Err(SyxDiagnostic::error(format!(
                        "Locked version {version} for '{dep_name}' does not satisfy {req}"
                    ))
                    .with_code("P014"));
                }
                (version.clone(), checksum.clone())
            }
            _ => {
                return Err(SyxDiagnostic::error(format!(
                    "SYX_INSTALL_LOCKED: '{dep_name}' is not a registry pin in syx.lock"
                ))
                .with_code("P014"));
            }
        }
    } else {
        let version = resolve_registry_version(&client, dep_name, &req)?;
        let checksum = client.package_checksum(dep_name, &version)?;
        (version, checksum)
    };
    let signature = client.package_signature(dep_name, &version)?;
    let bytes = download_registry_tarball(
        &client,
        project_root,
        lock,
        dep_name,
        &version,
    )?;

    verify_checksum(&bytes, &checksum)?;
    let policy = lock
        .map(|l| l.signature_policy)
        .or_else(|| {
            try_load_lockfile(project_root)
                .ok()
                .flatten()
                .map(|l| l.signature_policy)
        });
    if !verify_package_signature_for_policy(&checksum, signature.as_deref(), policy) {
        return Err(SyxDiagnostic::error(format!(
            "Package signature verification failed for {dep_name}@{version}"
        ))
        .with_code("P011"));
    }

    let vendor = vendor_dir(project_root);
    let dest = vendor.join(format!("{dep_name}-{version}"));
    // Wave C W675 / C-03: refuse symlink dest + require wipe success (no `.ok()` swallow).
    prepare_vendor_extract_dest(&dest)?;
    extract_tar_gz(&bytes, &dest)?;

    let version_parsed = Version::parse(&version).map_err(|e| {
        SyxDiagnostic::error(format!("Invalid registry version '{version}': {e}"))
    })?;
    check_version_req(&dest, &req, dep_name)?;
    let rel_dir = path_relative_to_project(project_root, &dest);

    Ok(LockPackage {
        source: PackageSource::Registry {
            registry: registry_url.to_string(),
            version: version_parsed.to_string(),
            checksum,
            dir: rel_dir,
        },
    })
}

fn download_registry_tarball(
    client: &RegistryClient,
    project_root: &Path,
    lock: Option<&SyxLockfile>,
    name: &str,
    version: &str,
) -> SyxResult<Vec<u8>> {
    if let Some(cdn_base) = effective_artifact_cdn_base(project_root, lock) {
        if let Ok(cdn_client) = RegistryClient::new(&cdn_base) {
            if let Ok(bytes) = download_package(&cdn_client, name, version) {
                return Ok(bytes);
            }
        }
    }
    download_package(client, name, version)
}

fn verify_checksum(bytes: &[u8], expected: &str) -> SyxResult<()> {
    let expected = expected.strip_prefix("sha256:").unwrap_or(expected);
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let actual = format!("{:x}", hasher.finalize());
    if actual != expected {
        return Err(SyxDiagnostic::error(format!(
            "Checksum mismatch: expected sha256:{expected}, got sha256:{actual}"
        ))
        .with_code("P003"));
    }
    Ok(())
}

fn extract_security_err(message: impl Into<String>) -> SyxDiagnostic {
    SyxDiagnostic::error(message).with_code("P013")
}

fn dest_is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

/// Prepare a vendor extract slot: refuse symlink dest; wipe existing dir must succeed.
///
/// Wave C W675 / C-03 close-out: vendor-slot cache poison must fail (`P013`).
pub fn prepare_vendor_extract_dest(dest: &Path) -> SyxResult<()> {
    if dest_is_symlink(dest) {
        return Err(extract_security_err(format!(
            "Refusing vendor extract dest that is a symlink (cache poison): {}",
            dest.display()
        )));
    }
    match std::fs::symlink_metadata(dest) {
        Ok(_) => std::fs::remove_dir_all(dest).map_err(|e| {
            extract_security_err(format!(
                "Failed to wipe vendor slot {}: {e}",
                dest.display()
            ))
        })?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(extract_security_err(format!(
                "Failed to inspect vendor slot {}: {e}",
                dest.display()
            )));
        }
    }
    Ok(())
}

/// True when `path` is absolute or contains `..` (vendor-escape / traversal).
fn archive_path_is_unsafe(path: &Path) -> bool {
    path.components().any(|c| {
        matches!(
            c,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    })
}

/// True when resolving `link` relative to `entry_path`'s parent would leave the extract root.
fn link_target_escapes(entry_path: &Path, link: &Path) -> bool {
    if link.components().any(|c| matches!(c, Component::RootDir | Component::Prefix(_))) {
        return true;
    }
    let parent = entry_path.parent().unwrap_or_else(|| Path::new(""));
    let mut depth: i32 = 0;
    for c in parent.components().chain(link.components()) {
        match c {
            Component::CurDir => {}
            Component::Normal(_) => depth += 1,
            Component::ParentDir => {
                depth -= 1;
                if depth < 0 {
                    return true;
                }
            }
            Component::RootDir | Component::Prefix(_) => return true,
        }
    }
    false
}

/// Extract a package tarball under `dest`.
///
/// Wave C W674–W675 / C-03: paths with `..` / absolute prefixes, symlink/hardlink
/// targets that escape `dest`, and **symlink extract dest** (vendor-slot poison)
/// return **`Err` with `P013`** (must fail — not silently skip).
pub fn extract_tar_gz(bytes: &[u8], dest: &Path) -> SyxResult<()> {
    if dest_is_symlink(dest) {
        return Err(extract_security_err(format!(
            "Refusing vendor extract dest that is a symlink (cache poison): {}",
            dest.display()
        )));
    }
    std::fs::create_dir_all(dest).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to create {}: {e}", dest.display()))
    })?;
    let decoder = GzDecoder::new(bytes);
    let mut archive = Archive::new(decoder);
    let entries = archive.entries().map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read package archive: {e}"))
    })?;
    for entry in entries {
        let mut entry = entry.map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read archive entry: {e}"))
        })?;
        let path = entry.path().map_err(|e| {
            SyxDiagnostic::error(format!("Invalid archive entry path: {e}"))
        })?;
        if archive_path_is_unsafe(path.as_ref()) {
            return Err(extract_security_err(format!(
                "Refusing archive path that escapes extract root: {}",
                path.display()
            )));
        }
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            let link = entry
                .link_name()
                .map_err(|e| SyxDiagnostic::error(format!("Invalid archive link name: {e}")))?
                .ok_or_else(|| extract_security_err("Archive link entry missing link name"))?;
            if link_target_escapes(path.as_ref(), link.as_ref()) {
                return Err(extract_security_err(format!(
                    "Refusing archive link that escapes extract root: {} -> {}",
                    path.display(),
                    link.display()
                )));
            }
        }
        entry.unpack_in(dest).map_err(|e| {
            SyxDiagnostic::error(format!(
                "Failed to extract package to {}: {e}",
                dest.display()
            ))
        })?;
    }
    Ok(())
}

pub fn create_tar_gz(source_dir: &Path) -> SyxResult<Vec<u8>> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    {
        let mut tar_builder = tar::Builder::new(&mut encoder);
        tar_builder.append_dir_all(".", source_dir).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to create tarball: {e}"))
        })?;
        tar_builder.finish().map_err(|e| {
            SyxDiagnostic::error(format!("Failed to finish tarball: {e}"))
        })?;
    }
    encoder.finish().map_err(|e| SyxDiagnostic::error(format!("Failed to compress tarball: {e}")))
}

pub fn tarball_checksum(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}

pub fn validate_package_layout(root: &Path) -> SyxResult<SyxManifest> {
    let manifest_path = root.join("syx.toml");
    if !manifest_path.exists() {
        return Err(SyxDiagnostic::error(format!(
            "Package missing syx.toml at {}",
            root.display()
        )));
    }
    load_manifest(root)
}
