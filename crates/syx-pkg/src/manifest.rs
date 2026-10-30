use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RegistryConfig {
    #[serde(default)]
    pub default: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DependencySpec {
    Simple(String),
    Detailed {
        version: String,
        #[serde(default)]
        path: Option<String>,
        #[serde(default)]
        git: Option<String>,
        #[serde(default)]
        rev: Option<String>,
        #[serde(default)]
        registry: Option<String>,
    },
}

impl DependencySpec {
    pub fn resolve_path(&self) -> Option<&str> {
        match self {
            DependencySpec::Simple(path) => Some(path.as_str()),
            DependencySpec::Detailed { path, .. } => path.as_deref(),
        }
    }

    pub fn resolve_git(&self) -> Option<&str> {
        match self {
            DependencySpec::Simple(_) => None,
            DependencySpec::Detailed { git, .. } => git.as_deref(),
        }
    }

    pub fn resolve_rev(&self) -> Option<&str> {
        match self {
            DependencySpec::Simple(_) => None,
            DependencySpec::Detailed { rev, .. } => rev.as_deref(),
        }
    }

    pub fn resolve_registry(&self) -> Option<&str> {
        match self {
            DependencySpec::Simple(_) => None,
            DependencySpec::Detailed { registry, .. } => registry.as_deref(),
        }
    }

    pub fn version_req(&self) -> SyxResult<Option<VersionReq>> {
        match self {
            DependencySpec::Simple(_) => Ok(None),
            DependencySpec::Detailed { version, .. } => {
                let req = VersionReq::parse(version).map_err(|e| {
                    SyxDiagnostic::error(format!("Invalid semver requirement '{version}': {e}"))
                })?;
                Ok(Some(req))
            }
        }
    }

    pub fn source_kind(&self) -> DependencySourceKind {
        if self.resolve_path().is_some() {
            DependencySourceKind::Path
        } else if self.resolve_git().is_some() {
            DependencySourceKind::Git
        } else {
            DependencySourceKind::Registry
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencySourceKind {
    Path,
    Git,
    Registry,
}

#[derive(Debug, Clone)]
pub struct DependencyAddSpec {
    pub version: String,
    pub path: Option<String>,
    pub git: Option<String>,
    pub rev: Option<String>,
    pub registry: Option<String>,
}

impl DependencyAddSpec {
    pub fn path(path: impl Into<String>) -> Self {
        Self {
            version: "1.0".into(),
            path: Some(path.into()),
            git: None,
            rev: None,
            registry: None,
        }
    }

    pub fn git(url: impl Into<String>, version: impl Into<String>, rev: Option<String>) -> Self {
        Self {
            version: version.into(),
            path: None,
            git: Some(url.into()),
            rev,
            registry: None,
        }
    }

    pub fn registry(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            path: None,
            git: None,
            rev: None,
            registry: None,
        }
    }

    pub fn into_dependency_spec(self) -> DependencySpec {
        DependencySpec::Detailed {
            version: self.version,
            path: self.path,
            git: self.git,
            rev: self.rev,
            registry: self.registry,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConcurrencyConfig {
    #[serde(default)]
    pub strict: bool,
    #[serde(default)]
    pub default_isolation: Option<String>,
}

/// M3-E02 / ADR-311: opt-in permission grants from `syx.toml` `[permissions]`.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PermissionsConfig {
    #[serde(default)]
    pub sandbox: bool,
    #[serde(default)]
    pub allow_net: bool,
    #[serde(default)]
    pub allow_fs: bool,
    #[serde(default)]
    pub allow_env: bool,
    #[serde(default)]
    pub allow_process: bool,
    #[serde(default)]
    pub allow_plugin: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NativeConfig {
    #[serde(default)]
    pub libs: Vec<String>,
    #[serde(default)]
    pub lib_paths: Vec<String>,
    #[serde(default)]
    pub link_flags: Vec<String>,
    #[serde(default)]
    pub objects: Vec<String>,
    #[serde(default)]
    pub include_dirs: Vec<String>,
}

/// Phase 120: accept flat `macros = ["a"]` or table `[macros] exports = ["a"]`.
#[derive(Debug, Clone, Default)]
pub struct MacrosField(pub Vec<String>);

impl MacrosField {
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MacrosField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            List(Vec<String>),
            Table { #[serde(default)] exports: Vec<String> },
        }
        Ok(match Raw::deserialize(deserializer)? {
            Raw::List(v) => MacrosField(v),
            Raw::Table { exports } => MacrosField(exports),
        })
    }
}

impl Serialize for MacrosField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyxManifest {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub main: String,
    #[serde(default)]
    pub build: Option<String>,
    #[serde(default)]
    pub registry: RegistryConfig,
    #[serde(default)]
    pub dependencies: IndexMap<String, DependencySpec>,
    #[serde(default)]
    pub concurrency: ConcurrencyConfig,
    /// M3-E02 / ADR-311: permission sandbox grants (`[permissions]`).
    #[serde(default)]
    pub permissions: PermissionsConfig,
    /// Phase 52/120: exported macro names (`macros = [...]` or `[macros].exports`).
    #[serde(default)]
    pub macros: MacrosField,
    #[serde(default)]
    pub native: NativeConfig,
}

impl Default for SyxManifest {
    fn default() -> Self {
        Self {
            name: "app".into(),
            version: "0.1.0".into(),
            main: "main.sy".into(),
            registry: RegistryConfig::default(),
            dependencies: IndexMap::new(),
            concurrency: ConcurrencyConfig::default(),
            permissions: PermissionsConfig::default(),
            macros: MacrosField::default(),
            build: None,
            native: NativeConfig::default(),
        }
    }
}

pub fn load_manifest(root: &Path) -> SyxResult<SyxManifest> {
    let path = root.join("syx.toml");
    if path.exists() {
        let text = std::fs::read_to_string(&path).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read syx.toml: {e}"))
        })?;
        toml::from_str(&text).map_err(|e| SyxDiagnostic::error(format!("Invalid syx.toml: {e}")))
    } else {
        Ok(SyxManifest::default())
    }
}

pub fn save_manifest(root: &Path, manifest: &SyxManifest) -> SyxResult<()> {
    let text = toml::to_string_pretty(manifest).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to serialize syx.toml: {e}"))
    })?;
    std::fs::write(root.join("syx.toml"), text).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to write syx.toml: {e}"))
    })
}

pub fn default_registry_url(manifest: &SyxManifest) -> String {
    registry_url_chain(manifest)
        .into_iter()
        .next()
        .unwrap_or_else(|| "https://packages.sy.dev".into())
}

/// Phase 12 BD2: primary → CDN staging → mirror failover chain.
pub fn registry_url_chain(manifest: &SyxManifest) -> Vec<String> {
    let mut urls = Vec::new();
    if let Some(url) = &manifest.registry.default {
        urls.push(url.clone());
    }
    let primary = std::env::var("SYX_REGISTRY")
        .or_else(|_| std::env::var("SYX_REGISTRY_MIRROR"))
        .unwrap_or_else(|_| "https://packages.sy.dev".into());
    if !urls.iter().any(|u| u == &primary) {
        urls.push(primary);
    }
    if let Ok(staging) = std::env::var("SYX_REGISTRY_STAGING") {
        if !staging.is_empty() && !urls.iter().any(|u| u == &staging) {
            urls.push(staging);
        }
    }
    if let Ok(cdn) = std::env::var("SYX_REGISTRY_CDN") {
        if !cdn.is_empty() && !urls.iter().any(|u| u == &cdn) {
            urls.push(cdn);
        }
    }
    if let Ok(mirror) = std::env::var("SYX_REGISTRY_MIRROR") {
        if !mirror.is_empty() && !urls.iter().any(|u| u == &mirror) {
            urls.push(mirror);
        }
    }
    // Phase 18 CH3: staging→prod promotion moves staging to head of failover chain.
    if std::env::var("SYX_REGISTRY_PROMOTE_STAGING")
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        if let Ok(staging) = std::env::var("SYX_REGISTRY_STAGING") {
            if !staging.is_empty() {
                urls.retain(|u| u != &staging);
                urls.insert(0, staging);
            }
        }
    }
    urls
}

pub fn read_package_version(root: &Path) -> SyxResult<Version> {
    let manifest = load_manifest(root)?;
    Version::parse(&manifest.version).map_err(|e| {
        SyxDiagnostic::error(format!(
            "Invalid version '{}' in {}: {e}",
            manifest.version,
            root.join("syx.toml").display()
        ))
    })
}

pub fn check_version_req(root: &Path, req: &VersionReq, dep_name: &str) -> SyxResult<Version> {
    let version = read_package_version(root)?;
    if !req.matches(&version) {
        return Err(SyxDiagnostic::error(format!(
            "Package '{dep_name}' version {version} does not satisfy requirement {req}"
        ))
        .with_code("R001"));
    }
    Ok(version)
}

#[allow(dead_code)] // path resolution helper retained for pkg deepen
pub fn package_root_candidates(dep_root: &Path, module_name: &str, main: &str) -> Vec<PathBuf> {
    let first_segment = module_name.split('.').next().unwrap_or(module_name);
    let dotted_file = module_name.replace('.', "/");
    vec![
        dep_root.join(format!("{module_name}.sy")),
        dep_root.join(format!("{dotted_file}.sy")),
        dep_root.join(format!("{first_segment}.sy")),
        dep_root.join("src").join(format!("{first_segment}.sy")),
        dep_root.join(main),
    ]
}
