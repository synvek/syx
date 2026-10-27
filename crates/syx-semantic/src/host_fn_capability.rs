//! M3-E03 / ADR-313: compile-time HostFn capability-table lookup.
//! Backend columns come from `packages/internal/host_fn_capabilities.generated.json`.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde_json::Value;

const HOST_FN_CAPABILITIES_JSON: &str =
    include_str!("../../../packages/internal/host_fn_capabilities.generated.json");

/// Known backend columns in the IR HostFn capability table.
pub const HOST_FN_BACKENDS: &[&str] = &["vm", "mir", "llvm", "wasm", "cranelift"];

#[derive(Debug, Clone)]
pub struct HostFnCapability {
    pub status: String,
    pub backends: HashMap<String, String>,
    #[allow(dead_code)]
    pub notes: String,
}

static TABLE: OnceLock<HashMap<String, HostFnCapability>> = OnceLock::new();

fn capability_table() -> &'static HashMap<String, HostFnCapability> {
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        let v: Value = serde_json::from_str(HOST_FN_CAPABILITIES_JSON).unwrap_or(Value::Null);
        if let Some(arr) = v.get("hosts").and_then(|h| h.as_array()) {
            for host in arr {
                let Some(name) = host.get("name").and_then(|n| n.as_str()) else {
                    continue;
                };
                let status = host
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("unsupported")
                    .to_string();
                let notes = host
                    .get("notes")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let mut backends = HashMap::new();
                if let Some(b) = host.get("backends").and_then(|x| x.as_object()) {
                    for (k, val) in b {
                        if let Some(s) = val.as_str() {
                            backends.insert(k.clone(), s.to_string());
                        }
                    }
                }
                for key in HOST_FN_BACKENDS {
                    backends
                        .entry((*key).to_string())
                        .or_insert_with(|| "unsupported".to_string());
                }
                map.insert(
                    name.to_string(),
                    HostFnCapability {
                        status,
                        backends,
                        notes,
                    },
                );
            }
        }
        map
    })
}

/// Full capability row for a HostFn inventory name (e.g. `FileExists`).
pub fn host_fn_capability(capability_name: &str) -> Option<&'static HostFnCapability> {
    capability_table().get(capability_name)
}

/// Overall capability-table `status` for a HostFn inventory name.
pub fn host_fn_overall_status(capability_name: &str) -> Option<&'static str> {
    host_fn_capability(capability_name).map(|c| c.status.as_str())
}

/// Per-backend status (`vm` / `mir` / `llvm` / `wasm` / `cranelift`).
pub fn host_fn_backend_status(capability_name: &str, backend: &str) -> Option<&'static str> {
    host_fn_capability(capability_name).and_then(|c| c.backends.get(backend).map(|s| s.as_str()))
}

/// Fail-closed: explicit `unsupported`, or missing row (table default).
pub fn host_fn_is_unsupported(capability_name: &str) -> bool {
    match host_fn_overall_status(capability_name) {
        Some("unsupported") | None => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_print_has_backend_columns() {
        let cap = host_fn_capability("FmtPrint").expect("FmtPrint row");
        assert_ne!(cap.status, "unsupported");
        for backend in HOST_FN_BACKENDS {
            assert!(
                cap.backends.contains_key(*backend),
                "missing backend {backend}"
            );
        }
        assert_eq!(host_fn_backend_status("FmtPrint", "vm"), Some("real"));
    }

    #[test]
    fn missing_host_is_unsupported() {
        assert!(host_fn_is_unsupported("DefinitelyNotAHostFn"));
        assert!(host_fn_backend_status("DefinitelyNotAHostFn", "vm").is_none());
    }
}
