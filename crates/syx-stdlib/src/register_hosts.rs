//! Phase 47: central Host manifest — single audit point for manifest `host_ops` parity.

use crate::bigint_hosts::BIGINT_HOST_TABLE;
use crate::collection_hosts::COLLECTION_HOST_TABLE;
use crate::{lookup_manifest_entry, manifest_entries};
use crate::numeric_hosts::{host_slug, NUMERIC_HOST_TABLE};

/// True when `surface.method` is declared in manifest `host_ops`.
pub fn is_manifest_host_op(surface: &str, op: &str) -> bool {
    let Some(entry) = lookup_manifest_entry(surface) else {
        return false;
    };
    let Some(ops) = entry.host_ops else {
        return false;
    };
    // Public size API is only `count` (no `length` / `len` aliases).
    ops.contains(&op)
}

/// Resolve manifest `host_ops` entry to expected `__syx_*` host name.
pub fn manifest_host_name(surface: &str, op: &str) -> Option<String> {
    let entry = lookup_manifest_entry(surface)?;
    let canonical = entry.alias_of.unwrap_or(entry.surface);
    if canonical == "Array" {
        // Public names → host slugs (`count`→`len`; camelCase method → snake host).
        let host_op = match op {
            "count" => "len",
            "sortBy" => "sort_by",
            "flatMap" => "flat_map",
            other => other,
        };
        return Some(format!("__syx_array_{host_op}"));
    }
    if canonical == "Set" {
        return Some(format!("__syx_set_{op}"));
    }
    if canonical == "Dictionary" {
        let op = match op {
            "containsKey" | "contains" => "contains",
            other => other,
        };
        return Some(format!("__syx_dict_{op}"));
    }
    if canonical == "BigInt" {
        return match op {
            "from_string" => Some("__syx_bigint_from_string".into()),
            "from_i128" => Some("__syx_bigint_from_i128".into()),
            other => Some(format!("__syx_bigint_{other}")),
        };
    }
    if canonical == "Result" {
        return Some(format!("__syx_result_{op}"));
    }
    let slug = host_slug(canonical)?;
    Some(format!("__syx_{slug}_{op}"))
}

/// All host names from static tables (numeric + collection + bigint + result).
pub fn all_registered_host_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = NUMERIC_HOST_TABLE.iter().map(|e| e.name).collect();
    names.extend(COLLECTION_HOST_TABLE.iter().map(|e| e.name));
    names.extend(BIGINT_HOST_TABLE.iter().map(|e| e.name));
    names.extend(crate::result_hosts::RESULT_HOST_TABLE.iter().map(|e| e.name));
    names
}

/// Verify every manifest `host_ops` entry has a registered host (Phase 47 gate).
pub fn manifest_host_ops_parity() -> Vec<String> {
    let mut missing = Vec::new();
    let registered: std::collections::HashSet<&str> =
        all_registered_host_names().into_iter().collect();
    for e in manifest_entries() {
        let Some(ops) = e.host_ops else { continue };
        let surface = e.alias_of.unwrap_or(e.surface);
        for op in ops {
            let Some(name) = manifest_host_name(surface, op) else {
                missing.push(format!("{surface}.{op}: no name mapping"));
                continue;
            };
            if !registered.contains(name.as_str()) {
                missing.push(format!("{surface}.{op} -> {name}"));
            }
        }
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_host_ops_all_registered() {
        let missing = manifest_host_ops_parity();
        assert!(
            missing.is_empty(),
            "missing hosts: {:?}",
            missing
        );
    }
}
