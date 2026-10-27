use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "lowercase")]
pub enum PackageSource {
    Path {
        version: String,
        path: String,
    },
    Git {
        git: String,
        rev: String,
        dir: String,
    },
    Registry {
        registry: String,
        version: String,
        checksum: String,
        dir: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockPackage {
    #[serde(flatten)]
    pub source: PackageSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockPluginEntry {
    pub id: String,
    pub hosts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_base: Option<u32>,
}

fn default_lock_version() -> u32 {
    16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SignaturePolicy {
    Strict,
    #[default]
    #[serde(alias = "required")]
    Mandatory,
    #[serde(alias = "optional")]
    Optional,
}

impl SignaturePolicy {
    pub fn requires_signature(self) -> bool {
        matches!(self, Self::Strict | Self::Mandatory)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyxLockfile {
    #[serde(default = "default_lock_version")]
    pub version: u32,
    #[serde(default)]
    pub syx_version: String,
    /// Minimum SYX toolchain version required to consume this lockfile.
    #[serde(default)]
    pub syx_version_floor: String,
    /// UTC epoch seconds when dependencies were last resolved.
    #[serde(default)]
    pub resolved_at: String,
    #[serde(default)]
    pub registry: String,
    /// Failover registry URLs recorded at resolve time (lock v9).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub registry_mirrors: Vec<String>,
    /// CDN base URL for tarball downloads (lock v9+).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_cdn_base: Option<String>,
    /// Multi-region CDN URLs (lock v10).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_cdn_regions: Vec<String>,
    /// Region selected at resolve time (lock v10).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resolved_region: String,
    /// Distributed cache mirror nodes (lock v10).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cache_remote_nodes: Vec<String>,
    /// Plugin hot-unload epoch (lock v11).
    #[serde(default)]
    pub plugin_unload_epoch: u64,
    /// Marketplace CDN base for plugin bundles (lock v11).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketplace_cdn_base: Option<String>,
    /// Build farm queue URL (lock v11).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub farm_queue_url: Option<String>,
    /// Publisher key rotation counter (lock v12).
    #[serde(default)]
    pub publisher_key_rotation: u32,
    /// Active farm worker id (lock v12).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub farm_worker_id: Option<String>,
    /// Plugin bundle sha256 by id (lock v12).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub plugin_bundle_sha256: IndexMap<String, String>,
    /// Multi-tenant farm scope (lock v13).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Publisher key rotation cron spec e.g. `@daily` (lock v13).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_cron_spec: Option<String>,
    /// Farm worker lease TTL seconds (lock v13).
    #[serde(default)]
    pub farm_lease_ttl_secs: u32,
    /// Farm worker pool ids (lock v13).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub farm_worker_pool: Vec<String>,
    /// HSM key reference for publisher signing (lock v14).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hsm_key_ref: Option<String>,
    /// Per-tenant compile-minute billing meter id (lock v14).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_meter_id: Option<String>,
    /// HSM rotation cron job id (lock v15).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hsm_rotation_cron_job: Option<String>,
    /// Per-tenant billing settlement id (lock v15).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_settlement_id: Option<String>,
    /// Marketplace plugin unload epoch (lock v15).
    #[serde(default)]
    pub marketplace_plugin_epoch: u64,
    /// VS Code marketplace publish epoch (lock v16).
    #[serde(default)]
    pub marketplace_publish_epoch: u64,
    /// Extension signing key id for marketplace bundles (lock v16).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_signing_key_id: Option<String>,
    /// Publisher signing key id recorded at resolve time (lock v5).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub publisher_key_id: String,
    /// Signature verification policy for registry installs (lock v5).
    #[serde(default)]
    pub signature_policy: SignaturePolicy,
    /// Registry package name → tarball sha256 checksum (`sha256:…`).
    #[serde(default)]
    pub tarball_sha256: IndexMap<String, String>,
    #[serde(default)]
    pub packages: IndexMap<String, LockPackage>,
    #[serde(default)]
    pub plugins: Vec<LockPluginEntry>,
}

impl Default for SyxLockfile {
    fn default() -> Self {
        Self {
            version: default_lock_version(),
            syx_version: String::new(),
            syx_version_floor: String::new(),
            resolved_at: String::new(),
            registry: String::new(),
            registry_mirrors: Vec::new(),
            artifact_cdn_base: None,
            artifact_cdn_regions: Vec::new(),
            resolved_region: String::new(),
            cache_remote_nodes: Vec::new(),
            plugin_unload_epoch: 0,
            marketplace_cdn_base: None,
            farm_queue_url: None,
            publisher_key_rotation: 0,
            farm_worker_id: None,
            plugin_bundle_sha256: IndexMap::new(),
            tenant_id: None,
            rotation_cron_spec: None,
            farm_lease_ttl_secs: 300,
            farm_worker_pool: Vec::new(),
            hsm_key_ref: None,
            billing_meter_id: None,
            hsm_rotation_cron_job: None,
            billing_settlement_id: None,
            marketplace_plugin_epoch: 0,
            marketplace_publish_epoch: 0,
            extension_signing_key_id: None,
            publisher_key_id: String::new(),
            signature_policy: SignaturePolicy::default(),
            tarball_sha256: IndexMap::new(),
            packages: IndexMap::new(),
            plugins: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub root: PathBuf,
}

impl LockPackage {
    pub fn to_installed(&self, project_root: &Path) -> SyxResult<InstalledPackage> {
        match &self.source {
            PackageSource::Path { version, path } => Ok(InstalledPackage {
                name: String::new(),
                version: version.clone(),
                root: project_root.join(path),
            }),
            PackageSource::Git { dir, .. } => Ok(InstalledPackage {
                name: String::new(),
                version: String::new(),
                root: project_root.join(dir),
            }),
            PackageSource::Registry { dir, version, .. } => Ok(InstalledPackage {
                name: String::new(),
                version: version.clone(),
                root: project_root.join(dir),
            }),
        }
    }

    pub fn root_relative(&self) -> &str {
        match &self.source {
            PackageSource::Path { path, .. } => path,
            PackageSource::Git { dir, .. } | PackageSource::Registry { dir, .. } => dir,
        }
    }

    pub fn resolved_version(&self) -> String {
        match &self.source {
            PackageSource::Path { version, .. } | PackageSource::Registry { version, .. } => {
                version.clone()
            }
            PackageSource::Git { .. } => String::new(),
        }
    }
}

impl InstalledPackage {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

pub fn load_lockfile(root: &Path) -> SyxResult<SyxLockfile> {
    let path = root.join("syx.lock");
    if !path.exists() {
        return Err(SyxDiagnostic::error(format!(
            "Missing syx.lock at {}",
            path.display()
        ))
        .with_code("P001"));
    }
    let text = std::fs::read_to_string(&path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read syx.lock: {e}"))
    })?;
    toml::from_str(&text).map_err(|e| {
        SyxDiagnostic::error(format!("Invalid syx.lock: {e}")).with_code("P001")
    })
}

pub fn save_lockfile(root: &Path, lock: &SyxLockfile) -> SyxResult<()> {
    let text = toml::to_string_pretty(lock).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to serialize syx.lock: {e}"))
    })?;
    std::fs::write(root.join("syx.lock"), text).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to write syx.lock: {e}"))
    })
}

pub fn try_load_lockfile(root: &Path) -> SyxResult<Option<SyxLockfile>> {
    let path = root.join("syx.lock");
    if !path.exists() {
        return Ok(None);
    }
    load_lockfile(root).map(Some)
}
