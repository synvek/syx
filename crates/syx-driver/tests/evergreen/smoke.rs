//! SYX architecture smoke — manifest, ADR, invariants.
// was: smoke

use syx_driver::{compile_file_with_options, emit_mir_text, DriverOptions, NativeBackend};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn adr_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/adr/architecture-decisions.md")
}

#[test]
fn manifest_current() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    assert_eq!(m["phase"], 139);
    // S2-I264: public version follows crate semver (CARGO_PKG_VERSION).
    assert_eq!(m["version"], env!("CARGO_PKG_VERSION"));
}

#[test]
fn architecture_decisions_exists() {
    let adr = std::fs::read_to_string(adr_path()).expect("architecture-decisions.md");
    assert!(
        adr.contains("SYX 117.0")
            || adr.contains("Phase 139")
            || adr.contains("SYX 96.0")
            || adr.contains("Phase 118")
    );
    assert!(
        adr.contains("LLVM is the sole native backend")
            || adr.contains("fileprivate")
            || adr.contains("Decision")
    );
}

#[test]
fn cache_v91() {
    // Cache tag + heap digest live in peeled modules (not hub lib.rs).
    let fp = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/cache_fingerprint.rs"),
    )
    .unwrap();
    let lib = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"),
    )
    .unwrap();
    assert!(
        fp.contains("syx-cache-v91") || fp.contains("syx-cache-v1:"),
        "cache fingerprint must retain v91 history or v1 tag bake"
    );
    assert!(
        lib.contains("heap_v100_phase128_track_c_wasm_digest")
            || fp.contains("heap_v100_phase128_track_c_wasm_digest"),
        "heap_v100 digest helper must remain wired"
    );
}

#[test]
fn cranelift_enum_removed() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"),
    )
    .unwrap();
    assert!(!src.contains("NativeBackend::Cranelift"));
    assert!(NativeBackend::parse("cranelift").is_err());
}

#[test]
fn add_assign_host_registered() {
    assert!(syx_stdlib::lookup_numeric_host("__syx_int_add_assign").is_some());
    assert!(syx_stdlib::lookup_numeric_host("__syx_int_sub_assign").is_some());
}

#[test]
fn seccomp_no_fallback() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-macro/src/external_sandbox.rs"),
    )
    .unwrap();
    assert!(!src.contains("or_else(|_| InProcessMacroExpander"));
    assert!(src.contains("E004"));
}

#[test]
fn c5b_wasm_sandbox_fail_closed_without_wasmtime() {
    use std::ffi::CString;
    use syx_driver::embed_v2::{
        embed_wasm_inprocess_fallback_allowed, syx_embed_v2_free, syx_embed_v2_run,
    };

    // Deny wasmtime via capability list so fail-closed does not depend on PATH.
    std::env::set_var("SYX_EMBED_WASM_SANDBOX", "1");
    std::env::set_var("SYX_EMBED_CAPABILITIES", "print");
    std::env::remove_var("SYX_EMBED_DENY_UNKNOWN");

    assert!(
        !embed_wasm_inprocess_fallback_allowed(),
        "C-5b: WASM_SANDBOX=1 must forbid in-process fallback"
    );

    let src = CString::new("func main() { print(1) }").unwrap();
    let ptr = syx_embed_v2_run(src.as_ptr());
    assert!(
        ptr.is_null(),
        "C-5b: SYX_EMBED_WASM_SANDBOX=1 with wasmtime unavailable must return null"
    );
    syx_embed_v2_free(ptr);

    std::env::remove_var("SYX_EMBED_WASM_SANDBOX");
    std::env::remove_var("SYX_EMBED_CAPABILITIES");
}

#[test]
fn c5b_default_still_inprocess() {
    use std::ffi::CString;
    use syx_driver::embed_v2::{
        embed_wasm_inprocess_fallback_allowed, syx_embed_v2_free, syx_embed_v2_run,
    };

    std::env::remove_var("SYX_EMBED_WASM_SANDBOX");
    std::env::remove_var("SYX_EMBED_CAPABILITIES");
    assert!(
        embed_wasm_inprocess_fallback_allowed(),
        "default must allow in-process"
    );

    let src = CString::new("func main() { print(42) }").unwrap();
    let ptr = syx_embed_v2_run(src.as_ptr());
    assert!(
        !ptr.is_null(),
        "C-5b: without WASM_SANDBOX, in-process embed must still succeed"
    );
    let result = unsafe { &*ptr };
    assert_eq!(result.exit_code, 0);
    syx_embed_v2_free(ptr);
}

#[test]
fn c5c1_inprocess_timeout_and_cli_merge() {
    use std::ffi::CString;
    use std::sync::Mutex;
    use syx_config::{EmbedConfig, SyxRuntimeConfig};
    use syx_driver::embed_v2::{embed_timeout_ms, syx_embed_v2_free, syx_embed_v2_run};

    // Serialize env/config mutation (parallel evergreen tests share process env).
    static LOCK: Mutex<()> = Mutex::new(());
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::remove_var("SYX_EMBED_WASM_SANDBOX");
    std::env::remove_var("SYX_EMBED_TIMEOUT_MS");

    let mut cfg = SyxRuntimeConfig::default();
    cfg.embed = EmbedConfig {
        sandbox: false,
        timeout_ms: Some(999),
        memory_mb: None,
    };
    syx_config::install_runtime_config(cfg);
    assert_eq!(
        embed_timeout_ms(),
        Some(999),
        "CLI/config timeout when env unset"
    );

    std::env::set_var("SYX_EMBED_TIMEOUT_MS", "42");
    assert_eq!(embed_timeout_ms(), Some(42), "env must win over CLI");

    // Hang guest: worker exceeds wall clock → null (C-5c-1).
    std::env::set_var("SYX_EMBED_TIMEOUT_MS", "30");
    let src = CString::new("func main() { while true { } }").unwrap();
    let ptr = syx_embed_v2_run(src.as_ptr());
    assert!(
        ptr.is_null(),
        "C-5c-1: SYX_EMBED_TIMEOUT_MS must null on hang guest"
    );
    syx_embed_v2_free(ptr);

    std::env::remove_var("SYX_EMBED_TIMEOUT_MS");
    syx_config::install_runtime_config(SyxRuntimeConfig::default());
}

#[test]
fn c5c2_embed_seccomp_non_linux_null() {
    use std::ffi::CString;
    use std::sync::Mutex;
    use syx_driver::embed_v2::syx_embed_v2_run;
    #[cfg(target_os = "linux")]
    use syx_driver::embed_v2::syx_embed_v2_free;

    static LOCK: Mutex<()> = Mutex::new(());
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::remove_var("SYX_EMBED_WASM_SANDBOX");
    std::env::remove_var("SYX_EMBED_TIMEOUT_MS");
    std::env::set_var("SYX_EMBED_SECCOMP", "1");
    let src = CString::new("func main() { print(1) }").unwrap();
    let ptr = syx_embed_v2_run(src.as_ptr());
    #[cfg(not(target_os = "linux"))]
    assert!(
        ptr.is_null(),
        "C-5c-2: non-Linux SYX_EMBED_SECCOMP=1 must null"
    );
    #[cfg(target_os = "linux")]
    {
        // Linux: install in worker; simple print should still succeed under net-deny.
        assert!(
            !ptr.is_null(),
            "C-5c-2: Linux SYX_EMBED_SECCOMP=1 should run print under net-deny"
        );
        syx_embed_v2_free(ptr);
    }
    std::env::remove_var("SYX_EMBED_SECCOMP");
}

#[test]
fn c5c2_hooks_and_mode_labels() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let embed = std::fs::read_to_string(root.join("crates/syx-driver/src/embed_v2.rs"))
        .expect("embed_v2");
    assert!(
        embed.contains("embed_seccomp_mvp_enabled")
            && embed.contains("try_install_seccomp_netdeny")
            && embed.contains("C-5c-2"),
        "embed must wire seccomp install"
    );
    let mac = std::fs::read_to_string(root.join("crates/syx-macro/src/external_sandbox.rs"))
        .expect("sandbox");
    assert!(
        mac.contains("try_install_seccomp_netdeny")
            && (mac.contains("MODE_SECCOMP_NETDENY_WIRED")
                || mac.contains("seccomp-netdeny-wired")),
        "macro must install + wired mode"
    );
    let filt = std::fs::read_to_string(root.join("crates/syx-macro/src/seccomp_linux.rs"))
        .expect("seccomp_linux");
    assert!(
        filt.contains("SYS_socket")
            && filt.contains("SYS_connect")
            && filt.contains("PR_SET_SECCOMP")
            && filt.contains("net-deny"),
        "filter must be net-deny BPF"
    );

    // Mode labels (env mutation serialized lightly).
    std::env::remove_var("SYX_MACRO_SECCOMP");
    assert_eq!(syx_macro::macro_sandbox_mode(), "sandbox-reexec-only");
    std::env::set_var("SYX_MACRO_SECCOMP", "1");
    #[cfg(target_os = "linux")]
    assert_eq!(
        syx_macro::macro_sandbox_mode(),
        syx_macro::MODE_SECCOMP_NETDENY_WIRED
    );
    #[cfg(not(target_os = "linux"))]
    assert_eq!(
        syx_macro::macro_sandbox_mode(),
        syx_macro::MODE_SECCOMP_UNSUPPORTED
    );
    std::env::remove_var("SYX_MACRO_SECCOMP");
}

#[test]
fn c5d_embed_rss_non_linux_null() {
    use std::ffi::CString;
    use std::sync::Mutex;
    use syx_driver::embed_v2::syx_embed_v2_run;
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    use syx_driver::embed_v2::syx_embed_v2_free;

    static LOCK: Mutex<()> = Mutex::new(());
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::remove_var("SYX_EMBED_WASM_SANDBOX");
    std::env::remove_var("SYX_EMBED_TIMEOUT_MS");
    std::env::remove_var("SYX_EMBED_SECCOMP");
    // Generous AS/Job wall so Linux/Windows print still succeeds.
    std::env::set_var("SYX_EMBED_RSS_MB", "2048");
    let src = CString::new("func main() { print(1) }").unwrap();
    let ptr = syx_embed_v2_run(src.as_ptr());
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    assert!(
        ptr.is_null(),
        "C-5d/C-5e: macOS/other SYX_EMBED_RSS_MB must null (fail-closed)"
    );
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        assert!(
            !ptr.is_null(),
            "C-5d/C-5e: Linux/Windows SYX_EMBED_RSS_MB=2048 should run print under hard wall"
        );
        syx_embed_v2_free(ptr);
    }
    std::env::remove_var("SYX_EMBED_RSS_MB");
}

#[test]
fn c5e_jobobject_hooks_and_fail_closed() {
    use std::ffi::CString;
    use std::sync::Mutex;
    use syx_driver::embed_v2::syx_embed_v2_run;

    static LOCK: Mutex<()> = Mutex::new(());
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let jo = std::fs::read_to_string(root.join("crates/syx-macro/src/jobobject_windows.rs"))
        .expect("jobobject_windows");
    assert!(
        jo.contains("CreateJobObjectW")
            && jo.contains("AssignProcessToJobObject")
            && jo.contains("PROCESS_MEMORY_LIMIT")
            && jo.contains("JOBOBJECT_EXTENDED_LIMIT_INFORMATION")
            && jo.contains("try_apply_rss_hard_wall_bytes")
            && jo.contains("MODE_JOB_OBJECT_MEMORY_WIRED")
            && jo.contains("C-5e"),
        "jobobject_windows must wire CreateJobObject + PROCESS_MEMORY_LIMIT"
    );
    let embed = std::fs::read_to_string(root.join("crates/syx-driver/src/embed_v2.rs"))
        .expect("embed_v2");
    assert!(
        embed.contains("try_apply_rss_hard_wall")
            && embed.contains("C-5e")
            && embed.contains("target_os = \"windows\""),
        "embed must dispatch RSS hard wall to Windows Job Object"
    );
    let mac = std::fs::read_to_string(root.join("crates/syx-macro/src/external_sandbox.rs"))
        .expect("sandbox");
    assert!(
        mac.contains("try_apply_rss_hard_wall_for_pid") && mac.contains("C-5e"),
        "macro parent must assign Job Object on Windows"
    );

    // macOS/other: RSS still fail-closed (C-5e does not claim macOS).
    std::env::remove_var("SYX_EMBED_WASM_SANDBOX");
    std::env::remove_var("SYX_EMBED_SECCOMP");
    std::env::set_var("SYX_EMBED_RSS_MB", "64");
    let src = CString::new("func main() { print(1) }").unwrap();
    let ptr = syx_embed_v2_run(src.as_ptr());
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    assert!(
        ptr.is_null(),
        "C-5e: non-Linux/Windows SYX_EMBED_RSS_MB must remain null"
    );
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        use syx_driver::embed_v2::syx_embed_v2_free;
        if !ptr.is_null() {
            syx_embed_v2_free(ptr);
        }
    }
    std::env::remove_var("SYX_EMBED_RSS_MB");

    assert_eq!(
        syx_macro::MODE_JOB_OBJECT_MEMORY_WIRED,
        "jobobject-memory-wired"
    );
    assert_eq!(
        syx_macro::MODE_JOB_OBJECT_UNSUPPORTED,
        "jobobject-unsupported-fail-closed"
    );
}

#[test]
fn c5d_memory_cli_merge_and_hooks() {
    use std::sync::Mutex;
    use syx_config::{EmbedConfig, SyxRuntimeConfig};
    use syx_driver::embed_v2::embed_memory_limit_bytes;

    static LOCK: Mutex<()> = Mutex::new(());
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::remove_var("SYX_EMBED_MEMORY_MB");
    let mut cfg = SyxRuntimeConfig::default();
    cfg.embed = EmbedConfig {
        sandbox: false,
        timeout_ms: None,
        memory_mb: Some(3),
    };
    syx_config::install_runtime_config(cfg);
    assert_eq!(
        embed_memory_limit_bytes(),
        Some(3 * 1024 * 1024),
        "CLI/config memory when env unset"
    );
    std::env::set_var("SYX_EMBED_MEMORY_MB", "7");
    assert_eq!(
        embed_memory_limit_bytes(),
        Some(7 * 1024 * 1024),
        "env must win over CLI memory_mb"
    );
    std::env::remove_var("SYX_EMBED_MEMORY_MB");
    syx_config::install_runtime_config(SyxRuntimeConfig::default());

    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let embed = std::fs::read_to_string(root.join("crates/syx-driver/src/embed_v2.rs"))
        .expect("embed_v2");
    assert!(
        embed.contains("try_set_as_limit_bytes")
            && embed.contains("try_set_as_limit_bytes_for_pid")
            && embed.contains("C-5d")
            && embed.contains("runtime_config()")
            && embed.contains("memory_mb"),
        "embed must wire prlimit + MEMORY env∪CLI"
    );
    let mac = std::fs::read_to_string(root.join("crates/syx-macro/src/external_sandbox.rs"))
        .expect("sandbox");
    assert!(
        mac.contains("SYX_MACRO_RSS_MB")
            && mac.contains("try_set_as_limit_bytes")
            && mac.contains("C-5d"),
        "macro must wire RLIMIT_AS + fail-closed"
    );
    let rl = std::fs::read_to_string(root.join("crates/syx-macro/src/rlimit_linux.rs")).expect("rl");
    assert!(
        rl.contains("RLIMIT_AS") && rl.contains("prlimit") && rl.contains("MODE_RLIMIT_AS_WIRED"),
        "rlimit_linux must use prlimit(RLIMIT_AS)"
    );
}

#[test]
fn c5c1_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let embed = std::fs::read_to_string(root.join("crates/syx-driver/src/embed_v2.rs"))
        .expect("embed_v2.rs");
    assert!(
        embed.contains("wait_command_output_deadline")
            && embed.contains("child.kill()")
            && embed.contains("runtime_config().embed.timeout_ms")
            && embed.contains("C-5c-1"),
        "embed must wire wasmtime deadline kill + CLI merge"
    );
    assert!(
        !embed.contains("let _ = timeout_secs"),
        "must not discard timeout_secs"
    );

    let mac = std::fs::read_to_string(root.join("crates/syx-macro/src/external_sandbox.rs"))
        .expect("external_sandbox.rs");
    assert!(
        mac.contains("SYX_MACRO_TIMEOUT_MS")
            && mac.contains("wait_child_output_deadline")
            && mac.contains("child.kill()")
            && mac.contains("M040"),
        "macro must opt-in wall clock → M040"
    );
}

#[test]
fn object_drop_llvm_abi() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-llvm/src/lib.rs"),
    )
    .unwrap();
    // Wave A W534 / ADR-878: handle C ABI declares deleted.
    assert!(!src.contains("declare void @syx_rt_object_retain"));
    assert!(!src.contains("declare void @syx_rt_object_release"));
    assert!(!src.contains("declare i32 @syx_rt_object_drop"));
}

#[test]
fn wasm_object_imports() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-wasm-run/src/lib.rs"),
    )
    .unwrap();
    // Wave A W534 / ADR-878: link_syx_rt_object_abi retired.
    assert!(!src.contains("fn link_syx_rt_object_abi"));
}

#[test]
fn mir_object_handle_drop_peeled() {
    // Wave A W541 / ADR-885: object_handle_drop drained with handle table peel.
    // Wave A W544 / ADR-888: MirOp::ObjectRefWrap also peeled (DictBuild dual-track birth kept).
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-mir-run/src/runtime.rs"),
    )
    .unwrap();
    assert!(
        !src.contains("register_object_handle")
            && !src.contains("MirOp::ObjectRefWrap =>")
            && !src.contains("fn object_ref_wrap")
            && (src.contains("ADR-885")
                || src.contains("peel object_handle")
                || src.contains("ADR-888")),
        "mir-run must not call peeled register_object_handle; MirOp ObjectRef path must be absent"
    );
}

#[test]
fn semantic_e005() {
    // Witness check peeled into semantic_analyzer.rs (hub lib.rs no longer spells it).
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-semantic/src/semantic_analyzer.rs"),
    )
    .unwrap();
    assert!(src.contains("check_dataclass_codable_witness"));
    assert!(src.contains("E005"));
}

#[test]
fn llvm_sole_native_documented() {
    let adr = std::fs::read_to_string(adr_path()).unwrap();
    assert!(adr.contains("LLVM is the sole native backend"));
}

#[test]
fn lsp_macro_expanded_field() {
    // Field moved with LSP IDE peel out of hub lib.rs.
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lsp_ide.rs"),
    )
    .unwrap();
    assert!(src.contains("macro_expanded"));
}

#[test]
fn witness_matrix_attached() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-macro/src/attached.rs"),
    )
    .unwrap();
    assert!(src.contains("witness_constraint_matrix"));
}

#[test]
fn data_class_where_parser() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-parser/src/lib.rs"),
    )
    .unwrap();
    assert!(src.contains("parse_where_clause"));
}

#[test]
fn engine_fixed_point() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-macro/src/engine.rs"),
    )
    .unwrap();
    assert!(src.contains("expand_to_fixed_point"));
}

#[test]
fn assign_mir_hosts() {
    let mir = emit_mir_text(
        &fixture("operators/assign_host.sy"),
        &DriverOptions::default(),
    )
    .unwrap();
    assert!(mir.contains("__syx_int_add_assign") || mir.contains("CallHost"));
}

#[test]
fn generic_witness_missing_fail() {
    assert!(compile_file_with_options(
        &fixture("macros/generic_witness_missing.sy"),
        &DriverOptions::default()
    )
    .is_err());
}

#[test]
fn int_compare_hosts() {
    for slug in [
        "__syx_int_eq",
        "__syx_int_ne",
        "__syx_int_lt",
        "__syx_int_le",
        "__syx_int_compare",
    ] {
        assert!(syx_stdlib::lookup_numeric_host(slug).is_some(), "{slug}");
    }
}

#[test]
fn llvm_only_backend() {
    assert_eq!(DriverOptions::default().native_backend, NativeBackend::Llvm);
}

#[test]
fn object_handles_module() {
    // Wave A W541 / ADR-885: object_handles.rs peeled (table API deleted).
    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-runtime-core/src/object_handles.rs")
            .exists(),
        "object_handles.rs must remain deleted after W541 peel"
    );
}

#[test]
fn cranelift_compat_deleted() {
    assert!(!std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/cranelift_compat.rs")
        .exists());
}
