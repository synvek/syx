//! SYX 17.5 runtime configuration — CLI-first with deprecated env fallback (ADR-052).

use std::sync::{Mutex, MutexGuard, OnceLock};

/// Core prelude module names auto-visible when stdlib is enabled (ADR-064, Phase 44 SYX 22.0).
pub const CORE_PRELUDE_MODULES: &[&str] = &[
    "Math",
    "Int8", "Int16", "Int32", "Int64", "Int128",
    "UInt8", "UInt16", "UInt32", "UInt64", "UInt128",
    "Int", "UInt",
    "Float32", "Float64", "Float", "Double",
    "Bool", "Character", "String",
    "Array", "Set", "Dictionary", "OrderedSet", "Deque", "Result", "BigInt",
    "Hasher", "Random", "JSON", "Time", "Env", "Path",
    "Concurrency", "SIMD4", "Fmt",
];

#[derive(Debug, Clone)]
pub struct EmbedConfig {
    pub sandbox: bool,
    pub timeout_ms: Option<u64>,
    pub memory_mb: Option<u64>,
}

impl Default for EmbedConfig {
    fn default() -> Self {
        Self {
            sandbox: false,
            timeout_ms: None,
            memory_mb: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeBackendChoice {
    Llvm,
}

impl Default for NativeBackendChoice {
    fn default() -> Self {
        Self::Llvm
    }
}

#[derive(Debug, Clone)]
pub struct WasmConfig {
    pub aot: bool,
    /// Force 100% `mir_exec` fallback (debug parity with interpreter).
    pub mir_exec_only: bool,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            aot: env_truthy("SYX_WASM_AOT"),
            mir_exec_only: env_truthy("SYX_WASM_MIR_EXEC_ONLY"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LlvmConfig {
    pub aot: bool,
}

impl Default for LlvmConfig {
    fn default() -> Self {
        Self { aot: llvm_aot_from_env() }
    }
}

#[derive(Debug, Clone)]
pub struct CraneliftConfig {
    pub minimal: bool,
    pub p20_gate: bool,
}

impl Default for CraneliftConfig {
    fn default() -> Self {
        Self {
            minimal: env_truthy("SYX_CRANELIFT_MINIMAL"),
            p20_gate: cranelift_p20_from_env(),
        }
    }
}

/// M3-E02 / ADR-310–311 + W1035 / ADR-1379 + W1036 / ADR-1380: opt-in permission sandbox + domain grants.
#[derive(Debug, Clone)]
pub struct PermissionGrants {
    /// When true, un-granted domains fail closed at semantic check.
    pub sandbox: bool,
    /// Blanket grant for the `net` domain (TcpConnect / HttpGet thin slice).
    pub allow_net: bool,
    /// Blanket grant for the `fs` domain (FileRead / FileWrite deepen slice).
    pub allow_fs: bool,
    /// Blanket grant for the `env` domain (EnvGet / EnvSet thin slice).
    pub allow_env: bool,
    /// Blanket grant for the `process` domain (ProcessExec / ProcessSpawn thin slice).
    pub allow_process: bool,
    /// Blanket grant for the `plugin` domain (`__syx_plugin_*` prefix thin slice).
    pub allow_plugin: bool,
}

impl Default for PermissionGrants {
    fn default() -> Self {
        Self {
            sandbox: env_truthy("SYX_PERMISSION_SANDBOX"),
            allow_net: env_truthy("SYX_ALLOW_NET"),
            allow_fs: env_truthy("SYX_ALLOW_FS"),
            allow_env: env_truthy("SYX_ALLOW_ENV"),
            allow_process: env_truthy("SYX_ALLOW_PROCESS"),
            allow_plugin: env_truthy("SYX_ALLOW_PLUGIN"),
        }
    }
}

impl PermissionGrants {
    /// OR-merge CLI / env / `syx.toml` `[permissions]` flags into this grant set.
    pub fn merge_flags(
        &mut self,
        sandbox: bool,
        allow_net: bool,
        allow_fs: bool,
        allow_env: bool,
        allow_process: bool,
        allow_plugin: bool,
    ) {
        self.sandbox |= sandbox;
        self.allow_net |= allow_net;
        self.allow_fs |= allow_fs;
        self.allow_env |= allow_env;
        self.allow_process |= allow_process;
        self.allow_plugin |= allow_plugin;
    }
}

/// Unified runtime configuration (Phase 37 GB).
#[derive(Debug, Clone)]
pub struct SyxRuntimeConfig {
    pub jit_tier2: bool,
    pub jit_fastpath: bool,
    pub enable_std: bool,
    /// Phase 38: digest/HMAC Host functions (`import Crypto`).
    pub enable_crypto: bool,
    /// Phase 38: SIMD types and ops.
    pub enable_simd: bool,
    /// Phase 40: property wrapper parser/desugar GA (`--no-property-wrapper` opt-out).
    pub enable_property_wrapper: bool,
    /// Phase 41: user `#macro` param gensym hygiene (`--no-macro-hygiene` opt-out).
    pub enable_macro_hygiene: bool,
    /// Phase 41: WASM guest simd128 pilot for `SIMD4<Int32> +` (`SYX_WASM_SIMD128=1`).
    pub enable_wasm_simd128: bool,
    pub embed: EmbedConfig,
    pub wasm: WasmConfig,
    pub llvm: LlvmConfig,
    pub cranelift: CraneliftConfig,
    pub native_backend: NativeBackendChoice,
    /// M3-E02 permission grants (default off / env-backed).
    pub permission: PermissionGrants,
    /// M3-E03 / ADR-313: opt-in HostFn capability-table compile gate (`unsupported` → S026).
    pub ir_capability_gate: bool,
}

impl Default for SyxRuntimeConfig {
    fn default() -> Self {
        Self {
            jit_tier2: jit_tier2_from_env(),
            jit_fastpath: env_truthy("SYX_JIT_FASTPATH"),
            enable_std: true,
            enable_crypto: env_falsey_opt_out("SYX_ENABLE_CRYPTO", true),
            enable_simd: env_falsey_opt_out("SYX_ENABLE_SIMD", true),
            enable_property_wrapper: env_falsey_opt_out("SYX_PROPERTY_WRAPPER_PARSER", true),
            enable_macro_hygiene: env_falsey_opt_out("SYX_MACRO_HYGIENE", true),
            enable_wasm_simd128: env_truthy("SYX_WASM_SIMD128"),
            embed: EmbedConfig::default(),
            wasm: WasmConfig::default(),
            llvm: LlvmConfig::default(),
            cranelift: CraneliftConfig::default(),
            native_backend: NativeBackendChoice::default(),
            permission: PermissionGrants::default(),
            ir_capability_gate: env_truthy("SYX_IR_CAPABILITY_GATE"),
        }
    }
}

impl SyxRuntimeConfig {
    pub fn from_cli(
        jit: Option<bool>,
        jit_fastpath: Option<bool>,
        enable_std: bool,
        enable_crypto: Option<bool>,
        enable_simd: Option<bool>,
        enable_property_wrapper: Option<bool>,
        enable_macro_hygiene: Option<bool>,
        embed_sandbox: Option<bool>,
        embed_timeout_ms: Option<u64>,
        embed_memory_mb: Option<u64>,
        llvm_aot: Option<bool>,
        wasm_aot: Option<bool>,
        wasm_mir_exec_only: Option<bool>,
        backend: Option<NativeBackendChoice>,
        permission_sandbox: Option<bool>,
        allow_net: Option<bool>,
        allow_fs: Option<bool>,
        allow_env: Option<bool>,
        allow_process: Option<bool>,
        allow_plugin: Option<bool>,
        ir_capability_gate: Option<bool>,
    ) -> Self {
        let mut cfg = Self::default();
        cfg.enable_std = enable_std;
        if let Some(on) = enable_crypto {
            cfg.enable_crypto = on;
        }
        if let Some(on) = enable_simd {
            cfg.enable_simd = on;
        }
        if let Some(on) = enable_property_wrapper {
            cfg.enable_property_wrapper = on;
        }
        if let Some(on) = enable_macro_hygiene {
            cfg.enable_macro_hygiene = on;
        }
        if let Some(on) = jit {
            cfg.jit_tier2 = on;
        }
        if let Some(on) = jit_fastpath {
            cfg.jit_fastpath = on;
        }
        if let Some(on) = embed_sandbox {
            cfg.embed.sandbox = on;
        }
        if embed_timeout_ms.is_some() {
            cfg.embed.timeout_ms = embed_timeout_ms;
        }
        if embed_memory_mb.is_some() {
            cfg.embed.memory_mb = embed_memory_mb;
        }
        if let Some(on) = llvm_aot {
            cfg.llvm.aot = on;
        }
        if let Some(on) = wasm_aot {
            cfg.wasm.aot = on;
        }
        if let Some(on) = wasm_mir_exec_only {
            cfg.wasm.mir_exec_only = on;
        }
        if let Some(b) = backend {
            cfg.native_backend = b;
        }
        if let Some(on) = permission_sandbox {
            cfg.permission.sandbox = on;
        }
        if let Some(on) = allow_net {
            cfg.permission.allow_net = on;
        }
        if let Some(on) = allow_fs {
            cfg.permission.allow_fs = on;
        }
        if let Some(on) = allow_env {
            cfg.permission.allow_env = on;
        }
        if let Some(on) = allow_process {
            cfg.permission.allow_process = on;
        }
        if let Some(on) = allow_plugin {
            cfg.permission.allow_plugin = on;
        }
        if let Some(on) = ir_capability_gate {
            cfg.ir_capability_gate = on;
        }
        cfg
    }
}

static RUNTIME_CONFIG: OnceLock<Mutex<SyxRuntimeConfig>> = OnceLock::new();

fn runtime_lock() -> MutexGuard<'static, SyxRuntimeConfig> {
    RUNTIME_CONFIG
        .get_or_init(|| Mutex::new(SyxRuntimeConfig::default()))
        .lock()
        .unwrap_or_else(|e: std::sync::PoisonError<_>| e.into_inner())
}

pub fn install_runtime_config(config: SyxRuntimeConfig) {
    *runtime_lock() = config;
}

pub fn runtime_config() -> SyxRuntimeConfig {
    runtime_lock().clone()
}

pub fn jit_tier2_enabled() -> bool {
    runtime_lock().jit_tier2
}

pub fn jit_fastpath_enabled() -> bool {
    runtime_lock().jit_fastpath
}

pub fn llvm_aot_enabled() -> bool {
    runtime_lock().llvm.aot
}

pub fn wasm_aot_enabled() -> bool {
    let cfg = runtime_lock();
    cfg.wasm.aot && !cfg.wasm.mir_exec_only
}

pub fn wasm_mir_exec_only() -> bool {
    runtime_lock().wasm.mir_exec_only
}

/// Phase 90: WASM AOT tier-3 — zero `__syx_aot_exec_op` fallback (`SYX_WASM_AOT_TIER3=0` opt-out).
pub fn wasm_aot_tier3_enabled() -> bool {
    std::env::var("SYX_WASM_AOT_TIER3")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 106 GA: forbid `SYX_WASM_MIR_EXEC_ONLY` and tier3-off as production WASM paths.
pub fn wasm_ga_production_wasm_path() -> bool {
    let mir_exec_only = runtime_lock().wasm.mir_exec_only
        || std::env::var("SYX_WASM_MIR_EXEC_ONLY")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(false);
    !mir_exec_only && wasm_aot_tier3_enabled()
}

/// Phase 91: sync WASM uses `__syx_aot_run_op` instead of `mir_exec` import (`SYX_WASM_ZERO_MIR_EXEC=0` opt-out).
pub fn wasm_aot_zero_mir_exec_enabled() -> bool {
    std::env::var("SYX_WASM_ZERO_MIR_EXEC")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 92: sync WASM AOT must have zero `__syx_aot_run_op` fallback calls (`SYX_WASM_ZERO_RUN_OP=0` opt-out).
pub fn wasm_aot_zero_run_op_enabled() -> bool {
    std::env::var("SYX_WASM_ZERO_RUN_OP")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

pub fn cranelift_p20_gate_enabled() -> bool {
    runtime_lock().cranelift.p20_gate
}

pub fn crypto_enabled() -> bool {
    runtime_lock().enable_crypto
}

pub fn simd_enabled() -> bool {
    runtime_lock().enable_simd
}

pub fn property_wrapper_enabled() -> bool {
    runtime_lock().enable_property_wrapper
}

pub fn macro_hygiene_enabled() -> bool {
    runtime_lock().enable_macro_hygiene
}

pub fn wasm_simd128_enabled() -> bool {
    runtime_lock().enable_wasm_simd128
}

pub fn parse_backend(s: &str) -> Option<NativeBackendChoice> {
    match s.to_ascii_lowercase().as_str() {
        "llvm" => Some(NativeBackendChoice::Llvm),
        _ => None,
    }
}

fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn env_falsey_opt_out(name: &str, default: bool) -> bool {
    std::env::var(name)
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(default)
}

fn jit_tier2_from_env() -> bool {
    // ADR-140 / M1-D04: default **off** (opt-in via SYX_JIT_TIER2=1 or `syx --jit`).
    if let Ok(v) = std::env::var("SYX_JIT_TIER2") {
        if v == "0" || v.eq_ignore_ascii_case("false") {
            return false;
        }
        if v == "1" || v.eq_ignore_ascii_case("true") {
            return true;
        }
    }
    false
}

fn llvm_aot_from_env() -> bool {
    if std::env::var("SYX_LLVM_AOT").is_ok() {
        eprintln!("warning: SYX_LLVM_AOT is deprecated; use `syx --llvm-aot` (Phase 37)");
    }
    // Phase 63: LLVM AOT on by default; opt out with SYX_LLVM_AOT=0.
    env_falsey_opt_out("SYX_LLVM_AOT", true)
}

fn cranelift_p20_from_env() -> bool {
    // Phase 37 GD: P20 replaces P19 as prod CI gate default.
    if std::env::var("SYX_CRANELIFT_P19").is_ok() {
        eprintln!("warning: SYX_CRANELIFT_P19 is deprecated; P20 is CI default (Phase 37)");
    }
    if std::env::var("SYX_CRANELIFT_P20").is_ok() {
        eprintln!("warning: SYX_CRANELIFT_P20 is deprecated; use CLI config (Phase 37)");
    }
    env_falsey_opt_out("SYX_CRANELIFT_P20", true)
        || env_truthy("SYX_CRANELIFT_P19")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_cli_can_disable_jit() {
        let cfg = SyxRuntimeConfig::from_cli(
            Some(false),
            None,
            true,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(!cfg.jit_tier2);
    }

    #[test]
    fn config_jit_tier2_default_off() {
        std::env::remove_var("SYX_JIT_TIER2");
        let cfg = SyxRuntimeConfig::default();
        assert!(!cfg.jit_tier2);
    }

    #[test]
    fn config_phase38_feature_flags_default_on() {
        let cfg = SyxRuntimeConfig::default();
        assert!(cfg.enable_crypto);
        assert!(cfg.enable_simd);
    }

    #[test]
    fn config_p20_gate_default_on() {
        let cfg = SyxRuntimeConfig::default();
        assert!(cfg.cranelift.p20_gate);
    }

    #[test]
    fn core_prelude_module_count() {
        assert!(CORE_PRELUDE_MODULES.len() >= 13);
    }
}
