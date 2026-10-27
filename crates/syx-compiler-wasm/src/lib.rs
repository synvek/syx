//! Browser-targeted SYX compiler (FrontendPipeline → MIR → WASM bytes).

use std::ffi::c_char;
use std::slice;
use std::sync::Mutex;

use getrandom::register_custom_getrandom;
use serde::Serialize;
use syx_config::{SyxRuntimeConfig, WasmConfig};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_frontend::{FrontendOptions, FrontendPipeline, StopAfter};
use syx_mir::MirModule;
use syx_mir_run::{install_module_string_pool, MirRuntime};

/// Deterministic fill for wasm32 / custom getrandom — compiler path does not need CSPRNG;
/// keeps the cdylib free of wasm-bindgen so the page can `WebAssembly.instantiate` raw.
fn compiler_getrandom(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    for (i, b) in dest.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(17).wrapping_add(0x5A);
    }
    Ok(())
}
register_custom_getrandom!(compiler_getrandom);

static LAST_WASM: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static LAST_DIAGNOSTICS_JSON: Mutex<Option<String>> = Mutex::new(None);
/// Guest AOT host state for the last successful compile (browser `syx.*` imports).
static LAST_GUEST_RT: Mutex<Option<MirRuntime>> = Mutex::new(None);

fn install_guest_runtime(mir: &MirModule) {
    let mut rt = MirRuntime::new();
    rt.install_functions(mir.functions.clone());
    install_module_string_pool(&mut rt, mir);
    let mut guard = LAST_GUEST_RT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(rt);
}

#[derive(Debug, Clone, Serialize)]
pub struct CompileDiagnostic {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompileResult {
    pub ok: bool,
    pub diagnostics: Vec<CompileDiagnostic>,
}

impl CompileResult {
    fn from_error(err: SyxDiagnostic) -> Self {
        Self {
            ok: false,
            diagnostics: vec![CompileDiagnostic {
                message: err.message,
                code: err.code,
            }],
        }
    }

    fn ok() -> Self {
        Self {
            ok: true,
            diagnostics: vec![],
        }
    }
}

fn diagnostic_from_err(err: SyxDiagnostic) -> CompileDiagnostic {
    CompileDiagnostic {
        message: err.message,
        code: err.code,
    }
}

fn browser_runtime_config() -> SyxRuntimeConfig {
    SyxRuntimeConfig {
        wasm: WasmConfig {
            aot: true,
            mir_exec_only: false,
        },
        ..SyxRuntimeConfig::default()
    }
}

const BROWSER_PRELUDE: &str = include_str!(concat!(env!("OUT_DIR"), "/browser_prelude.sy"));

/// Phase 69: embedded browser prelude (driver protocol graph + Iterator `next() -> T?`).
pub fn browser_prelude_source() -> &'static str {
    BROWSER_PRELUDE
}

/// Phase 72: SHA-256 fingerprint of embedded browser prelude (cache v44 / CI parity).
pub fn browser_prelude_graph_hash() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(browser_prelude_source().as_bytes());
    format!("{:x}", hasher.finalize())
}

fn with_browser_prelude(source: &str) -> String {
    format!("{BROWSER_PRELUDE}\n{source}")
}

/// Phase 68: compile with embedded prelude unified graph (class + Comparable + sorted).
pub fn compile_source_with_prelude_diagnostics(name: &str, source: &str) -> CompileResult {
    compile_source_with_diagnostics(name, &with_browser_prelude(source))
}

/// Compile a single source string to MIR module via FrontendPipeline (M1-A02).
pub fn compile_source_to_mir(name: &str, source: &str) -> SyxResult<MirModule> {
    let file = SourceFile::new(name.to_string(), source.to_string());
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Mir,
        desugar_top_level: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file, &opts)?;
    art.mir
        .ok_or_else(|| SyxDiagnostic::error("FrontendPipeline returned no MIR"))
}

/// Compile source to MIR text (legacy wasm_web glue).
pub fn compile_source_to_mir_text(name: &str, source: &str) -> SyxResult<String> {
    Ok(syx_mir::format_mir_module(&compile_source_to_mir(name, source)?))
}

/// Compile SYX source with structured diagnostics (no WASM on failure).
pub fn compile_source_with_diagnostics(name: &str, source: &str) -> CompileResult {
    let file = SourceFile::new(name.to_string(), source.to_string());
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Mir,
        desugar_top_level: true,
        collecting_semantic: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = match FrontendPipeline::run(file, &opts) {
        Ok(a) => a,
        Err(e) => return CompileResult::from_error(e),
    };
    if !art.diagnostics.is_empty() {
        return CompileResult {
            ok: false,
            diagnostics: art
                .diagnostics
                .into_iter()
                .map(diagnostic_from_err)
                .collect(),
        };
    }
    if let Some(sem) = &art.semantic {
        if !sem.errors.is_empty() {
            return CompileResult {
                ok: false,
                diagnostics: sem.errors.iter().cloned().map(diagnostic_from_err).collect(),
            };
        }
    }
    let Some(mir) = art.mir else {
        return CompileResult::from_error(SyxDiagnostic::error("FrontendPipeline returned no MIR"));
    };
    syx_config::install_runtime_config(browser_runtime_config());
    match syx_wasm_emit::emit_wasm_with_options(&mir, true) {
        Ok(bytes) => {
            install_guest_runtime(&mir);
            let mut guard = LAST_WASM.lock().unwrap_or_else(|e| e.into_inner());
            *guard = Some(bytes);
            CompileResult::ok()
        }
        Err(e) => CompileResult::from_error(e),
    }
}

/// Compile SYX source to WASM bytes with tier-1 AOT enabled.
pub fn compile_source_to_wasm_bytes(name: &str, source: &str) -> SyxResult<Vec<u8>> {
    syx_config::install_runtime_config(browser_runtime_config());
    let mir = compile_source_to_mir(name, source)?;
    syx_wasm_emit::emit_wasm_with_options(&mir, true)
}

fn store_diagnostics_json(result: &CompileResult) {
    let json = serde_json::to_string(result).unwrap_or_else(|_| {
        r#"{"ok":false,"diagnostics":[{"message":"serialization failed","code":null}]}"#.into()
    });
    let mut guard = LAST_DIAGNOSTICS_JSON
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    *guard = Some(json);
}

/// C ABI: bump-style alloc for host JS to write UTF-8 / out-len slots into linear memory.
///
/// # Safety
/// Caller owns the returned region of `size` bytes until [`dealloc`].
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let mut buf = Vec::<u8>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// C ABI: free a region previously returned by [`alloc`].
///
/// # Safety
/// `ptr` must be from [`alloc`] with the same `size`, or null.
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    let _ = Vec::from_raw_parts(ptr, 0, size);
}

/// C ABI: compile source → WASM bytes stored in crate-global buffer; returns pointer or null.
///
/// # Safety
/// `name`/`src` must be valid UTF-8 for `name_len`/`src_len`. `out_len` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn syx_compile_source_to_wasm_bytes(
    name: *const c_char,
    name_len: usize,
    src: *const c_char,
    src_len: usize,
    out_len: *mut usize,
) -> *const u8 {
    if name.is_null() || src.is_null() || out_len.is_null() {
        return std::ptr::null();
    }
    let name = match utf8_cstr(name, name_len) {
        Ok(s) => s,
        Err(_) => return std::ptr::null(),
    };
    let src = match utf8_cstr(src, src_len) {
        Ok(s) => s,
        Err(_) => return std::ptr::null(),
    };
    let result = compile_source_with_diagnostics(&name, &src);
    store_diagnostics_json(&result);
    if !result.ok {
        *out_len = 0;
        return std::ptr::null();
    }
    let guard = LAST_WASM.lock().unwrap_or_else(|e| e.into_inner());
    let Some(buf) = guard.as_ref() else {
        *out_len = 0;
        return std::ptr::null();
    };
    *out_len = buf.len();
    buf.as_ptr()
}

/// C ABI: run one MIR opcode against the last compile's guest [`MirRuntime`].
///
/// `mem` / `mem_len` is the guest linear memory (CONST_STR etc. read from it). May be null/0.
///
/// # Safety
/// `mem` must be valid for `mem_len` bytes when non-null.
#[no_mangle]
pub unsafe extern "C" fn syx_guest_mir_exec(
    op: i32,
    a: i32,
    b: i32,
    mem: *mut u8,
    mem_len: usize,
) -> i32 {
    let mut guard = LAST_GUEST_RT.lock().unwrap_or_else(|e| e.into_inner());
    let Some(rt) = guard.as_mut() else {
        return 0;
    };
    let mut owned = Vec::new();
    let memory: &mut [u8] = if !mem.is_null() && mem_len > 0 {
        slice::from_raw_parts_mut(mem, mem_len)
    } else {
        &mut owned
    };
    rt.exec(op as u32, a, b, 0, memory).unwrap_or(0)
}

/// C ABI: `__syx_aot_end_fn` host.
#[no_mangle]
pub extern "C" fn syx_guest_end_fn() {
    let mut guard = LAST_GUEST_RT.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(rt) = guard.as_mut() {
        rt.end_function();
    }
}

/// C ABI: `__syx_aot_take_catch_pc` host.
#[no_mangle]
pub extern "C" fn syx_guest_take_catch_pc(passthrough: i32) -> i32 {
    let mut guard = LAST_GUEST_RT.lock().unwrap_or_else(|e| e.into_inner());
    let Some(rt) = guard.as_mut() else {
        return passthrough;
    };
    let catch = rt.take_catch_pc();
    if catch >= 0 {
        -(catch + 2)
    } else {
        passthrough
    }
}

/// C ABI: pop/convert RT return for display (AOT `main` export is exit-code 0).
#[no_mangle]
pub extern "C" fn syx_guest_get_return() -> i32 {
    let mut guard = LAST_GUEST_RT.lock().unwrap_or_else(|e| e.into_inner());
    let Some(rt) = guard.as_mut() else {
        return 0;
    };
    rt.return_value()
}

/// C ABI: last compile diagnostics as UTF-8 JSON (`CompileResult`); pointer valid until next compile.
///
/// # Safety
/// `out_len` must be non-null when called.
#[no_mangle]
pub unsafe extern "C" fn syx_last_compile_diagnostics_json(out_len: *mut usize) -> *const u8 {
    if out_len.is_null() {
        return std::ptr::null();
    }
    let guard = LAST_DIAGNOSTICS_JSON
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let Some(json) = guard.as_ref() else {
        *out_len = 0;
        return std::ptr::null();
    };
    *out_len = json.len();
    json.as_ptr()
}

unsafe fn utf8_cstr(ptr: *const c_char, len: usize) -> Result<String, ()> {
    let bytes = slice::from_raw_parts(ptr as *const u8, len);
    std::str::from_utf8(bytes)
        .map(|s| s.to_string())
        .map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_hello_main() {
        let mir = compile_source_to_mir(
            "main.sy",
            r#"func main() { print("hi") }"#,
        )
        .expect("mir");
        assert!(mir.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn compiles_to_wasm_bytes() {
        let bytes = compile_source_to_wasm_bytes(
            "main.sy",
            r#"func main() { return 42 }"#,
        )
        .expect("wasm");
        assert!(bytes.len() > 8);
        assert_eq!(&bytes[0..4], b"\0asm");
    }

    // Host wasmtime roundtrip lives in syx-wasm-run / driver tests — keep this
    // crate free of syx-wasm-run so wasm32 feature unification stays slim.

    #[test]
    fn diagnostics_on_parse_error() {
        let result = compile_source_with_diagnostics("main.sy", "func main() { ??? }");
        assert!(!result.ok);
        assert!(!result.diagnostics.is_empty());
    }

    #[test]
    fn diagnostics_ok_on_valid_source() {
        let result = compile_source_with_diagnostics(
            "main.sy",
            r#"func main() { return 1 }"#,
        );
        assert!(result.ok);
        assert!(result.diagnostics.is_empty());
    }
}
