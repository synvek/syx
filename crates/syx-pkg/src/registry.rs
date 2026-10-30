use std::path::PathBuf;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageIndex {
    pub name: String,
    pub versions: Vec<PackageVersionEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageVersionEntry {
    pub version: String,
    pub checksum: String,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub yanked: bool,
}

pub struct RegistryClient {
    base_url: String,
    file_root: Option<PathBuf>,
    http: Option<reqwest::blocking::Client>,
    auth_token: Option<String>,
}

impl RegistryClient {
    pub fn new(registry_url: &str) -> SyxResult<Self> {
        Self::new_with_token(registry_url, std::env::var("SYX_REGISTRY_TOKEN").ok())
    }

    pub fn new_with_token(registry_url: &str, auth_token: Option<String>) -> SyxResult<Self> {
        if let Some(path) = registry_url.strip_prefix("file://") {
            let root = PathBuf::from(path);
            if !root.exists() {
                std::fs::create_dir_all(&root).map_err(|e| {
                    SyxDiagnostic::error(format!("Failed to create file registry dir: {e}"))
                })?;
            }
            if !root.is_dir() {
                return Err(SyxDiagnostic::error(format!(
                    "File registry not found at {}",
                    root.display()
                )));
            }
            Ok(Self {
                base_url: registry_url.to_string(),
                file_root: Some(root),
                http: None,
                auth_token,
            })
        } else {
            let client = reqwest::blocking::Client::builder()
                .build()
                .map_err(|e| SyxDiagnostic::error(format!("Failed to create HTTP client: {e}")))?;
            Ok(Self {
                base_url: registry_url.trim_end_matches('/').to_string(),
                file_root: None,
                http: Some(client),
                auth_token,
            })
        }
    }

    pub fn fetch_index(&self, name: &str) -> SyxResult<PackageIndex> {
        if let Some(root) = &self.file_root {
            let index_path = root
                .join("v1")
                .join("packages")
                .join(name)
                .join("index.json");
            let text = std::fs::read_to_string(&index_path).map_err(|e| {
                SyxDiagnostic::error(format!(
                    "Failed to read registry index {}: {e}",
                    index_path.display()
                ))
            })?;
            serde_json::from_str(&text).map_err(|e| {
                SyxDiagnostic::error(format!("Invalid registry index JSON: {e}"))
            })
        } else {
            let url = format!("{}/v1/packages/{name}", self.base_url);
            let client = self.http.as_ref().unwrap();
            let text = client.get(&url).send().map_err(|e| {
                SyxDiagnostic::error(format!("Registry request failed for {url}: {e}"))
                    .with_code("P003")
            })?;
            if !text.status().is_success() {
                return Err(SyxDiagnostic::error(format!(
                    "Registry returned {} for {url}",
                    text.status()
                ))
                .with_code("P003"));
            }
            let body = text.text().map_err(|e| {
                SyxDiagnostic::error(format!("Failed to read registry response: {e}"))
            })?;
            serde_json::from_str(&body).map_err(|e| {
                SyxDiagnostic::error(format!("Invalid registry index JSON: {e}"))
            })
        }
    }

    pub fn package_checksum(&self, name: &str, version: &str) -> SyxResult<String> {
        let index = self.fetch_index(name)?;
        index
            .versions
            .iter()
            .find(|v| v.version == version)
            .map(|v| v.checksum.clone())
            .ok_or_else(|| {
                SyxDiagnostic::error(format!("Version {version} not found for package {name}"))
                    .with_code("P003")
            })
    }

    pub fn package_signature(&self, name: &str, version: &str) -> SyxResult<Option<String>> {
        let index = self.fetch_index(name)?;
        Ok(index
            .versions
            .iter()
            .find(|v| v.version == version)
            .and_then(|v| v.signature.clone()))
    }

    pub fn download(&self, name: &str, version: &str) -> SyxResult<Vec<u8>> {
        if let Some(root) = &self.file_root {
            let download_path = root
                .join("v1")
                .join("packages")
                .join(name)
                .join(version)
                .join("download");
            std::fs::read(&download_path).map_err(|e| {
                SyxDiagnostic::error(format!(
                    "Failed to read package tarball {}: {e}",
                    download_path.display()
                ))
                .with_code("P003")
            })
        } else {
            let url = format!("{}/v1/packages/{name}/{version}/download", self.base_url);
            let client = self.http.as_ref().unwrap();
            let resp = client.get(&url).send().map_err(|e| {
                SyxDiagnostic::error(format!("Download failed for {url}: {e}")).with_code("P003")
            })?;
            if !resp.status().is_success() {
                return Err(SyxDiagnostic::error(format!(
                    "Download returned {} for {url}",
                    resp.status()
                ))
                .with_code("P003"));
            }
            resp.bytes()
                .map(|b| b.to_vec())
                .map_err(|e| SyxDiagnostic::error(format!("Failed to read download body: {e}")))
        }
    }

    pub fn publish(
        &self,
        name: &str,
        version: &str,
        tarball: &[u8],
        checksum: &str,
        signature: Option<&str>,
    ) -> SyxResult<()> {
        if crate::registry_require_signature() && signature.is_none() {
            return Err(SyxDiagnostic::error(
                "Unsigned publish rejected: registry lock v9 mandatory signature policy",
            )
            .with_code("P012"));
        }
        if let Some(root) = &self.file_root {
            let pkg_dir = root.join("v1").join("packages").join(name);
            std::fs::create_dir_all(pkg_dir.join(version)).map_err(|e| {
                SyxDiagnostic::error(format!("Failed to create publish dir: {e}"))
            })?;
            std::fs::write(pkg_dir.join(version).join("download"), tarball).map_err(|e| {
                SyxDiagnostic::error(format!("Failed to write tarball: {e}"))
            })?;
            let index_path = pkg_dir.join("index.json");
            let mut index = if index_path.exists() {
                let text = std::fs::read_to_string(&index_path).unwrap_or_default();
                serde_json::from_str::<PackageIndex>(&text).unwrap_or(PackageIndex {
                    name: name.to_string(),
                    versions: vec![],
                })
            } else {
                PackageIndex {
                    name: name.to_string(),
                    versions: vec![],
                }
            };
            if !index.versions.iter().any(|v| v.version == version) {
                index.versions.push(PackageVersionEntry {
                    version: version.to_string(),
                    checksum: checksum.to_string(),
                    signature: signature.map(str::to_string),
                    yanked: false,
                });
            } else if let Some(entry) = index.versions.iter_mut().find(|v| v.version == version) {
                entry.checksum = checksum.to_string();
                if signature.is_some() {
                    entry.signature = signature.map(str::to_string);
                }
            }
            let json = serde_json::to_string_pretty(&index).map_err(|e| {
                SyxDiagnostic::error(format!("Failed to serialize index: {e}"))
            })?;
            std::fs::write(&index_path, json).map_err(|e| {
                SyxDiagnostic::error(format!("Failed to write index: {e}"))
            })?;
            Ok(())
        } else {
            if self.auth_token.is_none() {
                return Err(SyxDiagnostic::error(
                    "SYX_REGISTRY_TOKEN or --token required for HTTP registry publish",
                )
                .with_code("P010"));
            }
            let url = format!("{}/v1/packages/{name}/{version}", self.base_url);
            let client = self.http.as_ref().unwrap();
            let resp = client
                .put(&url)
                .header(
                    "Authorization",
                    format!("Bearer {}", self.auth_token.as_deref().unwrap_or("")),
                )
                .body(tarball.to_vec())
                .send()
                .map_err(|e| SyxDiagnostic::error(format!("Publish failed for {url}: {e}")))?;
            if !resp.status().is_success() {
                return Err(SyxDiagnostic::error(format!(
                    "Publish returned {} for {url}",
                    resp.status()
                )));
            }
            Ok(())
        }
    }
}

pub fn resolve_registry_version(
    client: &RegistryClient,
    name: &str,
    req: &VersionReq,
) -> SyxResult<String> {
    let index = client.fetch_index(name)?;
    let mut candidates: Vec<Version> = index
        .versions
        .iter()
        .filter(|v| !v.yanked)
        .filter_map(|v| Version::parse(&v.version).ok())
        .filter(|v| req.matches(v))
        .collect();
    candidates.sort();
    candidates
        .last()
        .map(|v| v.to_string())
        .ok_or_else(|| {
            SyxDiagnostic::error(format!(
                "No registry version of '{name}' satisfies {req}"
            ))
            .with_code("P003")
        })
}

pub fn download_package(client: &RegistryClient, name: &str, version: &str) -> SyxResult<Vec<u8>> {
    client.download(name, version)
}
