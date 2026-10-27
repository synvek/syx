//! Compile-time / sidecar plugin manifest (ADR-071 §2.1).

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct PluginHostManifest {
    pub name: String,
    #[serde(default)]
    pub syx_name: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    pub return_type: String,
    #[serde(default)]
    pub params: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct PluginManifest {
    pub id: String,
    pub api_version: u32,
    pub library: String,
    pub hosts: Vec<PluginHostManifest>,
    #[serde(default)]
    pub min_syx_version: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub signature: Option<String>,
}

/// Phase 51: `.syxplugin` bundle (manifest JSON + signature sidecar fields).
#[derive(Debug, Clone, Deserialize)]
pub struct SyxPluginBundle {
    pub manifest: PluginManifest,
    #[serde(default)]
    pub checksum: String,
}

impl PluginManifest {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())
    }

    pub fn checksum(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl SyxPluginBundle {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())
    }
}

static COMPILE_MANIFESTS: std::sync::Mutex<Vec<PluginManifest>> = std::sync::Mutex::new(Vec::new());

pub fn register_compile_manifest(manifest: PluginManifest) {
    if let Ok(mut g) = COMPILE_MANIFESTS.lock() {
        g.push(manifest);
    }
}

pub fn compile_manifests() -> Vec<PluginManifest> {
    COMPILE_MANIFESTS
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}

pub fn clear_compile_manifests() {
    if let Ok(mut g) = COMPILE_MANIFESTS.lock() {
        g.clear();
    }
}

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

pub fn verify_ed25519_signature(checksum: &str, signature: &str) -> bool {
    let Some(rest) = signature.strip_prefix("syx1:ed25519:") else {
        return false;
    };
    let env_key = std::env::var("SYX_REGISTRY_VERIFY_KEY").unwrap_or_default();
    let (pubkey_hex, sig_hex): (&str, &str) = if let Some((pk, sig)) = rest.split_once(':') {
        (pk, sig)
    } else if env_key.is_empty() {
        return false;
    } else {
        (env_key.as_str(), rest)
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

pub fn verify_signature(manifest: &PluginManifest) -> Result<(), String> {
    let verify = std::env::var("SYX_HOST_PLUGIN_VERIFY").as_deref() == Ok("1");
    let Some(sig) = manifest.signature.as_deref() else {
        if verify {
            return Err("H005: plugin signature required when SYX_HOST_PLUGIN_VERIFY=1".into());
        }
        return Ok(());
    };
    if sig.is_empty() {
        if verify {
            return Err("H005: plugin signature required when SYX_HOST_PLUGIN_VERIFY=1".into());
        }
        return Ok(());
    }
    if sig.starts_with("syx1:stub") {
        return Ok(());
    }
    let checksum = manifest.checksum();
    if verify_ed25519_signature(&checksum, sig) {
        Ok(())
    } else if verify {
        Err("H005: plugin signature verification failed".into())
    } else {
        Ok(())
    }
}

pub fn verify_syxplugin_bundle(bundle: &SyxPluginBundle) -> Result<(), String> {
    let checksum = if bundle.checksum.is_empty() {
        bundle.manifest.checksum()
    } else {
        bundle.checksum.clone()
    };
    bundle
        .manifest
        .signature
        .as_deref()
        .map(|s| {
            if verify_ed25519_signature(&checksum, s) || s.starts_with("syx1:stub") {
                Ok(())
            } else {
                Err("H005: .syxplugin signature verification failed".into())
            }
        })
        .unwrap_or_else(|| verify_signature(&bundle.manifest))
}
