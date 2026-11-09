use std::path::PathBuf;

/// Core prelude modules — Phase 83 SYX 61.0 (Core Authority; type names as prelude modules).
const CORE: &[(&str, &str)] = &[
    (
        "Hasher",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/hasher.sy"
        )),
    ),
    (
        "Int8",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/int8.sy"
        )),
    ),
    (
        "Int16",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/int16.sy"
        )),
    ),
    (
        "Int32",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/int32.sy"
        )),
    ),
    (
        "Int64",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/int64.sy"
        )),
    ),
    (
        "Int128",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/int128.sy"
        )),
    ),
    (
        "UInt8",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/uint8.sy"
        )),
    ),
    (
        "UInt16",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/uint16.sy"
        )),
    ),
    (
        "UInt32",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/uint32.sy"
        )),
    ),
    (
        "UInt64",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/uint64.sy"
        )),
    ),
    (
        "UInt128",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/uint128.sy"
        )),
    ),
    (
        "Int",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/int.sy"
        )),
    ),
    (
        "UInt",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/uint.sy"
        )),
    ),
    (
        "Float32",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/float32.sy"
        )),
    ),
    (
        "Float64",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/float64.sy"
        )),
    ),
    (
        "Float",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/float.sy"
        )),
    ),
    (
        "Double",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/double.sy"
        )),
    ),
    (
        "Bool",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/scalar/bool.sy"
        )),
    ),
    (
        "Character",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/scalar/character.sy"
        )),
    ),
    (
        "String",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/text/string.sy"
        )),
    ),
    (
        "Array",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/collections/array.sy"
        )),
    ),
    (
        "Set",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/collections/set.sy"
        )),
    ),
    (
        "Dictionary",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/collections/dictionary.sy"
        )),
    ),
    (
        "Result",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/algebraic/result.sy"
        )),
    ),
    (
        "BigInt",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/numeric/bigint.sy"
        )),
    ),
    (
        "Concurrency",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/concurrency.sy"
        )),
    ),
];

/// Embedded protocol definitions (not prelude-injected; used for jump-to-def and semantic registration).
const PROTOCOLS: &[(&str, &str)] = &[
    (
        "Equatable",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/protocols/equatable.sy"
        )),
    ),
    (
        "Hashable",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/protocols/hashable.sy"
        )),
    ),
    (
        "Comparable",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/protocols/comparable.sy"
        )),
    ),
    (
        "CustomStringConvertible",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/protocols/custom_string_convertible.sy"
        )),
    ),
    (
        "Codable",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/protocols/codable.sy"
        )),
    ),
    (
        "Error",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/protocols/error.sy"
        )),
    ),
];

const REFERENCE_BUILTINS: &[(&str, &str)] = &[
    (
        "Object",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/reference/object.sy"
        )),
    ),
    (
        "ObjectIdentifier",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/reference/object_identifier.sy"
        )),
    ),
];

const VOID_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/core/builtins/special/void.sy"
));

/// Top-level host builtins (`print` / `readLine` / `reflect`) — browsable authority.
const GLOBALS_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/core/builtins/special/globals.sy"
));

const GLOBALS_CORE_PATH: &str = "core/builtins/special/globals.sy";

const GLOBAL_BUILTIN_NAMES: &[&str] = &["print", "readLine", "reflect"];

const HTTP_RESPONSE_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/io/types/http_response.sy"
));

const INTERNAL_TYPES_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/internal/types.sy"
));

const INTERNAL_MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/internal/manifest.json"
));

const NET_SOURCE: &str = concat!(
    // ADR-236: named HTTP types before Http/Url facades that return them.
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/io/types/http_response.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/http_types.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/mod.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/url.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/websocket.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/tcp.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/udp.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/net/dns.sy"
    )),
);

const TEXT_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/text/mod.sy"
));

const SYNC_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/sync/mod.sy"
));

/// SA-3-01 / SA-4-04a: System aggregate = internal `SyxHost` then public domain sources.
/// `__syx_*` live only in `packages/internal/system_host.sy`.
const SYSTEM_SOURCE: &str = concat!(
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/internal/system_host.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/core/env.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/core/path.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/core/time.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/datetime/mod.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/os/mod.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/process/mod.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/process/process_builder.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/io/mod.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/io/file_handle.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/io/file_stream.sy"
    )),
    "\n",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/io/file_watcher.sy"
    )),
);

const SQL_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/data/sql.sy"
));

const TESTING_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/testing/mod.sy"
));

const LOG_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/log/mod.sy"
));

const TRACE_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/trace/mod.sy"
));

const METRICS_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/metrics/mod.sy"
));

const PROFILER_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/profiler/mod.sy"
));

const DI_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/di/mod.sy"
));

const UI_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/ui/mod.sy"
));

const LINALG_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../packages/linalg/mod.sy"
));

/// Explicit import modules (`import System` / `import Networking` / …).
const IMPORTABLE: &[(&str, &str)] = &[
    ("System", SYSTEM_SOURCE),
    ("Networking", NET_SOURCE),
    (
        "Crypto",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/crypto/mod.sy"
        )),
    ),
    ("Text", TEXT_SOURCE),
    ("Synchronization", SYNC_SOURCE),
    ("SQL", SQL_SOURCE),
    ("Testing", TESTING_SOURCE),
    ("Logging", LOG_SOURCE),
    ("Tracing", TRACE_SOURCE),
    ("Metrics", METRICS_SOURCE),
    ("Profiling", PROFILER_SOURCE),
    ("DI", DI_SOURCE),
    ("UI", UI_SOURCE),
    ("Numerics", LINALG_SOURCE),
    (
        "JSON",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/json.sy"
        )),
    ),
    (
        "Fmt",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/text/fmt.sy"
        )),
    ),
    // SA-3-05 Collections merge deferred: OrderedSet/Deque stay separate importables
    // (homonym classes shadow builtins if concatenated under `import Collections`).
    (
        "OrderedSet",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/collections/ordered_set.sy"
        )),
    ),
    (
        "Deque",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/builtins/collections/deque.sy"
        )),
    ),
    (
        "Math",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/math.sy"
        )),
    ),
    (
        "Random",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/random.sy"
        )),
    ),
    (
        "SIMD4",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/core/simd.sy"
        )),
    ),
];

/// Names users may write after `import ` (stdlib explicit-import surface).
pub fn importable_module_names() -> impl Iterator<Item = &'static str> {
    IMPORTABLE.iter().map(|(name, _)| *name)
}

pub fn core_prelude() -> &'static [(&'static str, &'static str)] {
    CORE
}

/// SA-4-04: stdlib `__syx_*` authority (no faer `plugin_*`).
pub fn syx_host_source() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/internal/system_host.sy"
    ))
}

/// SA-4-04: Numerics/linalg host (includes faer `plugin_*`).
pub fn linalg_host_source() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../packages/internal/linalg_host.sy"
    ))
}

fn host_source_for_module(module_name: &str) -> &'static str {
    match module_name {
        "Numerics" => linalg_host_source(),
        _ => syx_host_source(),
    }
}

/// Prepend `SyxHost` when the unit does not already define it (idempotent).
pub fn ensure_syx_host(src: &str) -> String {
    ensure_syx_host_for("System", src)
}

/// Prepend the host surface appropriate for `module_name`.
pub fn ensure_syx_host_for(module_name: &str, src: &str) -> String {
    if src.contains("enum SyxHost") {
        return src.to_string();
    }
    let host = host_source_for_module(module_name);
    let mut out = String::with_capacity(host.len() + src.len() + 1);
    out.push_str(host);
    out.push('\n');
    out.push_str(src);
    out
}

pub fn embedded_protocols() -> &'static [(&'static str, &'static str)] {
    PROTOCOLS
}

/// Reference builtins (`Object`, `ObjectIdentifier`) — not in `core_prelude`, but
/// must still land in the module graph so extension methods (`Object_valueForKey`, …)
/// merge into MIR like Dictionary/Array witnesses.
pub fn embedded_reference_builtins() -> &'static [(&'static str, &'static str)] {
    REFERENCE_BUILTINS
}

pub fn lookup(module_name: &str) -> Option<(&'static str, &'static str)> {
    let first = module_name.split('.').next().unwrap_or(module_name);
    IMPORTABLE
        .iter()
        .find(|(name, _)| *name == first)
        .map(|(name, source)| (*name, *source))
}

pub fn lookup_core(module_name: &str) -> Option<(&'static str, &'static str)> {
    CORE.iter()
        .chain(PROTOCOLS.iter())
        .chain(REFERENCE_BUILTINS.iter())
        .find(|(name, _)| *name == module_name)
        .map(|(name, source)| (*name, *source))
}

/// Phase 44: resolve builtin type/protocol surface name to Core authority source.
pub fn lookup_core_builtin(surface: &str) -> Option<&'static str> {
    if let Some((_, src)) = CORE.iter().find(|(n, _)| *n == surface) {
        return Some(src);
    }
    if let Some((_, src)) = PROTOCOLS.iter().find(|(n, _)| *n == surface) {
        return Some(src);
    }
    if let Some((_, src)) = REFERENCE_BUILTINS.iter().find(|(n, _)| *n == surface) {
        return Some(src);
    }
    match surface {
        "Void" => Some(VOID_SOURCE),
        "HTTPResponse" => Some(HTTP_RESPONSE_SOURCE),
        _ => lookup_global_builtin(surface),
    }
}

/// Authority source for language-global host builtins (`print`, `readLine`, `reflect`).
pub fn globals_source() -> &'static str {
    GLOBALS_SOURCE
}

pub fn is_global_builtin(name: &str) -> bool {
    GLOBAL_BUILTIN_NAMES.contains(&name)
}

pub fn lookup_global_builtin(name: &str) -> Option<&'static str> {
    if is_global_builtin(name) {
        Some(GLOBALS_SOURCE)
    } else {
        None
    }
}

pub fn virtual_global_builtin_path() -> PathBuf {
    PathBuf::from(format!("<stdlib>/{GLOBALS_CORE_PATH}"))
}

/// Walk up from `start` looking for a monorepo `packages/<rel>` file.
pub fn find_workspace_packages_file(
    start: &std::path::Path,
    rel_under_packages: &str,
) -> Option<PathBuf> {
    let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    let mut cur = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start
    };
    let rel = if rel_under_packages.starts_with("packages/") {
        rel_under_packages.to_string()
    } else {
        format!("packages/{rel_under_packages}")
    };
    for _ in 0..12 {
        let candidate = cur.join(&rel);
        if candidate.is_file() {
            return Some(candidate);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

pub fn virtual_core_builtin_path(surface: &str) -> Option<PathBuf> {
    if is_global_builtin(surface) {
        return Some(virtual_global_builtin_path());
    }
    syx_stdlib::lookup_manifest_entry(surface)
        .map(|e| PathBuf::from(format!("<stdlib>/{}", e.core_path)))
}

pub fn virtual_path(name: &str) -> PathBuf {
    PathBuf::from(format!("<stdlib>/{name}.sy"))
}

/// Single-file importable stdlib modules under `packages/<dir>/mod.sy`.
/// `System` / `Networking` are multi-file conceptually but aggregated on disk for overlay.
pub fn workspace_single_file_relpath(module_name: &str) -> Option<&'static str> {
    let first = module_name.split('.').next().unwrap_or(module_name);
    match first {
        "System" => Some("packages/system/mod.sy"),
        "Networking" => Some("packages/networking/mod.sy"),
        "Crypto" => Some("packages/crypto/mod.sy"),
        "Text" => Some("packages/text/mod.sy"),
        "Synchronization" => Some("packages/sync/mod.sy"),
        "SQL" => Some("packages/data/sql.sy"),
        "Testing" => Some("packages/testing/mod.sy"),
        "Logging" => Some("packages/log/mod.sy"),
        "Tracing" => Some("packages/trace/mod.sy"),
        "Metrics" => Some("packages/metrics/mod.sy"),
        "Profiling" => Some("packages/profiler/mod.sy"),
        "DI" => Some("packages/di/mod.sy"),
        "UI" => Some("packages/ui/mod.sy"),
        "Numerics" => Some("packages/linalg/mod.sy"),
        "JSON" => Some("packages/core/json.sy"),
        "Fmt" => Some("packages/text/fmt.sy"),
        "OrderedSet" => Some("packages/core/builtins/collections/ordered_set.sy"),
        "Deque" => Some("packages/core/builtins/collections/deque.sy"),
        "Math" => Some("packages/core/math.sy"),
        "Random" => Some("packages/core/random.sy"),
        "SIMD4" => Some("packages/core/simd.sy"),
        _ => None,
    }
}

/// Walk up from `start` looking for a monorepo `packages/.../mod.sy` overlay.
/// Lets LSP/check pick up stdlib edits without requiring a `syx` rebuild.
pub fn find_workspace_stdlib_file(start: &std::path::Path, module_name: &str) -> Option<PathBuf> {
    let rel = workspace_single_file_relpath(module_name)?;
    // Prefer a canonical start so relative `.` / `./foo` can still walk up to
    // the monorepo root (PathBuf::pop on `.` does not leave the cwd).
    let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    let mut cur = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start
    };
    for _ in 0..12 {
        let candidate = cur.join(rel);
        if candidate.is_file() {
            return Some(candidate);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

pub fn internal_types_source() -> &'static str {
    INTERNAL_TYPES_SOURCE
}

pub fn internal_manifest_json() -> &'static str {
    INTERNAL_MANIFEST
}

pub fn virtual_internal_types_path() -> PathBuf {
    PathBuf::from("<stdlib>/internal/types.sy")
}

/// Phase 42+: modules that must not appear in user `import` statements.
pub fn is_forbidden_import(name: &str) -> bool {
    matches!(name, "Internal" | "_Builtin" | "_Internal" | "internal")
}

/// Phase 44: reserved core prelude module names (shadow guard S023).
pub fn is_prelude_module_name(name: &str) -> bool {
    syx_config::CORE_PRELUDE_MODULES.iter().any(|m| *m == name)
}

/// Phase 44: retired Phase 43 *Ops / Arrays module names (clean break).
/// SA-0-03: live `import Text` is importable — not retired.
pub fn is_retired_prelude_name(name: &str) -> bool {
    syx_stdlib::retired_ops_module_names().contains(&name)
}

/// Phase 44: parse `@builtin(id)` internal_id from embedded Core builtin source.
pub fn builtin_internal_id_from_source(source: &str) -> Option<&str> {
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("@builtin(") {
            if let Some(id) = rest.strip_suffix(')') {
                let id = id.trim().trim_matches('"');
                return Some(id);
            }
        }
        if let Some(rest) = trimmed.strip_prefix("@host(") {
            if let Some(id) = rest.strip_suffix(')') {
                let id = id.trim().trim_matches('"');
                return Some(id);
            }
        }
    }
    None
}
