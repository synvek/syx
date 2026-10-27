mod bridge;
mod mir_runtime;
mod atomics;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::{MirModule, MirOp};
use syx_wasm_emit::{aot, host};
use wasmtime::{Caller, Config, Engine, Linker, Module, Store};

// Phase 21 CT1 / Phase 22 CY1: WASM threads MVP + prod opt-in worker queue.
pub const WASM_THREADS_MVP: &str = atomics::WASM_THREADS_MVP;
pub use atomics::{
    get_message, post_message, wasm_thread_worker_consume_fuel, wasm_thread_worker_fuel_remaining,
    wasm_thread_worker_reset_fuel, wasm_threads_mvp_enabled, wasm_threads_nightly_default_enabled,
    wasm_threads_prod_enabled, wasm_threads_release_default, wasm_threads_release_default_evaluated,
    wasm_threads_release_prod_hardened, SharedAtomicsBuffer, WorkerMessage,
};

// Phase 20 CO4 R&D spike retained for reference.
pub const WASM_THREADS_RD_SPIKE: &str = "SharedArrayBuffer+Atomics research only (Phase 20)";

pub use bridge::{install_mir_module, install_wasm_task_hosts, wasm_vm_runtime_for, WasmRunContext};
pub use mir_runtime::MirRuntime;

/// Phase 56: WASM guest can proxy dynamic host plugins through the WASI host.
pub fn wasm_guest_plugin_proxy_enabled() -> bool {
    true
}

fn init_wasm_guest_plugins() {
    if wasm_guest_plugin_proxy_enabled() {
        syx_host_plugin::register_echo_plugin_in_process();
    }
}

/// Host-side WASI preview1 fd table mapped into a temporary directory.
#[derive(Debug)]
pub struct WasiHost {
    pub temp_dir: tempfile::TempDir,
    open_files: HashMap<i32, File>,
    next_fd: i32,
}

impl WasiHost {
    pub fn new() -> SyxResult<Self> {
        Ok(Self {
            temp_dir: tempfile::tempdir().map_err(|e| {
                SyxDiagnostic::error(format!("Failed to create WASI temp dir: {e}"))
            })?,
            open_files: HashMap::new(),
            next_fd: 3,
        })
    }

    pub fn temp_path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }

    fn read_path_from_memory(caller: &mut Caller<'_, WasmStore>, ptr: i32, len: i32) -> Option<String> {
        let memory = caller.get_export("memory")?.into_memory()?;
        let data = memory.data(&mut *caller);
        let start = ptr as usize;
        let end = start.saturating_add(len as usize);
        if end > data.len() {
            return None;
        }
        Some(String::from_utf8_lossy(&data[start..end]).into_owned())
    }

    fn write_i32_to_memory(caller: &mut Caller<'_, WasmStore>, ptr: i32, value: i32) -> bool {
        let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
            return false;
        };
        let offset = ptr as usize;
        let mut data = memory.data_mut(caller);
        if offset.saturating_add(4) > data.len() {
            return false;
        }
        data[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        true
    }

    fn read_iovs(caller: &mut Caller<'_, WasmStore>, iovs: i32, iovs_len: i32) -> Vec<(usize, usize)> {
        let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
            return Vec::new();
        };
        let data = memory.data(&mut *caller);
        let mut out = Vec::new();
        for i in 0..iovs_len as usize {
            let base = iovs as usize + i * 8;
            if base + 8 > data.len() {
                break;
            }
            let buf = i32::from_le_bytes(data[base..base + 4].try_into().unwrap()) as usize;
            let len = i32::from_le_bytes(data[base + 4..base + 8].try_into().unwrap()) as usize;
            out.push((buf, len));
        }
        out
    }

    pub fn close_fd(&mut self, fd: i32) -> bool {
        self.open_files.remove(&fd).is_some()
    }

    pub fn path_open(
        &mut self,
        caller: &mut Caller<'_, WasmStore>,
        _dirfd: i32,
        path_ptr: i32,
        path_len: i32,
        _oflags: i32,
        fd_out_ptr: i32,
    ) -> i32 {
        let Some(rel) = Self::read_path_from_memory(caller, path_ptr, path_len) else {
            return 28; // EINVAL
        };
        let rel = rel.trim_start_matches('/');
        let full = self.temp_dir.path().join(rel);
        if let Some(parent) = full.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&full)
        {
            Ok(f) => f,
            Err(_) => return 44, // ENOENT-ish
        };
        let fd = self.next_fd;
        self.next_fd += 1;
        self.open_files.insert(fd, file);
        if !Self::write_i32_to_memory(caller, fd_out_ptr, fd) {
            return 28;
        }
        0
    }

    pub fn fd_read(
        &mut self,
        caller: &mut Caller<'_, WasmStore>,
        fd: i32,
        iovs: i32,
        iovs_len: i32,
        nread_ptr: i32,
    ) -> i32 {
        let Some(file) = self.open_files.get_mut(&fd) else {
            return 8; // EBADF
        };
        let mut total = 0usize;
        for (buf_ptr, len) in Self::read_iovs(&mut *caller, iovs, iovs_len) {
            let mut chunk = vec![0u8; len];
            let read = file.read(&mut chunk).unwrap_or(0);
            if read == 0 {
                break;
            }
            let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
                return 28;
            };
            let data = memory.data_mut(&mut *caller);
            if buf_ptr + read > data.len() {
                return 28;
            }
            data[buf_ptr..buf_ptr + read].copy_from_slice(&chunk[..read]);
            total += read;
            if read < len {
                break;
            }
        }
        if !Self::write_i32_to_memory(caller, nread_ptr, total as i32) {
            return 28;
        }
        0
    }

    pub fn fd_write(
        &mut self,
        caller: &mut Caller<'_, WasmStore>,
        fd: i32,
        iovs: i32,
        iovs_len: i32,
        nwritten_ptr: i32,
    ) -> i32 {
        let Some(file) = self.open_files.get_mut(&fd) else {
            return 8;
        };
        let mut total = 0usize;
        for (buf_ptr, len) in Self::read_iovs(&mut *caller, iovs, iovs_len) {
            let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
                return 28;
            };
            let data = memory.data(&*caller);
            if buf_ptr + len > data.len() {
                return 28;
            }
            let written = file.write(&data[buf_ptr..buf_ptr + len]).unwrap_or(0);
            total += written;
        }
        let _ = file.flush();
        if !Self::write_i32_to_memory(caller, nwritten_ptr, total as i32) {
            return 28;
        }
        0
    }
}

pub struct WasmStore {
    pub rt: MirRuntime,
    pub ctx: Arc<WasmRunContext>,
    pub wasi: Option<Arc<Mutex<WasiHost>>>,
    /// Wave A W202 / ADR-546: host CallHost failure without active try (VM embed parity).
    pub host_fail: Option<SyxDiagnostic>,
}

pub struct WasmRunResult {
    pub exit_code: i32,
    pub printed: Vec<String>,
}

fn init_wasm_rt(mir: Arc<MirModule>) -> (MirRuntime, Arc<WasmRunContext>) {
    let ctx = install_mir_module(Arc::clone(&mir));
    let mut rt = MirRuntime::default();
    rt.install_functions(mir.functions.clone());
    rt.set_runtime(wasm_vm_runtime_for(Arc::clone(&ctx)));
    syx_mir_run::install_module_string_pool(&mut rt, &mir);
    (rt, ctx)
}

fn collect_printed(ctx: &WasmRunContext, rt: &MirRuntime) -> Vec<String> {
    let printed = ctx.printed_snapshot();
    if printed.is_empty() {
        rt.printed.clone()
    } else {
        printed
    }
}

fn mir_uses_actors(module: &MirModule) -> bool {
    use syx_mir::MirOp;
    module.functions.iter().any(|f| {
        f.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::ActorCall(_) | MirOp::ActorSend(_) | MirOp::ActorRegister))
        })
    })
}

pub fn wasm_mir_fallback_enabled() -> bool {
    std::env::var("SYX_WASM_MIR_FALLBACK")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn wasi_enabled() -> bool {
    std::env::var("SYX_WASI")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn link_wasi_preview1(linker: &mut Linker<WasmStore>) -> SyxResult<()> {
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "environ_sizes_get",
            |_caller: Caller<'_, WasmStore>, env_count: i32, env_buf_size: i32| -> i32 {
                let _ = (env_count, env_buf_size);
                0
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link environ_sizes_get: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "environ_get",
            |_caller: Caller<'_, WasmStore>, _environ: i32, _environ_buf: i32| -> i32 {
                0
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link environ_get: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "path_open",
            |mut caller: Caller<'_, WasmStore>,
             dirfd: i32,
             _dirflags: i32,
             path_ptr: i32,
             path_len: i32,
             _oflags: i32,
             _fs_rights_base: i64,
             _fs_rights_inheriting: i64,
             _fs_flags: i32,
             fd_out: i32| -> i32 {
                let Some(wasi) = caller.data().wasi.clone() else {
                    return 28;
                };
                let Ok(mut host) = wasi.lock() else {
                    return 28;
                };
                host.path_open(&mut caller, dirfd, path_ptr, path_len, 0, fd_out)
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link path_open: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_read",
            |mut caller: Caller<'_, WasmStore>,
             fd: i32,
             iovs: i32,
             iovs_len: i32,
             nread: i32| -> i32 {
                let Some(wasi) = caller.data().wasi.clone() else {
                    return 28;
                };
                let Ok(mut host) = wasi.lock() else {
                    return 28;
                };
                host.fd_read(&mut caller, fd, iovs, iovs_len, nread)
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link fd_read: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_write",
            |mut caller: Caller<'_, WasmStore>,
             fd: i32,
             iovs: i32,
             iovs_len: i32,
             nwritten: i32| -> i32 {
                let Some(wasi) = caller.data().wasi.clone() else {
                    return 28;
                };
                let Ok(mut host) = wasi.lock() else {
                    return 28;
                };
                host.fd_write(&mut caller, fd, iovs, iovs_len, nwritten)
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link fd_write: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_close",
            |caller: Caller<'_, WasmStore>, fd: i32| -> i32 {
                let Some(wasi) = caller.data().wasi.clone() else {
                    return 28;
                };
                let Ok(mut host) = wasi.lock() else {
                    return 28;
                };
                if host.close_fd(fd) {
                    0
                } else {
                    8
                }
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link fd_close: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "clock_time_get",
            |_caller: Caller<'_, WasmStore>, _clock_id: i32, _precision: i64, _time_ptr: i32| -> i32 {
                0
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link clock_time_get: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "random_get",
            |_caller: Caller<'_, WasmStore>, _buf: i32, _buf_len: i32| -> i32 {
                0
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link random_get: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "poll_oneoff",
            |_caller: Caller<'_, WasmStore>,
             _in: i32,
             _out: i32,
             _nsubscriptions: i32,
             _nevents: i32| -> i32 {
                0
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link poll_oneoff: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "sock_open",
            |_caller: Caller<'_, WasmStore>,
             _af: i32,
             _sock_type: i32,
             _protocol: i32,
             _fd_out: i32| -> i32 {
                52 // ENOSYS-style stub for Phase 20 Top-10 parity
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link sock_open: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "sock_send",
            |_caller: Caller<'_, WasmStore>,
             _fd: i32,
             _iovs: i32,
             _iovs_len: i32,
             _flags: i32,
             _nwritten: i32| -> i32 {
                52
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link sock_send: {e}")))?;
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "sock_recv",
            |_caller: Caller<'_, WasmStore>,
             _fd: i32,
             _iovs: i32,
             _iovs_len: i32,
             _flags: i32,
             _nread: i32| -> i32 {
                52
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link sock_recv: {e}")))?;
    link_wasm_gc_stub(linker)?;
    Ok(())
}

/// Phase 21: WASM GC intrinsics stub (docs/10 remaining ❌ — graded no-op for parity).
/// Phase 22: when [`wasi_gc_phase22_enabled`], `alloc` uses [`gc_alloc_noop`] host hook.
pub fn wasi_gc_phase22_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE22")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 22 host hook: noop arena slot (always returns 0 regardless of size).
pub fn gc_alloc_noop(_size: i32) -> i32 {
    0
}

/// Phase 23 DG2: bump arena alloc scaffold (monotonic slot counter).
static GC_BUMP_NEXT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);

pub fn wasi_gc_phase23_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE23")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn gc_alloc_bump(size: i32) -> i32 {
    let slot = GC_BUMP_NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let _ = size.max(0);
    slot
}

pub fn gc_collect_mark_smoke() -> i32 {
    0
}

/// Phase 24 DL1: incremental mark-sweep scaffold (returns swept slot count).
static GC_SWEEP_COUNT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn wasi_gc_phase24_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE24")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn gc_collect_incremental_sweep() -> i32 {
    let n = GC_BUMP_NEXT.load(std::sync::atomic::Ordering::SeqCst).saturating_sub(1);
    GC_SWEEP_COUNT.store(n, std::sync::atomic::Ordering::SeqCst);
    n
}

pub fn gc_sweep_count() -> i32 {
    GC_SWEEP_COUNT.load(std::sync::atomic::Ordering::SeqCst)
}

/// Phase 25 DQ2: unified MirRuntime heap scaffold (`SYX_WASI_GC_PHASE25=1`).
pub fn wasi_gc_phase25_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE25")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 26 DV2: guest `syx_gc` wiring + unified heap (`SYX_WASI_GC_PHASE26=1`).
pub fn wasi_gc_phase26_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE26")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 27 EA2: guest GC default-on evaluation (`SYX_WASI_GC_PHASE27=1` prod gate).
pub fn wasi_gc_phase27_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE27")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase27_default_eval())
}

pub fn wasi_gc_phase27_default_eval() -> bool {
    std::env::var("SYX_WASI_GC_PHASE27_EVAL")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 28 EF2: guest GC prod default on (`SYX_WASI_GC_PHASE28=1`).
pub fn wasi_gc_phase28_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE28")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase28_prod_default())
}

pub fn wasi_gc_phase28_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE28_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 29 EH1/EK3: relocating GC MVP (`SYX_WASI_GC_PHASE29=1` or prod default).
pub fn wasi_gc_phase29_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE29")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase29_prod_default())
}

pub fn wasi_gc_phase29_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE29_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn gc_relocating_compact_budget() -> i32 {
    std::env::var("SYX_WASI_GC_RELOC_BUDGET")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(128)
}

static GC_RELOCATING_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

pub fn gc_relocating_enabled() -> bool {
    GC_RELOCATING_ACTIVE.load(std::sync::atomic::Ordering::SeqCst)
        || wasi_gc_phase29_enabled()
        || wasi_gc_phase30_enabled()
}

pub fn gc_relocating_mark_active() {
    GC_RELOCATING_ACTIVE.store(true, std::sync::atomic::Ordering::SeqCst);
}

pub fn gc_relocating_collect() -> i32 {
    gc_relocating_mark_active();
    let budget = gc_relocating_compact_budget();
    gc_unified_heap_collect() + budget
}

/// Phase 30 EM1: relocating GC prod — write-barrier scaffold + compaction.
pub fn wasi_gc_phase30_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE30")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase30_prod_default())
}

pub fn wasi_gc_phase30_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE30_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

static GC_WRITE_BARRIER_COUNT: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(0);

pub fn gc_write_barrier_mark(slot: i32) -> i32 {
    GC_WRITE_BARRIER_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    slot.max(0)
}

pub fn gc_write_barrier_count() -> i32 {
    GC_WRITE_BARRIER_COUNT.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_relocating_prod_collect() -> i32 {
    gc_relocating_mark_active();
    let barrier = gc_write_barrier_count();
    let budget = gc_relocating_compact_budget();
    gc_unified_heap_collect() + budget + barrier
}

/// Phase 31 ER1: generational relocating GC — young/old heaps + write-barrier deepening.
pub fn wasi_gc_phase31_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE31")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase31_prod_default())
}

pub fn wasi_gc_phase31_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE31_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

static GC_YOUNG_HEAP: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GC_OLD_HEAP: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GC_GENERATIONAL_PROMOTIONS: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(0);

pub fn gc_generational_young_alloc(size: i32) -> i32 {
    let slot = GC_BUMP_NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    GC_YOUNG_HEAP.fetch_add(size.max(0), std::sync::atomic::Ordering::SeqCst);
    gc_write_barrier_mark(slot)
}

pub fn gc_generational_promote(slot: i32) -> i32 {
    GC_GENERATIONAL_PROMOTIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    GC_OLD_HEAP.fetch_add(slot.max(0), std::sync::atomic::Ordering::SeqCst);
    slot.max(0)
}

pub fn gc_generational_young_bytes() -> i32 {
    GC_YOUNG_HEAP.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_generational_old_bytes() -> i32 {
    GC_OLD_HEAP.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_generational_promotion_count() -> i32 {
    GC_GENERATIONAL_PROMOTIONS.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_generational_prod_collect() -> i32 {
    gc_relocating_mark_active();
    let young = gc_generational_young_bytes();
    let old = gc_generational_old_bytes();
    let barrier = gc_write_barrier_count();
    let budget = gc_relocating_compact_budget();
    let swept = gc_unified_heap_collect() + budget + barrier;
    GC_YOUNG_HEAP.store(0, std::sync::atomic::Ordering::SeqCst);
    swept + young + old
}

/// Phase 32 EW1: remembered set + card table generational GC (ADR-041).
pub fn wasi_gc_phase32_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE32")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase32_prod_default())
}

pub fn wasi_gc_phase32_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE32_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 33 FB1: WASI GC Phase 33 — guest prod default on + VM/native generational GC linkage (ADR-043).
pub fn wasi_gc_phase33_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE33")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase33_prod_default())
}

pub fn wasi_gc_phase33_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE33_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(true)
}

/// Phase 33 FB1: VM/native generational GC for `syx run` + native codegen paths.
pub fn vm_native_gc_enabled() -> bool {
    std::env::var("SYX_VM_NATIVE_GC")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(wasi_gc_phase33_enabled())
}

static VM_NATIVE_GC_REMEMBERED_SET: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(0);

pub fn vm_native_gc_remembered_set_mark(slot: i32) -> i32 {
    if !vm_native_gc_enabled() {
        return slot.max(0);
    }
    VM_NATIVE_GC_REMEMBERED_SET.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    gc_remembered_set_mark(slot)
}

pub fn vm_native_gc_remembered_set_count() -> i32 {
    VM_NATIVE_GC_REMEMBERED_SET.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn vm_native_gc_remembered_set_collect() -> i32 {
    if !vm_native_gc_enabled() {
        return 0;
    }
    gc_remembered_set_prod_collect() + vm_native_gc_remembered_set_count()
}

/// Phase 34 FG1: unified tri-backend generational GC prod default on (ADR-045).
pub fn unified_gc_prod_default() -> bool {
    std::env::var("SYX_UNIFIED_GC_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(true)
}

pub fn unified_gc_enabled() -> bool {
    std::env::var("SYX_UNIFIED_GC")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(unified_gc_prod_default() && vm_native_gc_enabled())
}

pub fn unified_gc_remembered_set_mark(slot: i32) -> i32 {
    if !unified_gc_enabled() {
        return slot.max(0);
    }
    vm_native_gc_remembered_set_mark(slot)
}

pub fn unified_gc_remembered_set_count() -> i32 {
    vm_native_gc_remembered_set_count() + gc_remembered_set_count()
}

pub fn unified_gc_remembered_set_collect() -> i32 {
    if !unified_gc_enabled() {
        return 0;
    }
    vm_native_gc_remembered_set_collect() + gc_remembered_set_prod_collect()
}

/// Phase 35 FL1: unified GC promotion prod default on (guest side).
pub fn unified_gc_promotion_prod_default() -> bool {
    std::env::var("SYX_UNIFIED_GC_PROMOTION_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(unified_gc_prod_default())
}

/// Phase 34 FG1: WASI GC Phase 34 — unified barrier with native GC (MIR v23 LICM).
pub fn wasi_gc_phase34_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE34")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase34_prod_default())
}

pub fn wasi_gc_phase34_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE34_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(unified_gc_prod_default())
}

/// Phase 35 FL1: WASI GC Phase 35 — unified GC promotion barrier (MIR v24 LICM).
pub fn wasi_gc_phase35_enabled() -> bool {
    std::env::var("SYX_WASI_GC_PHASE35")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(wasi_gc_phase35_prod_default())
}

pub fn wasi_gc_phase35_prod_default() -> bool {
    std::env::var("SYX_WASI_GC_PHASE35_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(unified_gc_promotion_prod_default())
}

pub fn unified_gc_promote(slot: i32) -> i32 {
    if !wasi_gc_phase35_enabled() && !unified_gc_enabled() {
        return slot.max(0);
    }
    gc_remembered_set_mark(slot)
}

pub fn unified_gc_promote_count() -> i32 {
    gc_remembered_set_count()
}

pub fn unified_gc_promotion_collect() -> i32 {
    if !wasi_gc_phase35_enabled() {
        return unified_gc_remembered_set_collect();
    }
    unified_gc_remembered_set_collect() + unified_gc_promote_count()
}

static GC_REMEMBERED_SET: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static GC_CARD_TABLE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn gc_remembered_set_mark(slot: i32) -> i32 {
    GC_REMEMBERED_SET.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    gc_write_barrier_mark(slot)
}

pub fn gc_card_table_mark(card: i32) -> i32 {
    GC_CARD_TABLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    card.max(0)
}

pub fn gc_remembered_set_count() -> i32 {
    GC_REMEMBERED_SET.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_card_table_count() -> i32 {
    GC_CARD_TABLE.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_remembered_set_prod_collect() -> i32 {
    gc_generational_prod_collect()
        + gc_remembered_set_count()
        + gc_card_table_count()
}

pub fn gc_sweep_incremental_budget() -> i32 {
    std::env::var("SYX_WASI_GC_SWEEP_BUDGET")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(256)
}

static GC_GUEST_WIRED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn gc_guest_wired() -> bool {
    GC_GUEST_WIRED.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn gc_guest_wire_mark() {
    GC_GUEST_WIRED.store(true, std::sync::atomic::Ordering::SeqCst);
}

static GC_UNIFIED_HEAP: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn gc_unified_heap_alloc(size: i32) -> i32 {
    let slot = GC_BUMP_NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    GC_UNIFIED_HEAP.fetch_add(size.max(0), std::sync::atomic::Ordering::SeqCst);
    slot
}

pub fn gc_unified_heap_collect() -> i32 {
    let swept = gc_collect_incremental_sweep();
    GC_UNIFIED_HEAP.store(0, std::sync::atomic::Ordering::SeqCst);
    swept
}

pub fn gc_unified_heap_bytes() -> i32 {
    GC_UNIFIED_HEAP.load(std::sync::atomic::Ordering::SeqCst)
}

fn link_syx_aot_imports(linker: &mut Linker<WasmStore>, wasm_module: &Module) -> SyxResult<()> {
    use syx_mir_run::op;
    use syx_wasm_emit::{aot, host};

    for import in wasm_module.imports() {
        if import.module() != host::MODULE {
            continue;
        }
        if import.name() == aot::CALL_WITNESS {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, fn_idx: i32, _method: i32| -> i32 {
                        use syx_mir_run::run_mir_function_with_args;
                        use syx_runtime_core::Value;
                        let mut owned = Vec::new();
                        let memory: &mut [u8] = if let Some(mem) = caller
                            .get_export("memory")
                            .and_then(|e| e.into_memory())
                        {
                            let len = mem.data_size(&caller) as usize;
                            let ptr = mem.data_ptr(&caller);
                            unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                        } else {
                            &mut owned
                        };
                        let rt = &mut caller.data_mut().rt;
                        let b = rt.stack.pop().unwrap_or(Value::Nil);
                        let a = rt.stack.pop().unwrap_or(Value::Nil);
                        if let Some(func) = rt.functions.get(fn_idx as usize).cloned() {
                            if let Ok(cmp) =
                                run_mir_function_with_args(&func, rt, memory, &[a, b])
                            {
                                rt.stack.push(cmp.clone());
                                return cmp.as_i64() as i32;
                            }
                        }
                        0
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::END_FN {
            let import_name = import.name().to_string();
            linker
                .func_wrap(host::MODULE, import.name(), move |mut caller: Caller<'_, WasmStore>| {
                    caller.data_mut().rt.end_function();
                })
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::TAKE_RT_I32 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>| -> i32 {
                        let v = caller
                            .data_mut()
                            .rt
                            .stack
                            .pop()
                            .unwrap_or(syx_runtime_core::Value::Nil);
                        match v {
                            syx_runtime_core::Value::Bool(b) => i32::from(b),
                            other => other.as_i32(),
                        }
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::STACK_OP {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, op_code: i32| {
                        let mut owned = Vec::new();
                        let memory: &mut [u8] = if let Some(mem) = caller
                            .get_export("memory")
                            .and_then(|e| e.into_memory())
                        {
                            let len = mem.data_size(&caller) as usize;
                            let ptr = mem.data_ptr(&caller);
                            unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                        } else {
                            &mut owned
                        };
                        caller
                            .data_mut()
                            .rt
                            .exec(op_code as u32, 0, 0, 0, memory)
                            .unwrap_or_else(|e| panic!("aot stack_op: {e}"));
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::TAKE_RT_F64 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>| -> f64 {
                        match caller
                            .data_mut()
                            .rt
                            .stack
                            .pop()
                            .unwrap_or(syx_runtime_core::Value::Nil)
                        {
                            syx_runtime_core::Value::F64(v) => v,
                            syx_runtime_core::Value::F32(v) => v as f64,
                            other => other.as_i64() as f64,
                        }
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::TAKE_RT_F32 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>| -> f32 {
                        match caller
                            .data_mut()
                            .rt
                            .stack
                            .pop()
                            .unwrap_or(syx_runtime_core::Value::Nil)
                        {
                            syx_runtime_core::Value::F32(v) => v,
                            syx_runtime_core::Value::F64(v) => v as f32,
                            other => other.as_i64() as f32,
                        }
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::PUSH_RT_F64 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, value: f64| {
                        caller
                            .data_mut()
                            .rt
                            .stack
                            .push(syx_runtime_core::Value::F64(value));
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::PUSH_RT_F32 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, value: f32| {
                        caller
                            .data_mut()
                            .rt
                            .stack
                            .push(syx_runtime_core::Value::F32(value));
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::TAKE_RT_I64 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>| -> i64 {
                        caller
                            .data_mut()
                            .rt
                            .stack
                            .pop()
                            .unwrap_or(syx_runtime_core::Value::Nil)
                            .as_i64()
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::PUSH_RT_I64 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, value: i64| {
                        caller
                            .data_mut()
                            .rt
                            .stack
                            .push(syx_runtime_core::Value::I64(value));
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::SIMD_GUEST_POP_V128 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>| -> wasmtime::V128 {
                        let bits = caller.data_mut().rt.simd_guest_pop_v128_bits();
                        wasmtime::V128::from(bits)
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::SIMD_GUEST_PUSH_V128 {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, element: i32, value: wasmtime::V128| {
                        let _ = caller
                            .data_mut()
                            .rt
                            .simd_guest_push_v128_bits(element, value.as_u128());
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::EXEC_OP || import.name() == aot::RUN_OP {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, op: i32, a: i32, b: i32| -> i32 {
                        let mut owned = Vec::new();
                        let memory: &mut [u8] = if let Some(mem) = caller
                            .get_export("memory")
                            .and_then(|e| e.into_memory())
                        {
                            let len = mem.data_size(&caller) as usize;
                            let ptr = mem.data_ptr(&caller);
                            unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                        } else {
                            &mut owned
                        };
                        caller
                            .data_mut()
                            .rt
                            .exec(op as u32, a, b, 0, memory)
                            .unwrap_or_else(|e| panic!("aot exec_op: {e}"))
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::HEAP_COLLECT {
            let import_name = import.name().to_string();
            linker
                .func_wrap(host::MODULE, import.name(), move |_caller: Caller<'_, WasmStore>| {
                    let _ = gc_unified_heap_collect();
                    let _ = syx_runtime_core::collect_global(&[]);
                })
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        if import.name() == aot::HEAP_COMPACT {
            let import_name = import.name().to_string();
            linker
                .func_wrap(host::MODULE, import.name(), move |mut caller: Caller<'_, WasmStore>| {
                    caller.data_mut().rt.heap_compact();
                })
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        // Wave A W18 / ADR-362: after call_indirect, poll framed pending catch (passthrough i32).
        if import.name() == aot::TAKE_CATCH_PC {
            let import_name = import.name().to_string();
            linker
                .func_wrap(
                    host::MODULE,
                    import.name(),
                    move |mut caller: Caller<'_, WasmStore>, passthrough: i32| -> i32 {
                        let catch = caller.data_mut().rt.take_catch_pc();
                        if catch >= 0 {
                            -(catch + 2)
                        } else {
                            passthrough
                        }
                    },
                )
                .map_err(|e| SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}")))?;
            continue;
        }
        let op_code = match import.name() {
            aot::LOAD_GLOBAL => Some(op::LOAD_GLOBAL),
            aot::LOAD_LOCAL => Some(op::LOAD_LOCAL),
            aot::STORE_GLOBAL => Some(op::STORE_GLOBAL),
            aot::STORE_LOCAL => Some(op::STORE_LOCAL),
            aot::PUSH_RT_I32 => Some(op::CONST_I32),
            aot::PUSH_RT_BOOL => Some(op::CONST_BOOL),
            aot::PUSH_RT_NIL => Some(op::CONST_NIL),
            aot::PUSH_RT_NUMERIC => Some(op::CONST_NUMERIC),
            aot::CONST_STR => Some(op::CONST_STR),
            aot::CONCAT_STR => Some(op::CONCAT_STR),
            aot::TO_STR => Some(op::TO_STR),
            aot::BEGIN_FN => Some(op::BEGIN_FN),
            aot::GET_RETURN => Some(op::GET_RETURN),
            aot::CALL_FN => Some(op::CALL_FN),
            aot::CALL_HOST => Some(op::CALL_HOST),
            aot::SET_FN_INDEX => Some(op::SET_FN_INDEX),
            aot::ARRAY_BUILD => Some(op::ARRAY_BUILD),
            aot::DICT_BUILD => Some(op::DICT_BUILD),
            aot::SET_BUILD => Some(op::SET_BUILD),
            aot::PRINT => Some(op::PRINT),
            aot::MAKE_ITERATOR => Some(op::MAKE_ITERATOR),
            aot::ITERATOR_NEXT => Some(op::ITERATOR_NEXT),
            aot::ARRAY_GET => Some(op::ARRAY_GET),
            aot::ARRAY_SET => Some(op::ARRAY_SET),
            aot::OBJECT_GET => Some(op::OBJECT_GET),
            aot::OBJECT_SET => Some(op::OBJECT_SET),
            aot::VALUE_EQ => Some(op::VALUE_EQ),
            aot::VALUE_NE => Some(op::VALUE_NE),
            aot::REF_EQ => Some(op::REF_EQ),
            aot::REF_NE => Some(op::REF_NE),
            aot::ARRAY_SORTED => Some(op::ARRAY_SORTED),
            aot::MAKE_CLOSURE => Some(op::MAKE_CLOSURE),
            aot::CALL_CLOSURE => Some(op::CALL_CLOSURE),
            aot::AWAIT_POLL => Some(op::AWAIT_POLL),
            aot::AWAIT_SUSPEND => Some(op::AWAIT_SUSPEND),
            aot::ACTOR_CALL => Some(op::ACTOR_CALL),
            aot::ACTOR_SEND => Some(op::ACTOR_SEND),
            aot::ACTOR_REGISTER => Some(op::ACTOR_REGISTER),
            aot::TASK_SPAWN => Some(op::TASK_SPAWN),
            aot::TASK_JOIN => Some(op::TASK_JOIN),
            aot::BEGIN_TRY => Some(op::BEGIN_TRY),
            aot::CATCH => Some(op::CATCH),
            aot::THROW => Some(op::THROW),
            aot::END_TRY => Some(op::END_TRY),
            aot::SIMD_ADD => Some(op::SIMD_ADD),
            aot::SIMD_SUB => Some(op::SIMD_SUB),
            aot::SIMD_MUL => Some(op::SIMD_MUL),
            aot::SIMD_DIV => Some(op::SIMD_DIV),
            aot::SIMD_EQ => Some(op::SIMD_EQ),
            aot::SIMD_LT => Some(op::SIMD_LT),
            aot::SIMD_GT => Some(op::SIMD_GT),
            aot::SIMD_NE => Some(op::SIMD_NE),
            aot::SIMD_LE => Some(op::SIMD_LE),
            aot::SIMD_GE => Some(op::SIMD_GE),
            aot::SIMD_AND => Some(op::SIMD_AND),
            aot::SIMD_OR => Some(op::SIMD_OR),
            aot::SIMD_XOR => Some(op::SIMD_XOR),
            aot::SIMD_NOT => Some(op::SIMD_NOT),
            aot::SIMD_SHL => Some(op::SIMD_SHL),
            aot::SIMD_SHR => Some(op::SIMD_SHR),
            aot::SIMD_MIN => Some(op::SIMD_MIN),
            aot::SIMD_MAX => Some(op::SIMD_MAX),
            aot::SIMD_F32_ADD => Some(op::SIMD_F32_ADD),
            aot::SIMD_F32_SUB => Some(op::SIMD_F32_SUB),
            aot::SIMD_F32_DIV => Some(op::SIMD_F32_DIV),
            aot::SIMD_F64_ADD => Some(op::SIMD_F64_ADD),
            aot::SIMD_F64_SUB => Some(op::SIMD_F64_SUB),
            aot::SIMD_F64_MUL => Some(op::SIMD_F64_MUL),
            aot::SIMD_F64_DIV => Some(op::SIMD_F64_DIV),
            aot::SIMD_I16_ADD => Some(op::SIMD_I16_ADD),
            aot::SIMD_I64_ADD => Some(op::SIMD_I64_ADD),
            aot::SIMD_F32_MUL => Some(op::SIMD_F32_MUL),
            aot::SIMD_LOAD => Some(op::SIMD_LOAD),
            aot::SIMD_STORE => Some(op::SIMD_STORE),
            aot::SIMD_SHUFFLE => Some(op::SIMD_SHUFFLE),
            aot::SIMD_WASM_SPILL => Some(op::SIMD_WASM_SPILL),
            aot::SIMD_WASM_FILL => Some(op::SIMD_WASM_FILL),
            aot::SIMD_GUEST_PUSH_V128 => Some(op::SIMD_GUEST_PUSH_V128),
            aot::SIMD_GUEST_POP_V128 => Some(op::SIMD_GUEST_POP_V128),
            aot::CONST_NUMERIC128 => Some(op::CONST_NUMERIC128),
            // Wave A W541 / ADR-885: OBJECT_REF_* opcodes peeled.
            // Wave A W544 / ADR-888: ObjectRef Retain/Release AOT MirOp stubs peeled.
            _ => None,
        };
        let import_name = import.name().to_string();
        let Some(op_code) = op_code else {
            continue;
        };

        match import.name() {
            aot::LOAD_GLOBAL | aot::ARRAY_BUILD | aot::DICT_BUILD | aot::SET_BUILD | aot::CALL_HOST
            | aot::SIMD_LOAD | aot::SIMD_STORE | aot::SIMD_SHUFFLE => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, a: i32| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let rt = &mut caller.data_mut().rt;
                            let result = rt
                                .exec(op_code, a, 0, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"));
                            // Wave A W18 / ADR-362: CallHost under try → pending_catch → guest dispatch.
                            if op_code == op::CALL_HOST {
                                let catch = rt.take_catch_pc();
                                if catch >= 0 {
                                    return -(catch + 2);
                                }
                            }
                            result
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::STORE_GLOBAL | aot::STORE_LOCAL | aot::SET_FN_INDEX | aot::PUSH_RT_I32 | aot::PUSH_RT_BOOL => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, a: i32| {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let _ = caller.data_mut().rt.exec(op_code, a, 0, 0, memory);
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::PUSH_RT_NIL => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>| {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let _ = caller.data_mut().rt.exec(op_code, 0, 0, 0, memory);
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::LOAD_LOCAL => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, a: i32| {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let _ = caller.data_mut().rt.exec(op_code, a, 0, 0, memory);
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::CONST_STR => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, off: i32, len: i32| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            caller
                                .data_mut()
                                .rt
                                .exec(op_code, off, len, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::CONCAT_STR => {
                // Phase 101: ignore guest i32 placeholders; RT stack order is MIR [left,right]
                // (STORE_LOCAL must consume ConstStr into locals so LoadLocal re-pushes right).
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, _left: i32, _right: i32| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            caller
                                .data_mut()
                                .rt
                                .exec(op::CONCAT_STR, 0, 0, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::TO_STR => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, _v: i32| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            caller
                                .data_mut()
                                .rt
                                .exec(op::TO_STR, 0, 0, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::MAKE_CLOSURE | aot::AWAIT_SUSPEND | aot::BEGIN_TRY | aot::BEGIN_FN | aot::CALL_FN => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, a: i32, b: i32| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            caller
                                .data_mut()
                                .rt
                                .exec(op_code, a, b, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            // ADR-631: Actor*/TaskSpawn name offsets address MirRuntime.string_pool, not wasm memory.
            aot::ACTOR_CALL | aot::ACTOR_SEND | aot::TASK_SPAWN => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, a: i32, b: i32| -> i32 {
                            let mut owned = Vec::new();
                            caller
                                .data_mut()
                                .rt
                                .exec(op_code, a, b, 0, &mut owned)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::PRINT => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, _val: i32| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let rt = &mut caller.data_mut().rt;
                            // Phase 100: guest ConstStr/ConcatStr leave the value on the RT stack;
                            // do not overwrite with the wasm i32 handle (often 0).
                            rt.exec(op::PRINT, 0, 0, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"));
                            0
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::MAKE_ITERATOR | aot::ITERATOR_NEXT | aot::ARRAY_GET | aot::ARRAY_SET
            | aot::OBJECT_GET | aot::OBJECT_SET | aot::VALUE_EQ | aot::VALUE_NE | aot::REF_EQ
            | aot::REF_NE | aot::ARRAY_SORTED | aot::CALL_CLOSURE | aot::AWAIT_POLL
            | aot::GET_RETURN
            | aot::ACTOR_REGISTER | aot::TASK_JOIN | aot::CATCH | aot::THROW | aot::END_TRY
            | aot::SIMD_ADD | aot::SIMD_SUB | aot::SIMD_MUL | aot::SIMD_DIV | aot::SIMD_EQ
            | aot::SIMD_LT | aot::SIMD_GT | aot::SIMD_NE | aot::SIMD_LE | aot::SIMD_GE
            | aot::SIMD_AND | aot::SIMD_OR | aot::SIMD_XOR | aot::SIMD_NOT | aot::SIMD_SHL
            | aot::SIMD_SHR | aot::SIMD_MIN | aot::SIMD_MAX | aot::SIMD_F32_ADD
            | aot::SIMD_F32_SUB | aot::SIMD_F32_MUL | aot::SIMD_F32_DIV | aot::SIMD_F64_ADD
            | aot::SIMD_F64_SUB | aot::SIMD_F64_MUL | aot::SIMD_F64_DIV
            | aot::SIMD_I16_ADD | aot::SIMD_I64_ADD => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>| -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            caller
                                .data_mut()
                                .rt
                                .exec(op_code, 0, 0, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::SIMD_WASM_SPILL | aot::SIMD_WASM_FILL | aot::PUSH_RT_NUMERIC => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>, slot: i32, scratch: i32| {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let _ = caller
                                .data_mut()
                                .rt
                                .exec(op_code, slot, scratch, 0, memory);
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            aot::CONST_NUMERIC128 => {
                linker
                    .func_wrap(
                        host::MODULE,
                        import.name(),
                        move |mut caller: Caller<'_, WasmStore>,
                              kind: i32,
                              lo0: i32,
                              lo1: i32,
                              hi0: i32,
                              hi1: i32|
                              -> i32 {
                            let mut owned = Vec::new();
                            let memory: &mut [u8] = if let Some(mem) = caller
                                .get_export("memory")
                                .and_then(|e| e.into_memory())
                            {
                                let len = mem.data_size(&caller) as usize;
                                let ptr = mem.data_ptr(&caller);
                                unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                            } else {
                                &mut owned
                            };
                            let rt = &mut caller.data_mut().rt;
                            let _ = rt.exec(op::SET_CONST128_LO, lo0, lo1, 0, memory);
                            let _ = rt.exec(op::SET_CONST128_HI, hi0, hi1, 0, memory);
                            rt.exec(op::CONST_NUMERIC128, kind, 0, 0, memory)
                                .unwrap_or_else(|e| panic!("aot import: {e}"))
                        },
                    )
                    .map_err(|e| {
                        SyxDiagnostic::error(format!("Failed to link `{import_name}`: {e}"))
                    })?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn link_syx_host_imports(linker: &mut Linker<WasmStore>, wasm_module: &Module) -> SyxResult<()> {
    use syx_mir_run::op;
    use syx_stdlib::host_fn_index_by_name;

    for import in wasm_module.imports() {
        if import.module() != host::MODULE || import.name() == host::MIR_EXEC {
            continue;
        }
        if import.name().starts_with("__syx_aot_") {
            continue;
        }
        let host_idx = host_fn_index_by_name(import.name()).ok_or_else(|| {
            SyxDiagnostic::error(format!("unknown WASM host import `{}`", import.name()))
        })?;
        linker
            .func_wrap(
                host::MODULE,
                import.name(),
                move |mut caller: Caller<'_, WasmStore>| -> i32 {
                    let mut owned = Vec::new();
                    let memory: &mut [u8] = if let Some(mem) = caller
                        .get_export("memory")
                        .and_then(|e| e.into_memory())
                    {
                        let len = mem.data_size(&caller) as usize;
                        let ptr = mem.data_ptr(&caller);
                        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
                    } else {
                        &mut owned
                    };
                    let rt = &mut caller.data_mut().rt;
                    match rt.exec(op::CALL_HOST, host_idx as i32, 0, 0, memory) {
                        Ok(_) => {
                            let catch = rt.take_catch_pc();
                            if catch >= 0 {
                                return -(catch + 2);
                            }
                            -1
                        }
                        Err(e) => {
                            let catch = rt.take_catch_pc();
                            if catch >= 0 {
                                return -(catch + 2);
                            }
                            caller.data_mut().host_fail = Some(e);
                            -1
                        }
                    }
                },
            )
            .map_err(|e| {
                SyxDiagnostic::error(format!(
                    "Failed to link host import `{}`: {e}",
                    import.name()
                ))
            })?;
    }
    Ok(())
}

// Wave A W534 / ADR-878: `link_syx_rt_object_abi` retired with handle C ABI
// true-delete. No wasm-emit consumers of syx_rt.object_retain/release/drop.

fn link_wasm_gc_stub(linker: &mut Linker<WasmStore>) -> SyxResult<()> {
    linker
        .func_wrap(
            "syx_gc",
            "alloc",
            |_caller: Caller<'_, WasmStore>, size: i32| -> i32 {
                if wasi_gc_phase33_enabled() {
                    gc_guest_wire_mark();
                    gc_relocating_mark_active();
                    gc_remembered_set_mark(size);
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase32_enabled() {
                    gc_guest_wire_mark();
                    gc_relocating_mark_active();
                    gc_remembered_set_mark(size);
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase31_enabled() {
                    gc_guest_wire_mark();
                    gc_generational_young_alloc(size)
                } else if wasi_gc_phase30_enabled() {
                    gc_guest_wire_mark();
                    gc_relocating_mark_active();
                    gc_write_barrier_mark(size);
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase29_enabled() {
                    gc_guest_wire_mark();
                    gc_relocating_mark_active();
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase28_enabled() {
                    gc_guest_wire_mark();
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase27_enabled() {
                    gc_guest_wire_mark();
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase26_enabled() {
                    gc_guest_wire_mark();
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase25_enabled() {
                    gc_unified_heap_alloc(size)
                } else if wasi_gc_phase24_enabled() || wasi_gc_phase23_enabled() {
                    gc_alloc_bump(size)
                } else if wasi_gc_phase22_enabled() {
                    gc_alloc_noop(size)
                } else {
                    size.max(0)
                }
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link syx_gc.alloc: {e}")))?;
    linker
        .func_wrap(
            "syx_gc",
            "collect",
            |_caller: Caller<'_, WasmStore>| -> i32 {
                if wasi_gc_phase33_enabled() {
                    gc_guest_wire_mark();
                    gc_remembered_set_prod_collect()
                } else if wasi_gc_phase32_enabled() {
                    gc_guest_wire_mark();
                    gc_remembered_set_prod_collect()
                } else if wasi_gc_phase31_enabled() {
                    gc_guest_wire_mark();
                    gc_generational_prod_collect()
                } else if wasi_gc_phase30_enabled() {
                    gc_guest_wire_mark();
                    gc_relocating_prod_collect()
                } else if wasi_gc_phase29_enabled() {
                    gc_guest_wire_mark();
                    gc_relocating_collect()
                } else if wasi_gc_phase28_enabled() {
                    gc_guest_wire_mark();
                    let budget = gc_sweep_incremental_budget();
                    gc_unified_heap_collect() + budget
                } else if wasi_gc_phase27_enabled() {
                    gc_guest_wire_mark();
                    gc_unified_heap_collect()
                } else if wasi_gc_phase26_enabled() {
                    gc_guest_wire_mark();
                    gc_unified_heap_collect()
                } else if wasi_gc_phase25_enabled() {
                    gc_unified_heap_collect()
                } else if wasi_gc_phase24_enabled() {
                    gc_collect_incremental_sweep()
                } else if wasi_gc_phase23_enabled() {
                    gc_collect_mark_smoke()
                } else {
                    0
                }
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link syx_gc.collect: {e}")))?;
    Ok(())
}

fn link_wasm_threads_mvp(linker: &mut Linker<WasmStore>) -> SyxResult<()> {
    linker
        .func_wrap(
            "syx_threads",
            "sab_atomic_add",
            |_caller: Caller<'_, WasmStore>, ptr: i32, index: i32, delta: i32| -> i32 {
                let _ = ptr;
                let _ = index;
                delta
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link sab_atomic_add: {e}")))?;
    linker
        .func_wrap("syx_threads", "sab_len", |_caller: Caller<'_, WasmStore>, ptr: i32| -> i32 {
            let _ = ptr;
            1
        })
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link sab_len: {e}")))?;
    Ok(())
}

fn run_mir_entry_capture(module: &MirModule, entry: &str) -> SyxResult<WasmRunResult> {
    let mir = Arc::new(module.clone());
    let (mut rt, ctx) = init_wasm_rt(mir);
    let module = ctx
        .module()
        .expect("mir installed");
    let main = module
        .functions
        .iter()
        .find(|f| f.name == entry)
        .ok_or_else(|| SyxDiagnostic::error(format!("No MIR function '{entry}'")))?;
    syx_vm::set_mir_closure_executor(syx_mir_run::execute_mir_closure);
    syx_vm::set_mir_closure_preferred(true);
    syx_mir_run::pin_active_mir_runtime(&mut rt);
    let code = syx_mir_run::run_mir_function(main, &mut rt, &mut [])?;
    syx_mir_run::unpin_active_mir_runtime();
    syx_vm::clear_mir_closure_executor();
    Ok(WasmRunResult {
        exit_code: code,
        printed: collect_printed(&ctx, &rt),
    })
}

fn uses_interpreter_string_pool(op_code: u32) -> bool {
    matches!(
        op_code,
        syx_mir_run::op::ACTOR_CALL
            | syx_mir_run::op::ACTOR_SEND
            | syx_mir_run::op::ACTOR_REGISTER
            | syx_mir_run::op::TASK_SPAWN
            | syx_mir_run::op::CALL_FN
    )
}

fn mir_exec_dispatch(caller: &mut Caller<'_, WasmStore>, op: i32, a: i32, b: i32) -> i32 {
    let op_code = op as u32;
    let mut owned = Vec::new();
    let memory_mut: &mut [u8] = if matches!(
        op_code,
        syx_mir_run::op::SIMD_WASM_SPILL | syx_mir_run::op::SIMD_WASM_FILL
    ) {
        if let Some(mem) = caller
            .get_export("memory")
            .and_then(|e| e.into_memory())
        {
            let len = mem.data_size(&mut *caller) as usize;
            let ptr = mem.data_ptr(&mut *caller);
            unsafe { std::slice::from_raw_parts_mut(ptr, len) }
        } else {
            &mut owned
        }
    } else if uses_interpreter_string_pool(op_code) {
        &mut owned
    } else if let Some(mem) = caller
        .get_export("memory")
        .and_then(|e| e.into_memory())
    {
        let len = mem.data_size(&mut *caller) as usize;
        let ptr = mem.data_ptr(&mut *caller);
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    } else {
        &mut owned
    };
    let rt = &mut caller.data_mut().rt;
    let result = rt
        .exec(op_code, a, b, 0, memory_mut)
        .unwrap_or_else(|e| panic!("{e}"));
    if op_code == syx_mir_run::op::GET_RETURN
        || op_code == syx_mir_run::op::POP_BOOL
        || op_code == syx_mir_run::op::PEEK_IS_NIL
    {
        return result;
    }
    let catch = rt.take_catch_pc();
    if catch >= 0 {
        return -(catch + 2);
    }
    -1
}

pub fn run_wasm(wasm: &[u8], entry: &str) -> SyxResult<i32> {
    run_wasm_with_module(wasm, entry, None)
}

fn wasm_engine() -> Engine {
    let mut config = Config::new();
    config.wasm_simd(true);
    Engine::new(&config).expect("wasmtime engine with simd")
}

pub fn run_wasm_capture(
    wasm: &[u8],
    entry: &str,
    module: Option<&MirModule>,
) -> SyxResult<WasmRunResult> {
    init_wasm_guest_plugins();
    if let Some(mir) = module {
        if wasm_mir_fallback_enabled() && mir_uses_actors(mir) {
            return run_mir_entry_capture(mir, entry);
        }
    }
    let engine = wasm_engine();
    let wasm_module = Module::new(&engine, wasm).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to load WASM module: {e:?}"))
    })?;
    let (rt, ctx) = if let Some(mir) = module {
        init_wasm_rt(Arc::new(mir.clone()))
    } else {
        (MirRuntime::default(), WasmRunContext::new())
    };
    let wasi_host = if wasi_enabled() {
        Some(Arc::new(Mutex::new(WasiHost::new()?)))
    } else {
        None
    };
    let mut store = Store::new(&engine, WasmStore {
        rt,
        ctx: Arc::clone(&ctx),
        wasi: wasi_host,
        host_fail: None,
    });
    let mut linker = Linker::new(&engine);

    linker
        .func_wrap(
            host::MODULE,
            host::MIR_EXEC,
            |mut caller: Caller<'_, WasmStore>, op: i32, a: i32, b: i32| -> i32 {
                mir_exec_dispatch(&mut caller, op, a, b)
            },
        )
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link mir_exec: {e}")))?;

    if wasi_enabled() {
        link_wasi_preview1(&mut linker)?;
    }

    if wasm_threads_mvp_enabled() {
        link_wasm_threads_mvp(&mut linker)?;
    }

    link_syx_aot_imports(&mut linker, &wasm_module)?;

    link_syx_host_imports(&mut linker, &wasm_module)?;

    // Wave A W534 / ADR-878: handle C ABI link retired (no emit consumers).

    let instance = linker
        .instantiate(&mut store, &wasm_module)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to instantiate WASM module: {e}")))?;
    let func = instance
        .get_typed_func::<(), i32>(&mut store, entry)
        .map_err(|e| {
            SyxDiagnostic::error(format!(
                "Failed to resolve exported function '{entry}': {e}"
            ))
        })?;
    syx_vm::set_mir_closure_executor(syx_mir_run::execute_mir_closure);
    syx_vm::set_mir_closure_preferred(true);
    syx_mir_run::pin_active_mir_runtime(&mut store.data_mut().rt);
    let exit_code = func.call(&mut store, ()).map_err(|e| {
        SyxDiagnostic::error(format!("WASM trap while invoking '{entry}': {e}"))
    })?;
    syx_mir_run::unpin_active_mir_runtime();
    syx_vm::clear_mir_closure_executor();
    if let Some(e) = store.data_mut().host_fail.take() {
        return Err(e);
    }
    let printed = collect_printed(&ctx, &store.data().rt);
    Ok(WasmRunResult { exit_code, printed })
}

pub fn run_wasm_with_module(
    wasm: &[u8],
    entry: &str,
    module: Option<&MirModule>,
) -> SyxResult<i32> {
    run_wasm_capture(wasm, entry, module).map(|r| r.exit_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_mir::{MirFunction, MirInstr, MirModule, MirOp};
    use syx_wasm_emit::emit_wasm;

    const ENUM_ASSOC_FIXTURE: &str = r#"
enum Result {
    case ok(Int)
    case err(Int)
}

func main() {
    let r = Result.ok(42)
    switch (r) {
    case .ok(let n) {
        print(n)
    }
    case .err(let e) {
        print(e)
    }
    }
    return 0
}
"#;

    fn simple_main(instrs: Vec<MirInstr>) -> MirModule {
        MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![syx_mir::MirBlock {
                    id: 0,
                    instructions: instrs,
                    successors: vec![],
                }],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
        }
    }

    #[test]
    fn actor_main_via_wasm_style_exec_sequence() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_module, optimize};
        use syx_mir_run::op;
        use syx_parser::Parser;

        let source = SourceFile::new(
            "t.sy",
            include_str!("../../syx-driver/tests/fixtures/parity/actor_isolation.sy"),
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let ctx = install_mir_module(Arc::new(mir.clone()));
        let mut rt = MirRuntime::default();
        rt.install_functions(mir.functions.clone());
        rt.set_runtime(wasm_vm_runtime_for(Arc::clone(&ctx)));
        syx_mir_run::install_module_string_pool(&mut rt, &mir);
        let main_idx = mir.functions.iter().position(|f| f.name == "main").unwrap();
        let main = &mir.functions[main_idx];
        rt.begin_function(main.local_count);
        for block in &main.blocks {
            for instr in &block.instructions {
                match &instr.op {
                    MirOp::Return => break,
                    MirOp::Call(name) => {
                        let idx = mir
                            .functions
                            .iter()
                            .position(|f| f.name == *name)
                            .unwrap();
                        rt.exec(op::CALL_FN, idx as i32, 0, 0, &mut [])
                            .expect("call");
                        if name.as_str() == "Counter_init" {
                            let actor = rt.stack.last().unwrap();
                            assert!(
                                syx_vm::actor_id_from_value(actor).is_some(),
                                "after init stack={:?} locals={:?}",
                                rt.stack,
                                rt.locals
                            );
                        }
                    }
                    MirOp::StoreLocal(i) => {
                        rt.exec_mir_op(&instr.op, &mut []).expect("store");
                        if *i == 0 {
                            let local = rt.locals.get(0).unwrap();
                            assert!(
                                syx_vm::actor_id_from_value(local).is_some(),
                                "after store local 0: {local:?}"
                            );
                        }
                    }
                    other => rt.exec_mir_op(other, &mut []).expect("op"),
                }
            }
        }
        let printed = ctx.printed_snapshot();
        assert!(printed.iter().any(|l| l.contains('2')), "{printed:?}");
    }

    #[test]
    fn actor_main_run_mir_function_succeeds() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_module, optimize};
        use syx_parser::Parser;

        let source = SourceFile::new(
            "t.sy",
            include_str!("../../syx-driver/tests/fixtures/parity/actor_isolation.sy"),
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let ctx = install_mir_module(Arc::new(mir.clone()));
        let mut rt = MirRuntime::default();
        rt.install_functions(mir.functions.clone());
        rt.set_runtime(wasm_vm_runtime_for(Arc::clone(&ctx)));
        syx_mir_run::install_module_string_pool(&mut rt, &mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let code = syx_mir_run::run_mir_function(main, &mut rt, &mut []).unwrap();
        assert_eq!(code, 0);
        let printed = ctx.printed_snapshot();
        assert!(printed.iter().any(|l| l.contains('2')), "{printed:?}");
    }

    #[test]
    fn enum_assoc_run_mir_function_succeeds() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_enum_shells, lower_module, merge_mir_functions, optimize};
        use syx_parser::Parser;

        let source = SourceFile::new("t.sy", ENUM_ASSOC_FIXTURE);
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        merge_mir_functions(&mut mir, lower_enum_shells(&module));
        optimize(&mut mir);
        let ctx = install_mir_module(Arc::new(mir.clone()));
        let mut rt = MirRuntime::default();
        rt.install_functions(mir.functions.clone());
        rt.set_runtime(wasm_vm_runtime_for(Arc::clone(&ctx)));
        syx_mir_run::install_module_string_pool(&mut rt, &mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let code = syx_mir_run::run_mir_function(main, &mut rt, &mut []).unwrap();
        assert_eq!(code, 0);
        let printed = ctx.printed_snapshot();
        assert!(printed.iter().any(|l| l == "42"), "{printed:?}");
    }

    #[test]
    fn enum_assoc_wasm_capture_succeeds() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_enum_shells, lower_module, merge_mir_functions, optimize};
        use syx_parser::Parser;

        let source = SourceFile::new("t.sy", ENUM_ASSOC_FIXTURE);
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        merge_mir_functions(&mut mir, lower_enum_shells(&module));
        optimize(&mut mir);
        let wasm = emit_wasm(&mir).unwrap();
        let result = run_wasm_capture(&wasm, "main", Some(&mir)).unwrap();
        assert_eq!(result.exit_code, 0);
        assert!(result.printed.iter().any(|l| l == "42"), "{:?}", result.printed);
    }

    #[test]
    fn counter_init_registers_actor_via_call_fn() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_module, optimize};
        use syx_mir_run::op;
        use syx_parser::Parser;
        use syx_vm::actor_id_from_value;

        let source = SourceFile::new(
            "t.sy",
            include_str!("../../syx-driver/tests/fixtures/parity/actor_isolation.sy"),
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let ctx = install_mir_module(Arc::new(mir.clone()));
        let mut rt = MirRuntime::default();
        rt.install_functions(mir.functions.clone());
        rt.set_runtime(wasm_vm_runtime_for(Arc::clone(&ctx)));
        syx_mir_run::install_module_string_pool(&mut rt, &mir);
        let idx = mir
            .functions
            .iter()
            .position(|f| f.name == "Counter_init")
            .expect("Counter_init");
        rt.begin_function(1);
        rt.exec_mir_op(
            &MirOp::ConstNumeric {
                kind: 3,
                bits: 0,
            },
            &mut [],
        )
        .expect("const");
        rt.call_fn(idx, &mut []).expect("init");
        let actor = rt.stack.last().expect("actor on stack");
        assert!(
            actor_id_from_value(actor).is_some(),
            "actor={actor:?} locals={:?}",
            rt.locals
        );
    }

    #[test]
    fn runs_actor_isolation_bytecode() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_module, optimize};
        use syx_parser::Parser;

        let source = SourceFile::new(
            "t.sy",
            include_str!("../../syx-driver/tests/fixtures/parity/actor_isolation.sy"),
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let wasm = emit_wasm(&mir).unwrap();
        let result = run_wasm_capture(&wasm, "main", Some(&mir)).unwrap();
        assert_eq!(result.exit_code, 0);
        assert!(
            result.printed.iter().any(|l| l.contains('2')),
            "printed={:?}",
            result.printed
        );
    }

    #[test]
    fn runs_emitted_main_returning_42() {
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(42),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        assert_eq!(run_wasm_with_module(&wasm, "main", Some(&module)).unwrap(), 42);
    }

    #[test]
    fn runs_string_split_method_call() {
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstStr("a,b,c".into()),
                span: None,
            },
            MirInstr {
                dest: Some(1),
                op: MirOp::ConstStr(",".into()),
                span: None,
            },
            MirInstr {
                dest: Some(2),
                op: MirOp::CallHost(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Print,
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        run_wasm_with_module(&wasm, "main", Some(&module)).unwrap();
    }

    #[test]
    fn runs_split_for_in_cfg() {
        use syx_diagnostics::SourceFile;
        use syx_mir::{lower_module, optimize};
        use syx_parser::Parser;

        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let parts = "a,b,c".split(",")
    for item in parts {
        print(item)
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);

        let wasm = emit_wasm(&mir).unwrap();
        run_wasm_with_module(&wasm, "main", Some(&mir)).unwrap();
    }

    #[test]
    fn runs_for_in_with_cfg() {
        use syx_mir::MirBlock;
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 2,
                entry: 0,
                blocks: vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![
                            MirInstr {
                                dest: Some(0),
                                op: MirOp::ConstI32(1),
                                span: None,
                            },
                            MirInstr {
                                dest: Some(1),
                                op: MirOp::ConstI32(2),
                                span: None,
                            },
                            MirInstr {
                                dest: Some(2),
                                op: MirOp::ConstI32(3),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::ArrayBuild(3),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::MakeIterator,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreLocal(0),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(1),
                                span: None,
                            },
                        ],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![
                            MirInstr {
                                dest: None,
                                op: MirOp::LoadLocal(0),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::IteratorNext,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::JumpIfFalse(3),
                                span: None,
                            },
                        ],
                        successors: vec![2, 3],
                    },
                    MirBlock {
                        id: 2,
                        instructions: vec![
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreLocal(1),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreLocal(0),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::LoadLocal(1),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Print,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(1),
                                span: None,
                            },
                        ],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 3,
                        instructions: vec![MirInstr {
                            dest: None,
                            op: MirOp::Return,
                            span: None,
                        }],
                        successors: vec![],
                    },
                ],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let wasm = emit_wasm(&module).unwrap();
        run_wasm_with_module(&wasm, "main", Some(&module)).unwrap();
    }

    #[test]
    fn p19_wasi_clock_time_get_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn p20_wasi_random_get_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn p20_wasi_poll_sock_environ_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        assert!(!wasm.is_empty());
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn p18_wasi_environ_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        assert!(!wasm.is_empty());
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn wasi_fd_links_and_run_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn p21_wasm_threads_mvp_smoke() {
        let buf = atomics::SharedAtomicsBuffer::new(2);
        assert_eq!(buf.atomic_add_i32(0, 41), 41);
        assert_eq!(buf.atomic_add_i32(0, 1), 42);
    }

    #[test]
    fn wasi_path_open_read_write_roundtrip() {
        let host = WasiHost::new().expect("wasi host");
        let path = host.temp_path().join("hello.txt");
        std::fs::write(&path, b"phase10").expect("seed");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        let mut buf = [0u8; 8];
        let n = file.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"phase10");
        file.write_all(b"-wasi").unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "phase10-wasi");
    }

    #[test]
    fn p22_wasi_gc_phase22_alloc_noop() {
        std::env::set_var("SYX_WASI_GC_PHASE22", "1");
        assert!(wasi_gc_phase22_enabled());
        assert_eq!(gc_alloc_noop(128), 0);
        assert_eq!(gc_alloc_noop(-1), 0);
        std::env::remove_var("SYX_WASI_GC_PHASE22");
    }

    #[test]
    fn p22_wasi_gc_phase22_run_smoke() {
        std::env::set_var("SYX_WASI", "1");
        std::env::set_var("SYX_WASI_GC_PHASE22", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        std::env::remove_var("SYX_WASI_GC_PHASE22");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn p21_wasi_gc_stub_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        let code = run_wasm_with_module(&wasm, "main", Some(&module));
        std::env::remove_var("SYX_WASI");
        assert_eq!(code.unwrap(), 0);
    }

    #[test]
    fn p21_wasi_fd_close_smoke() {
        std::env::set_var("SYX_WASI", "1");
        let module = simple_main(vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            },
        ]);
        let wasm = emit_wasm(&module).unwrap();
        assert!(!wasm.is_empty());
        std::env::remove_var("SYX_WASI");
    }
}
