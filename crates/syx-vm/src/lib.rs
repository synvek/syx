use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

use smol_str::SmolStr;

#[cfg(feature = "interpreter")]
use syx_codegen::{CompiledProgram, FunctionUnit, Instruction};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_stdlib::{host_fn_id_by_name, method_from_index, HostFnId, MethodId, NumericKind};

pub mod actor_mailbox;
mod async_iter_hosts;
mod blocking_execute;
mod channel_hosts;
mod grapheme;
#[cfg(feature = "interpreter")]
mod gui_invoke;
mod hosts;
#[cfg(feature = "http")]
mod http;
#[cfg(not(feature = "http"))]
#[path = "wasm_stubs/http.rs"]
mod http;
mod os_hosts;
#[cfg(feature = "interpreter")]
mod program_ctx;
mod script_hosts;
mod wasm_threads;
#[cfg(not(target_arch = "wasm32"))]
mod grpc_wire;
#[cfg(target_arch = "wasm32")]
#[path = "wasm_stubs/grpc_wire.rs"]
mod grpc_wire;
mod ws;
#[cfg(not(target_arch = "wasm32"))]
mod ws_rfc6455;
#[cfg(target_arch = "wasm32")]
#[path = "wasm_stubs/ws_rfc6455.rs"]
mod ws_rfc6455;
pub use grapheme::{grapheme_at, graphemes_vec, try_grapheme_at};
#[cfg(feature = "gui-dlopen")]
mod gui_dlopen;
mod stdlib_phase120_hosts;
mod stdlib_phase83_hosts;
mod stdlib_phase84_hosts;
mod stdlib_phase85_hosts;
mod stdlib_phase86_hosts;
mod stdlib_phase87_hosts;
mod stdlib_phase88_hosts;
#[cfg(not(target_arch = "wasm32"))]
mod stdlib_phase89_hosts;
#[cfg(target_arch = "wasm32")]
#[path = "wasm_stubs/stdlib_phase89_hosts.rs"]
mod stdlib_phase89_hosts;
mod stdlib_phase90_hosts;
mod stdlib_phase91_hosts;
mod stdlib_phase92_hosts;
mod stdlib_phase93_hosts;
mod stdlib_phase94_hosts;
mod stdlib_phase95_hosts;
mod stdlib_phase96_hosts;
mod stdlib_phase97_hosts;
mod stdlib_phase98_hosts;
#[cfg(feature = "gui-synui")]
mod synui_hosts;
#[cfg(not(feature = "gui-synui"))]
#[path = "synui_hosts_stub.rs"]
mod synui_hosts;
pub use stdlib_phase84_hosts::{
    child_pipe_force_close, child_pipe_live_count, clear_registered_tests, process_child_force_close,
    process_child_live_count, store_registered_test_outcomes, take_registered_test_outcomes,
    RegisteredTestOutcome,
};
pub use stdlib_phase89_hosts::{reset_rusqlite_store, sql_conn_force_close, sql_conn_live_count};
pub use stdlib_phase94_hosts::{
    gc_churn_stdout_lines, host_gc_churn_stdout, host_gui_winit_canvas_draw_phase94,
    host_gui_winit_event_pump_phase94, host_gui_winit_run_loop_once_phase94,
    host_orm_find_where_dict_rusqlite, orm_executed_sql_count, orm_last_executed_sql,
    reset_phase94_state,
};
pub use stdlib_phase95_hosts::reset_phase95_state;
pub use stdlib_phase96_hosts::reset_phase96_state;
pub use stdlib_phase97_hosts::{
    ensure_gui_loaded, gui_backend_name, gui_caret_metric_px, gui_caret_supported,
    gui_cjk_subset_char_count, gui_colr_grad_used, gui_colr_radial_or_sweep_used,
    gui_colr_radial_used, gui_colr_sweep_used, gui_dlopen_loaded, gui_doctor_coverage_fingerprint,
    gui_doctor_engine_tag, gui_doctor_present_mode, gui_doctor_widgets, gui_dylib_path_report,
    gui_egui_text_suppressed_used, gui_emoji_atlas_hit, gui_emoji_engine, gui_emoji_fingerprint,
    gui_fallback_hits, gui_font_fallback_fingerprint, gui_font_mgr_bundled_used,
    gui_font_mgr_fallback_used, gui_font_mgr_os_used, gui_glyph_coverage_percent,
    gui_harfbuzz_fingerprint, gui_hb_paint_path_used, gui_ime_commit_supported,
    gui_ime_preedit_fingerprint, gui_ime_preedit_supported, gui_no_double_paint_probe,
    gui_paint_shaped, gui_paragraph_paint_used, gui_sample_center_pixel, gui_shaped,
    gui_skia_glyph_fingerprint, gui_skia_glyph_used, gui_ttf_subset_fingerprint,
    gui_vertical_ascii_ink_used, gui_vertical_hb_vertical_used, gui_vertical_ink_used,
    gui_vertical_rl_fingerprint, gui_vertical_rl_layout_used, gui_visible_requested,
    gui_wgpu_composite_fingerprint, gui_wgpu_composite_used, gui_wgpu_present_used,
    host_codable_decode_l14, host_codable_encode_l14, host_codable_l14_register_migration,
    host_codable_l14_register_rename, host_gc_churn_heap, host_gui_button_create,
    host_gui_button_take_clicked, host_gui_click_inject, host_gui_key_inject,
    host_gui_label_create, host_gui_layout_configure, host_gui_layout_create, host_gui_on_click,
    host_gui_pixel_read_phase97, host_gui_text_field_create, host_gui_widget_get_text,
    host_gui_widget_set_text, host_gui_winit_canvas_draw_phase97,
    host_gui_winit_event_pump_phase97, host_gui_winit_run_loop_once_phase97,
    host_gui_winit_window_create, host_orm_find_where_dict_prepare, orm_prepared_row_count,
    orm_prepared_rows, require_gui_dylib_if_visible, reset_phase97_state, visible_window_count,
};
pub use stdlib_phase98_hosts::reset_phase98_state;
pub use synui_hosts::{
    reset_synui_state, synui_fingerprint, SYNUI_HOST_BRIDGE_FINGERPRINT,
    SYNUI_PATH_DEP_FINGERPRINT, SYNUI_UI_PACKAGE_FINGERPRINT,
    SYNUI_V10_SCROLLVIEW_SIGNAL_BIND_FINGERPRINT, SYNUI_V10_WEB_MOBILE_LAYOUT_DEEPEN_FINGERPRINT,
    SYNUI_V10_WIDGET_STACK_FINGERPRINT, SYNUI_V11_TAB_SIGNAL_BIND_FINGERPRINT,
    SYNUI_V11_WEB_MOBILE_CONTROLS_DEEPEN_FINGERPRINT, SYNUI_V11_WIDGET_TABVIEW_FINGERPRINT,
    SYNUI_V12_NAV_SIGNAL_BIND_FINGERPRINT, SYNUI_V12_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT,
    SYNUI_V12_WIDGET_NAVIGATION_FINGERPRINT, SYNUI_V2_CI_SIBLING_CHECKOUT_FINGERPRINT,
    SYNUI_V2_DESKTOP_RUN_FINGERPRINT, SYNUI_V2_SIGNAL_CALLBACK_FINGERPRINT,
    SYNUI_V2_WIDGET_EXPAND_FINGERPRINT, SYNUI_V3_DESKTOP_RUN_FINGERPRINT,
    SYNUI_V3_WIDGET_CHECKBOX_SLIDER_FINGERPRINT, SYNUI_V4_DESKTOP_ONCLICK_WIRE_FINGERPRINT,
    SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT, SYNUI_V4_WIDGET_SWITCH_RADIO_FINGERPRINT,
    SYNUI_V5_MOBILE_SURFACE_UNRETIRE_FINGERPRINT, SYNUI_V5_TOGGLE_RADIO_BIND_FINGERPRINT,
    SYNUI_V5_WIDGET_PROGRESS_PICKER_FINGERPRINT, SYNUI_V6_DESKTOP_RUN_LAYOUT_GA_FINGERPRINT,
    SYNUI_V6_PROGRESS_NUMBER_BIND_FINGERPRINT, SYNUI_V6_WIDGET_TEXTAREA_LIST_FINGERPRINT,
    SYNUI_V7_PICKER_SIGNAL_BIND_FINGERPRINT, SYNUI_V7_WEB_LANGUAGE_SURFACE_MVP_FINGERPRINT,
    SYNUI_V7_WIDGET_IMAGE_SCROLL_FINGERPRINT, SYNUI_V8_TEXTAREA_LIST_BIND_FINGERPRINT,
    SYNUI_V8_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT, SYNUI_V8_WIDGET_SPACER_FINGERPRINT,
    SYNUI_V9_IMAGE_SIGNAL_BIND_FINGERPRINT, SYNUI_V9_WEB_MOBILE_CONTROLS_DEEPEN_FINGERPRINT,
    SYNUI_V9_WIDGET_DIVIDER_FINGERPRINT,
};
#[cfg(feature = "gui-winit")]
mod gui_winit_real;
pub use stdlib_phase93_hosts::{
    gc_churn_event_count, host_gc_churn_event, host_gui_touch_inject,
    host_orm_find_where_dict_bound, orm_bound_value_count, reset_phase93_state,
};
mod task_hosts;
pub use blocking_execute::{execute_on_blocking_pool, register_blocking_execute};
pub use channel_hosts::try_poll_channel_recv;
pub use channel_hosts::{
    host_channel_create, host_channel_select_mixed_await, mixed_await_select_timeout_ms,
    register_mixed_await_cancel_probe, reset_select_fair_cursor,
    select_deadline_strict_fingerprint, select_deadline_unify_fingerprint, select_fair_cursor,
    select_fair_fingerprint, select_mixed_await_fingerprint, select_mixed_timeout_fingerprint,
    select_mixed_unified_fingerprint, select_timeout_syntax_fingerprint,
};
#[cfg(feature = "http")]
pub use gui_invoke::register_native_gui_handler;
pub use hosts::dispatch_numeric_host;
pub use hosts::try_poll_file_read;
pub use hosts::try_poll_tcp_read;
pub use hosts::{
    call_array_method_for_mir, call_receiver_method, file_handle_force_close,
    file_handle_live_count, file_stream_force_close, file_stream_live_count,
    file_watch_force_close, file_watch_live_count, host_fn_for, reset_file_handles_for_tests,
    tcp_listener_force_close, tcp_listener_live_count, tcp_stream_force_close,
    tcp_stream_live_count, try_call_array_hof, try_call_array_method_for_mir,
};
pub use hosts::{dispatch_bigint_host, dispatch_collection_host};
pub use http::execute_http_request;
pub use http::http_response;
pub use http::register_native_http_handler;
pub use http::try_poll_http_request;
pub use http::{http_body_stream_force_close, http_body_stream_live_count};
#[cfg(feature = "interpreter")]
pub use program_ctx::{
    clear_active_vm, clear_mir_closure_executor, clear_program_ctx, execute_closure, set_active_vm,
    set_mir_closure_executor, set_mir_closure_preferred, set_program_ctx, with_active_vm,
    with_program_ctx, ProgramCtx,
};
pub use stdlib_phase120_hosts::materialize_lazy_receiver;
pub use syx_runtime_core::{
    combine_value, cooperative_yield, display_numeric, ensure_object_id, finalize_state,
    hash_kind_tag, hash_value, hasher_from_value, hasher_value, is_hasher, make_object_identifier,
    numeric_add, numeric_bitand, numeric_bitnot, numeric_bitor, numeric_bitxor, numeric_cast,
    numeric_cmp, numeric_div, numeric_mod, numeric_mul, numeric_neg, numeric_shl, numeric_shr,
    numeric_sub, platform_int_value, simd_binop, try_numeric_cast, value_debug_description,
    value_description, value_equals, value_from_numeric, value_from_numeric128, value_identical,
    value_kind, value_object_id, HasherState, IteratorState, RuntimeBridge, SimdBinOp, Value,
    VmRuntime, OBJECT_ID_KEY,
};
pub use task_hosts::{
    bridge_task_group_ops, bridge_task_live_count, register_actor_reentrancy_policy_host,
    register_actor_shared_pool_hosts, register_blocking_isolation_hosts,
    register_blocking_pool_sizing_hosts, register_bridge_task_group_ops,
    register_bridge_task_live_count, register_concurrency_control_hosts,
    register_io_adapter_policy_host, register_task_group_cancel_host, register_task_hosts,
    register_task_live_count_host, register_task_local_hosts, register_task_pool_sizing_hosts,
    register_task_set_deadline_ms_host, register_trace_context_providers, BridgeTaskGroupOps,
};
pub use grpc_wire::{
    e02_grpc_interop_first_slice_ok, e02_grpc_loopback_peer_echo_wired_ok,
    e02_grpc_negative_framing_wired_ok, e02_grpc_negative_preface_fail_closed_ok,
    e02_grpc_negative_tests_first_slice_ok, e02_grpc_negative_truncated_data_fail_closed_ok,
    e02_grpc_streaming_negative_framing_wired_ok,
    e02_grpc_streaming_negative_truncated_data_fail_closed_ok,
    e02_streaming_negative_tests_first_slice_ok,
    e02_bidi_interleaved_first_slice_ok, e02_grpc_bidi_interleaved_loopback_ok,
    e02_grpc_bidi_interleaved_wired_ok, e02_grpc_bidi_streaming_first_slice_ok,
    e02_grpc_bidi_streaming_loopback_ok, e02_grpc_bidi_streaming_wired_ok,
    e02_grpc_client_streaming_first_slice_ok,
    e02_grpc_client_streaming_loopback_ok, e02_grpc_client_streaming_wired_ok,
    e02_grpc_server_streaming_loopback_ok, e02_grpc_server_streaming_wired_ok,
    e02_grpc_streaming_first_slice_ok, e02_grpc_tls_loopback_wired_ok,
    e02_grpc_tls_bidi_interleaved_loopback_ok, e02_grpc_tls_bidi_interleaved_wired_ok,
    e02_grpc_tls_bidi_streaming_loopback_ok, e02_grpc_tls_bidi_streaming_wired_ok,
    e02_grpc_tls_client_streaming_loopback_ok, e02_grpc_tls_client_streaming_wired_ok,
    e02_grpc_tls_server_streaming_loopback_ok, e02_grpc_tls_server_streaming_wired_ok,
    e02_grpc_tls_window_update_loopback_ok,
    e02_grpc_tls_window_update_connection_loopback_ok,
    e02_grpc_tls_window_update_connection_omit_credit_fail_closed_ok,
    e02_grpc_tls_window_update_connection_negative_wired_ok,
    e02_grpc_tls_window_update_connection_wired_ok,
    e02_grpc_tls_window_update_omit_credit_fail_closed_ok,
    e02_grpc_tls_window_update_negative_wired_ok, e02_grpc_tls_window_update_wired_ok,
    e02_grpc_wire_loopback_roundtrip_ok,     e02_grpc_window_update_connection_loopback_ok,
    e02_grpc_window_update_connection_omit_credit_fail_closed_ok,
    e02_grpc_window_update_connection_negative_wired_ok,
    e02_grpc_window_update_connection_wired_ok, e02_grpc_window_update_loopback_ok,
    e02_grpc_window_update_omit_credit_fail_closed_ok,
    e02_grpc_window_update_negative_wired_ok, e02_grpc_window_update_wired_ok,
    e02_grpcs_bidi_interleaved_first_slice_ok, e02_grpcs_bidi_streaming_first_slice_ok,
    e02_grpcs_client_streaming_first_slice_ok, e02_grpcs_first_slice_ok,
    e02_grpcs_streaming_first_slice_ok, e02_grpcs_window_update_connection_first_slice_ok,
    e02_grpcs_window_update_connection_negative_first_slice_ok,
    e02_grpcs_window_update_first_slice_ok,
    e02_grpcs_window_update_negative_first_slice_ok, e02_window_update_connection_first_slice_ok,
    e02_window_update_connection_negative_first_slice_ok,
    e02_window_update_first_slice_ok, e02_window_update_negative_first_slice_ok,
    encode_grpc_message, syx_grpc_real_enabled,
};
pub use ws::{
    e01_rfc6455_interop_first_slice_ok, e01_rfc6455_loopback_roundtrip_ok,
    e01_rfc6455_negative_tests_first_slice_ok, e01_rfc6455_wss_first_slice_ok, host_ws_connect,
    host_ws_send_async, try_poll_ws_request, ws_mock_enabled,
};
#[cfg(feature = "interpreter")]
mod jit;
#[cfg(feature = "interpreter")]
pub use jit::{
    cranelift_jit_tier2_prod_ready, invalidate_all_jit_caches, run_with_jit, JitCompiler,
};

static ACTOR_LOCKS: LazyLock<Mutex<HashMap<i64, Arc<Mutex<()>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_ACTOR_ID: AtomicI64 = AtomicI64::new(1);

fn actor_lock_for(id: i64) -> Arc<Mutex<()>> {
    let mut map = ACTOR_LOCKS.lock().unwrap_or_else(|e| e.into_inner());
    map.entry(id)
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .clone()
}

pub fn actor_id_from_value(v: &Value) -> Option<i64> {
    v.actor_id()
}

fn opaque_unwrap<'a>(v: &'a Value) -> &'a Value {
    if let Value::Object(map) | Value::Dict(map) = v {
        if let Some(payload) = map.get("__opaque_payload") {
            return payload;
        }
    }
    v
}

fn value_to_display(v: &Value) -> String {
    let v = opaque_unwrap(v);
    value_description(v)
}

/// VM GC pin bookkeeping shell.
///
/// Wave A W494 / ADR-838: dead `Gc::collect_mark_sweep` + `Gc::collect` APIs
/// **retired** (zero callers after W491 path retire). Wave A W495 / ADR-839:
/// MemoryPool alloc/pool dead-shell also retired. Wave A W496 / ADR-840:
/// empty `MemoryPool` type stub **retired**; ObjectRef RC storage still present —
/// `storage_cleanup_enabled` remains **false**; ≠ full ObjectRef migrate / ≠ enable.
#[derive(Default)]
pub struct Gc {
    pub pinned: Vec<usize>,
}

pub struct WeakReference {
    pub target: Option<usize>,
}

pub type HostFn = fn(&mut Vec<Value>) -> SyxResult<()>;

pub type PluginHostDispatch = fn(&str, &mut Vec<Value>) -> bool;

static PLUGIN_HOST_DISPATCH: std::sync::OnceLock<PluginHostDispatch> = std::sync::OnceLock::new();

pub fn register_plugin_host_dispatch(dispatch: PluginHostDispatch) {
    let _ = PLUGIN_HOST_DISPATCH.set(dispatch);
}

pub fn try_dispatch_plugin_host(name: &str, stack: &mut Vec<Value>) -> bool {
    PLUGIN_HOST_DISPATCH
        .get()
        .is_some_and(|dispatch| dispatch(name, stack))
}

#[derive(Debug, Clone)]
pub struct VmSuspendFrame {
    pub pc: usize,
    pub locals: Vec<Value>,
    pub stack: Vec<Value>,
    pub resume_fn: u32,
    pub state: u32,
}

#[cfg(feature = "interpreter")]
pub struct VmInstance {
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    pub globals: Vec<Value>,
    pub gc: Gc,
    pub host_fns: HashMap<String, HostFn>,
    pub printed: Vec<String>,
    pub runtime: Option<VmRuntime>,
    /// `(call_depth, catch_pc)` — catch PCs are only valid for the frame that issued `BeginTry`.
    try_handlers: Vec<(usize, usize)>,
    /// Saved `try_handlers.len()` at each `run_function_with_args` entry (mir-run parity).
    try_handler_frames: Vec<usize>,
    pending_catch: Option<usize>,
    /// Current `run_function_with_args` nesting depth (1 = entry / main).
    call_depth: usize,
    /// Throw that must unwind to an outer frame's `try` (ADR-145 nested throws).
    propagating_throw: Option<Value>,
    /// Async state machine resume state (local slot index for saved state id).
    pub async_state: u32,
    pub async_frame: Vec<Value>,
    pub suspended: bool,
    pub suspend_frame: Option<VmSuspendFrame>,
    /// Wave A W488 / ADR-832: nested call locals saved as GC-visible frames.
    pub locals_frames: Vec<Vec<Value>>,
    /// Wave A W498 / ADR-842: ManagedHeap slot indices from DictBuild ObjectRef
    /// probe (W497). Tracked for `collect_live_roots` / `HeapCompact` (mir-run
    /// `heap_slots` parity). ≠ migrate_enabled / ≠ cleanup / ≠ B-09 `[x]`.
    pub heap_slots: Vec<usize>,
    jit: crate::jit::JitCompiler,
}

#[cfg(feature = "interpreter")]
pub type WasmVM = VmInstance;

#[cfg(feature = "interpreter")]
impl Default for VmInstance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "interpreter")]
impl VmInstance {
    pub fn new() -> Self {
        let mut vm = Self {
            stack: Vec::new(),
            locals: Vec::new(),
            globals: Vec::new(),
            gc: Gc::default(),
            host_fns: HashMap::new(),
            printed: Vec::new(),
            runtime: None,
            try_handlers: Vec::new(),
            try_handler_frames: Vec::new(),
            pending_catch: None,
            call_depth: 0,
            propagating_throw: None,
            async_state: 0,
            async_frame: Vec::new(),
            suspended: false,
            suspend_frame: None,
            locals_frames: Vec::new(),
            heap_slots: Vec::new(),
            jit: crate::jit::JitCompiler::default(),
        };
        vm.register_default_hosts();
        vm
    }
}

#[cfg(feature = "interpreter")]
impl VmInstance {
    pub fn register_default_hosts(&mut self) {
        for id in HostFnId::all() {
            if let Some(f) = hosts::host_fn_for(*id) {
                self.host_fns.insert(id.name().into(), f);
            }
        }
    }

    pub fn run_program(&mut self, program: &CompiledProgram) -> SyxResult<Value> {
        crate::stdlib_phase84_hosts::clear_registered_tests();
        set_program_ctx(ProgramCtx {
            program: Arc::new(program.clone()),
            runtime: self.runtime.clone(),
        });
        set_active_vm(self);
        let result = self.run_program_inner(program);
        let outcomes = if result.is_ok() {
            self.drain_registered_tests(program)
        } else {
            crate::stdlib_phase84_hosts::clear_registered_tests();
            Vec::new()
        };
        crate::stdlib_phase84_hosts::store_registered_test_outcomes(outcomes);
        clear_active_vm();
        clear_program_ctx();
        result
    }

    pub fn drain_registered_tests(
        &mut self,
        program: &CompiledProgram,
    ) -> Vec<crate::stdlib_phase84_hosts::RegisteredTestOutcome> {
        let mut outcomes = Vec::new();
        loop {
            let batch = crate::stdlib_phase84_hosts::take_registered_test_bodies();
            if batch.is_empty() {
                break;
            }
            for (name, body) in batch {
                let failure = crate::stdlib_phase84_hosts::registered_body_error(self, program, &body);
                outcomes.push(crate::stdlib_phase84_hosts::RegisteredTestOutcome { name, failure });
            }
        }
        outcomes
    }

    fn run_program_inner(&mut self, program: &CompiledProgram) -> SyxResult<Value> {
        self.globals.resize(program.globals.len(), Value::Nil);
        for func in &program.functions {
            if func.name == "__init_global" {
                self.run_function(func, program)?;
            }
        }
        let main = program
            .functions
            .iter()
            .find(|f| f.name == program.entry_point)
            .ok_or_else(|| {
                SyxDiagnostic::error(format!("No entry function '{}' found", program.entry_point))
            })?;
        self.run_function(main, program)
    }

    pub fn run_function(
        &mut self,
        func: &FunctionUnit,
        program: &CompiledProgram,
    ) -> SyxResult<Value> {
        let v = self.run_function_with_args(func, program, &[])?;
        if let Some(err) = self.propagating_throw.take() {
            let msg = Self::uncaught_throw_message(&err);
            return Err(SyxDiagnostic::error(msg).with_code("R007"));
        }
        Ok(v)
    }

    /// ADR-140: expose JIT counters for tests / observability.
    pub fn jit_deopt_count(&self) -> u64 {
        self.jit.deopt_count()
    }

    pub fn jit_native_call_count(&self) -> u64 {
        self.jit.native_call_count()
    }

    pub fn jit_last_deopt_reason(&self) -> Option<&str> {
        self.jit.last_deopt_reason()
    }

    pub fn jit_native_ready(&self, func: &str) -> bool {
        self.jit.native_ready(func)
    }

    /// Wave A W135 / ADR-479: drop `BeginTry` handlers registered in the callee frame.
    fn pop_try_handler_frame(&mut self) {
        if let Some(base) = self.try_handler_frames.pop() {
            self.try_handlers.truncate(base);
        }
    }

    /// Wave A W488 / ADR-832 + W489 / ADR-833: live GC roots for `GcCollect` / `HeapCompact`.
    ///
    /// Includes stack/locals/globals plus nested `locals_frames`, optional
    /// `suspend_frame` stack/locals, and actor mailbox Value-queue snapshots.
    /// Does **not** claim VM↔MirRuntime heap convergence or unified-heap enable.
    pub fn collect_live_roots(&self) -> Vec<Value> {
        let mut roots = self.stack.clone();
        roots.extend(self.locals.clone());
        roots.extend(self.globals.clone());
        for frame in &self.locals_frames {
            roots.extend(frame.iter().cloned());
        }
        if let Some(frame) = &self.suspend_frame {
            roots.extend(frame.stack.iter().cloned());
            roots.extend(frame.locals.iter().cloned());
        }
        // Wave A W489 / ADR-833: in-flight actor mailbox Value queues.
        roots.extend(actor_mailbox::mailbox_collect_roots());
        // Wave A W498 / ADR-842 + W500 / ADR-844: ManagedHeap slots from
        // DictBuild ObjectRef probe and CallHost-returned ObjectRefs.
        for slot in &self.heap_slots {
            if let Ok(heap) = syx_runtime_core::global_heap().lock() {
                if let Some(v) = heap.get(*slot) {
                    roots.push(v.clone());
                }
            }
        }
        roots
    }


    /// Wave A W525 / ADR-869: StoreLocal ObjectRef dual-track death-path —
    /// prune matching `heap_slots` entries and `ManagedHeap::free_slot`. Keeps
    /// ObjectRef RC + Type_deinit on the StoreLocal path. ≠ RC retire / ≠
    /// deinit strip / ≠ migrate_enabled / ≠ B-09 `[x]`.
    fn maybe_untrack_objectref_value(&mut self, v: &Value) {
        if !v.is_object_ref() {
            return;
        }
        if !syx_runtime_core::ManagedHeap::enabled() {
            return;
        }
        let Some(id) = Value::object_ref_identity(v) else {
            return;
        };
        if let Ok(mut heap) = syx_runtime_core::global_heap().lock() {
            let mut remaining = Vec::with_capacity(self.heap_slots.len());
            for s in self.heap_slots.drain(..) {
                if heap.get(s).and_then(Value::object_ref_identity) == Some(id) {
                    heap.free_slot(s);
                } else {
                    remaining.push(s);
                }
            }
            self.heap_slots = remaining;
        }
    }

    // Wave A W553 / ADR-897: MutRefCell dual-register track helpers peeled
    // (true-delete). ObjectRef untrack retained. ≠ migrate flip ≠ enable ≠ B-09 `[x]`.

    /// Wave A W491 / ADR-835: fully_converged narrow GcCollect entry.
    ///
    /// ManagedHeap-only via `collect_global` (aligned with MirRuntime
    /// `heap_collect`). MemoryPool mark-sweep residual **retired** from this
    /// path (ADR-834 residual documented; ObjectRef/`MemoryPool` storage still
    /// present — ≠ enable / ≠ B-09 `[x]`).
    /// Wave A W527 / ADR-871: swept ObjectRef → Type_deinit (skip weak-dead).
    /// StoreLocal / run_class_deinits ObjectRef RC still live.
    pub fn run_gc_collect(&mut self, program: &CompiledProgram) -> SyxResult<()> {
        let roots = self.collect_live_roots();
        let swept = syx_runtime_core::collect_global(&roots);
        for v in swept {
            if Value::object_ref_is_weak_dead(&v) {
                continue;
            }
            self.run_deinit_for_value(&v, program)?;
        }
        Ok(())
    }

    pub fn run_function_with_args(
        &mut self,
        func: &FunctionUnit,
        program: &CompiledProgram,
        args: &[Value],
    ) -> SyxResult<Value> {
        if self.suspend_frame.is_none()
            && (crate::jit::JitCompiler::fastpath_enabled()
                || crate::jit::JitCompiler::tier2_opt_in_enabled())
        {
            self.jit.record_call(func.name.as_str());
            let _ = self.jit.maybe_compile(func.name.as_str(), program);
            if let Some(v) = self.jit.invoke_native(func.name.as_str(), args)? {
                return Ok(v);
            }
        }
        let outer_locals = std::mem::take(&mut self.locals);
        self.locals_frames.push(outer_locals);
        self.call_depth += 1;
        self.try_handler_frames.push(self.try_handlers.len());
        let mut pc = if let Some(frame) = self.suspend_frame.take() {
            self.locals = frame.locals;
            self.stack = frame.stack;
            self.async_state = frame.state;
            frame.pc
        } else {
            self.locals.resize(func.local_count as usize, Value::Nil);
            for (i, arg) in args.iter().enumerate() {
                if i < self.locals.len() {
                    self.locals[i] = arg.clone();
                }
            }
            0
        };
        self.suspended = false;
        while pc < func.instructions.len() {
            if let Some(catch_pc) = self.pending_catch.take() {
                pc = catch_pc;
                continue;
            }
            if self.propagating_throw.is_some() {
                break;
            }
            pc = self.exec_instruction(&func.instructions[pc], program, &func.instructions, pc)?;
            if self.propagating_throw.is_some() {
                break;
            }
            if self.suspended {
                break;
            }
        }
        self.call_depth = self.call_depth.saturating_sub(1);
        if self.suspended {
            self.locals = self.locals_frames.pop().unwrap_or_default();
            return Ok(Value::Nil);
        }
        if self.propagating_throw.is_some() {
            self.pop_try_handler_frame();
            self.locals = self.locals_frames.pop().unwrap_or_default();
            return Ok(Value::Nil);
        }
        if func.name == program.entry_point {
            self.run_class_deinits_if_enabled(program)?;
        }
        let result = self.stack.pop().unwrap_or(Value::Nil);
        self.pop_try_handler_frame();
        self.locals = self.locals_frames.pop().unwrap_or_default();
        Ok(result)
    }

    /// Phase 134: `super.init(...)` runs the superclass init body on an existing subclass `self`.
    fn run_init_on_existing_self(
        &mut self,
        func: &FunctionUnit,
        program: &CompiledProgram,
        self_val: Value,
        args: &[Value],
    ) -> SyxResult<Value> {
        let outer_locals = std::mem::take(&mut self.locals);
        self.locals_frames.push(outer_locals);
        self.locals.resize(func.local_count as usize, Value::Nil);
        for (i, arg) in args.iter().enumerate() {
            if i < self.locals.len() {
                self.locals[i] = arg.clone();
            }
        }
        if let Some(self_slot) = func.local_names.iter().position(|n| n == "self") {
            self.locals[self_slot] = self_val.clone();
        }
        let body_end = if func.init_wrap_pc > func.init_body_pc {
            func.init_wrap_pc as usize
        } else {
            func.instructions.len()
        };
        let mut pc = func.init_body_pc as usize;
        while pc < body_end {
            pc = self.exec_instruction(&func.instructions[pc], program, &func.instructions, pc)?;
            if self.suspended {
                break;
            }
        }
        let self_slot = func.local_names.iter().position(|n| n == "self");
        let result = if let Some(slot) = self_slot {
            self.locals[slot].clone()
        } else {
            self_val
        };
        if self.suspended {
            self.locals = self.locals_frames.pop().unwrap_or_default();
            return Ok(result);
        }
        // Historical: success leaves init locals in place and drops outer frame (still GC-visible via pop).
        let _ = self.locals_frames.pop();
        Ok(result)
    }

    fn run_class_deinits_if_enabled(&mut self, _program: &CompiledProgram) -> SyxResult<()> {
        // Wave A W526 / ADR-870: run_class_deinits ObjectRef ManagedHeap +
        // heap_slots unroot (death-path first-slice).
        // Wave A W528 / ADR-872: ObjectRef RC true-retire — no
        // release_object_ref / eager Type_deinit; finalizer via W527 collect
        // sweep. Keep maybe_untrack. ≠ Value API delete / ≠ migrate / ≠ enable.
        let locals: Vec<Value> = self.locals.clone();
        for local in locals {
            self.maybe_untrack_objectref_value(&local);
        }
        Ok(())
    }

    fn run_deinit_for_value(&mut self, value: &Value, program: &CompiledProgram) -> SyxResult<()> {
        let Some(type_name) = value.object_ref_type_name() else {
            return Ok(());
        };
        if matches!(
            type_name.as_str(),
            "ObjectIdentifier" | "Hasher" | "Result" | "MutRefCell"
        ) {
            return Ok(());
        }
        // Mark weak handles dead for codegen-emitted deinits (binding override),
        // not only the ObjectRefRelease RC path.
        Value::mark_object_weak_dead(value);
        let deinit_name = format!("{type_name}_deinit");
        if let Some(func) = program.functions.iter().find(|f| f.name == deinit_name) {
            self.run_function_with_args(func, program, &[value.clone()])?;
        }
        Ok(())
    }

    pub fn run_closure_with_captures(
        &mut self,
        func: &FunctionUnit,
        program: &CompiledProgram,
        captures: &[Value],
        args: &[Value],
    ) -> SyxResult<Value> {
        let saved_locals = std::mem::take(&mut self.locals);
        self.locals_frames.push(saved_locals);
        self.call_depth += 1;
        self.locals.resize(func.local_count as usize, Value::Nil);
        for (i, arg) in args.iter().enumerate() {
            if i < self.locals.len() {
                self.locals[i] = arg.clone();
            }
        }
        let capture_start = func.param_count as usize;
        for (i, cap) in captures.iter().enumerate() {
            let slot = capture_start + i;
            if slot < self.locals.len() {
                self.locals[slot] = cap.clone();
            }
        }
        let mut pc = 0usize;
        while pc < func.instructions.len() {
            if let Some(catch_pc) = self.pending_catch.take() {
                pc = catch_pc;
                continue;
            }
            if self.propagating_throw.is_some() {
                break;
            }
            match self.exec_instruction(&func.instructions[pc], program, &func.instructions, pc) {
                Ok(next) => pc = next,
                Err(err) => {
                    self.call_depth = self.call_depth.saturating_sub(1);
                    self.locals = self.locals_frames.pop().unwrap_or_default();
                    return Err(err);
                }
            }
            if self.propagating_throw.is_some() {
                break;
            }
        }
        self.call_depth = self.call_depth.saturating_sub(1);
        if self.propagating_throw.is_some() {
            self.locals = self.locals_frames.pop().unwrap_or_default();
            return Ok(Value::Nil);
        }
        let result = self.stack.pop().unwrap_or(Value::Nil);
        self.locals = self.locals_frames.pop().unwrap_or_default();
        Ok(result)
    }

    pub fn step_instruction(
        &mut self,
        func: &FunctionUnit,
        program: &CompiledProgram,
        pc: usize,
    ) -> SyxResult<usize> {
        self.exec_instruction(&func.instructions[pc], program, &func.instructions, pc)
    }

    /// Wave A W133 / ADR-477: uncaught facade dataclass throws expose `message` (not empty R007).
    fn uncaught_throw_message(err: &Value) -> String {
        let s = err.as_str();
        if !s.is_empty() {
            return s;
        }
        if let Value::Object(map) | Value::Dict(map) = err {
            if let Some(Value::Str(msg)) = map.get("message") {
                if !msg.is_empty() {
                    return msg.clone();
                }
            }
        }
        if let Some(fields) = Value::object_ref_user_fields(err) {
            for (k, v) in fields {
                if k == "message" {
                    let msg = v.as_str();
                    if !msg.is_empty() {
                        return msg;
                    }
                }
            }
        }
        value_description(err)
    }

    /// Deliver `err` to a `try` at the current call depth, or mark for outer unwind.
    fn raise_or_propagate(&mut self, err: Value, pc: usize) -> SyxResult<usize> {
        if let Some(&(depth, catch_pc)) = self.try_handlers.last() {
            if depth == self.call_depth {
                self.try_handlers.pop();
                self.stack.push(err);
                self.pending_catch = Some(catch_pc);
                return Ok(catch_pc);
            }
        }
        if self.call_depth <= 1 {
            let msg = Self::uncaught_throw_message(&err);
            return Err(SyxDiagnostic::error(msg).with_code("R007"));
        }
        self.propagating_throw = Some(err);
        Ok(pc)
    }

    /// Host `SyxDiagnostic`: preserve code (e.g. C004 / U001) when uncaught; else raise into try/catch.
    fn fail_host_or_raise(&mut self, err: SyxDiagnostic, pc: usize) -> SyxResult<usize> {
        let has_local_catch = self
            .try_handlers
            .last()
            .is_some_and(|&(depth, _)| depth == self.call_depth);
        if !has_local_catch && self.call_depth <= 1 {
            return Err(err);
        }
        let mut map = std::collections::HashMap::new();
        map.insert("message".into(), Value::Str(err.message.clone()));
        if let Some(code) = err.code.clone() {
            map.insert("code".into(), Value::Str(code));
        }
        self.raise_or_propagate(Value::Object(map), pc)
    }

    /// Dispatch `CallHost` without ObjectRef ManagedHeap tracking (W500 wraps this).
    fn exec_call_host(
        &mut self,
        idx: u32,
        program: &CompiledProgram,
        pc: usize,
    ) -> SyxResult<usize> {
        let name = syx_stdlib::host_fn_name_for_index(idx)
            .or_else(|| program.host_fn_names.get(idx as usize).map(|s| s.as_str()))
            .unwrap_or("");
        if name == "__syx_array_sorted" {
            let receiver = self.stack.pop().unwrap_or(Value::Nil);
            let sorted = self.sort_array_receiver(receiver, program)?;
            self.stack.push(sorted);
            return Ok(pc + 1);
        }
        // Prefer Result-aware dispatch so fail-closed hosts (ADR-225 HOF) are not
        // swallowed by `.unwrap_or(false)` into a silent Nil success.
        match hosts::dispatch_numeric_host(name, &mut self.stack) {
            Ok(true) => return Ok(pc + 1),
            Ok(false) => {}
            Err(err) => return self.fail_host_or_raise(err, pc),
        }
        match hosts::dispatch_collection_host(name, &mut self.stack) {
            Ok(true) => return Ok(pc + 1),
            Ok(false) => {}
            Err(err) => return self.fail_host_or_raise(err, pc),
        }
        match hosts::dispatch_bigint_host(name, &mut self.stack) {
            Ok(true) => return Ok(pc + 1),
            Ok(false) => {}
            Err(err) => return self.fail_host_or_raise(err, pc),
        }
        if crate::try_dispatch_plugin_host(name, &mut self.stack) {
            return Ok(pc + 1);
        }
        if let Some(host) = self.host_fns.get(name).copied() {
            if let Err(err) = host(&mut self.stack) {
                // Preserve diagnostic code for try/catch (`err.code`), e.g. C004.
                return self.fail_host_or_raise(err, pc);
            }
        }
        Ok(pc + 1)
    }

    /// After a nested call returns, either push the value or continue throw unwind.
    /// Wave A W506 / ADR-850: on success, dual-register stack-top MutRefCell into
    /// ManagedHeap + heap_slots (Call/init/closure / `@Lazy` materialize beyond
    /// CallHost). Wave A W550 / ADR-894: ObjectRef re-entry maybe_track peeled;
    /// MutRefCell track kept. Throw unwind unchanged. ≠ migrate / cleanup.
    fn finish_call(&mut self, result: Value, pc: usize) -> SyxResult<usize> {
        if let Some(err) = self.propagating_throw.take() {
            return self.raise_or_propagate(err, pc);
        }
        self.stack.push(result);
        Ok(pc + 1)
    }

    fn exec_instruction(
        &mut self,
        instr: &Instruction,
        program: &CompiledProgram,
        code: &[Instruction],
        pc: usize,
    ) -> SyxResult<usize> {
        match instr {
            Instruction::ConstI32(v) => self.stack.push(Value::I32(*v)),
            Instruction::ConstF64(v) => self.stack.push(Value::F64(*v)),
            Instruction::ConstStr(s) => self.stack.push(Value::maybe_offheap_string(
                Value::Str(s.clone()),
                syx_runtime_core::HeapSlotKind::String,
            )),
            Instruction::ConstBool(v) => self.stack.push(Value::Bool(*v)),
            Instruction::ConstNil => self.stack.push(Value::Nil),
            Instruction::Pop => {
                self.stack.pop();
            }
            Instruction::Dup => {
                // Wave A W523 / ADR-867: Dup ObjectRef RC bump retired
                // (true-retire first-slice). Keep clone/push + W510 dual-track.
                // Release/StoreLocal/deinit + Value API + RT ABI still live.
                // ≠ Value RC full delete / migrate / cleanup / enable.
                let v = self.stack.last().cloned().unwrap_or(Value::Nil);
                self.stack.push(v);
            }
            Instruction::LoadLocal(i) => {
                // Wave A W523 / ADR-867: LoadLocal ObjectRef RC bump retired
                // (true-retire first-slice; parity with Dup). Keep push +
                // W510 dual-track. SSA LoadLocal already no-retain (W517).
                // ≠ Value RC full delete / Release strip / migrate / enable.
                let v = self.locals.get(*i as usize).cloned().unwrap_or(Value::Nil);
                self.stack.push(v);
            }
            Instruction::StoreLocal(i) => {
                // Wave A W525 / ADR-869: StoreLocal ObjectRef ManagedHeap +
                // heap_slots unroot (death-path first-slice).
                // Wave A W528 / ADR-872: ObjectRef RC true-retire — no
                // release_object_ref / eager Type_deinit; finalizer via W527
                // collect sweep. Keep maybe_untrack. ≠ Value API delete /
                // ≠ migrate / ≠ enable.
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if self.locals.len() <= *i as usize {
                    self.locals.resize(*i as usize + 1, Value::Nil);
                }
                let old = std::mem::replace(&mut self.locals[*i as usize], v.clone());
                if old != v {
                    self.maybe_untrack_objectref_value(&old);
                }
            }
            Instruction::LoadGlobal(i) => {
                // Wave A W516 / ADR-860: after LoadGlobal push, dual-register
                // stack-top ObjectRef+MutRefCell into ManagedHeap + heap_slots.
                // No retain call (existing semantics). ≠ migrate / cleanup /
                // enable / SSA LoadLocal / B-09 [x].
                let v = if let Some(rt) = &self.runtime {
                    rt.load_global(*i as usize)
                } else {
                    self.globals.get(*i as usize).cloned().unwrap_or(Value::Nil)
                };
                self.stack.push(v);
            }
            Instruction::StoreGlobal(i) => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(rt) = &self.runtime {
                    rt.store_global(*i as usize, v);
                } else {
                    if self.globals.len() <= *i as usize {
                        self.globals.resize(*i as usize + 1, Value::Nil);
                    }
                    self.globals[*i as usize] = v;
                }
            }
            Instruction::I32Add => self.binop_i32(|a, b| a + b),
            Instruction::I32Sub => self.binop_i32(|a, b| a - b),
            Instruction::I32Mul => self.binop_i32(|a, b| a * b),
            Instruction::I32Div => self.binop_i32(|a, b| a / b),
            Instruction::I32Mod => self.binop_i32(|a, b| a % b),
            Instruction::I32Neg => {
                let v = self.stack.pop().unwrap_or(Value::I32(0));
                self.stack.push(Value::I32(-v.as_i32()));
            }
            Instruction::I32Eq => self.binop_bool(|a, b| a == b),
            Instruction::I32Ne => self.binop_bool(|a, b| a != b),
            Instruction::I32Lt => self.binop_i32_cmp(|a, b| a < b),
            Instruction::I32Le => self.binop_i32_cmp(|a, b| a <= b),
            Instruction::I32Gt => self.binop_i32_cmp(|a, b| a > b),
            Instruction::I32Ge => self.binop_i32_cmp(|a, b| a >= b),
            Instruction::BoolAnd => self.binop_bool(|a, b| a && b),
            Instruction::BoolOr => self.binop_bool(|a, b| a || b),
            Instruction::BoolNot => {
                let v = self.stack.pop().unwrap_or(Value::Bool(false));
                self.stack.push(Value::Bool(!v.as_bool()));
            }
            Instruction::Jump(target) => return Ok(*target),
            Instruction::JumpIfFalse(target) => {
                let cond = self.stack.pop().unwrap_or(Value::Bool(false));
                if !cond.as_bool() {
                    return Ok(*target);
                }
            }
            Instruction::Print => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                let text = value_to_display(&v);
                println!("{text}");
                if let Some(rt) = &self.runtime {
                    rt.record_print(text);
                } else {
                    self.printed.push(text);
                }
            }
            Instruction::ConcatStr => {
                let b = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let a = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let joined = format!("{}{}", a.as_str(), b.as_str());
                self.stack.push(Value::maybe_offheap_string(
                    Value::Str(joined),
                    syx_runtime_core::HeapSlotKind::String,
                ));
            }
            Instruction::ToStr => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Str(value_to_display(&v)));
            }
            Instruction::CallHost(idx) => {
                // Wave A W500 / ADR-844: after successful CallHost, dual-register
                // stack-top ObjectRef into ManagedHeap + heap_slots (covers host
                // paths such as `make_lazy_proxy`); identity-dedup for reused handles.
                // Wave A W504 / ADR-848: also track stack-top MutRefCell (covers
                // host paths such as `host_ordered_set_new`); ≠ migrate / cleanup.
                let next = self.exec_call_host(*idx, program, pc)?;
                return Ok(next);
            }
            Instruction::BeginTry(catch_pc) => {
                self.try_handlers.push((self.call_depth, *catch_pc));
            }
            Instruction::EndTry => {
                self.try_handlers.pop();
            }
            Instruction::Catch => {}
            Instruction::JumpIfNil(target) => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if matches!(v, Value::Nil) {
                    return Ok(*target);
                }
                self.stack.push(v);
            }
            Instruction::ThrowError => {
                let err = self.stack.pop().unwrap_or(Value::Str("error".into()));
                return self.raise_or_propagate(err, pc);
            }
            Instruction::BindResult(_) => {}
            Instruction::ReturnErr => {
                let err = self.stack.pop().unwrap_or(Value::Str("error".into()));
                return self.raise_or_propagate(err, pc);
            }
            Instruction::MakeIterator => {
                let top = self.stack.pop().unwrap_or(Value::Nil);
                let witness_next = match top.mut_ref_cell_get_field("__witness_Iterator_next") {
                    Value::I32(fn_idx) => Some(fn_idx as u32),
                    _ => None,
                };
                if let Some(next_fn) = witness_next {
                    self.stack
                        .push(Value::Iterator(Box::new(IteratorState::Witness {
                            obj: Box::new(top),
                            next_fn,
                        })));
                    return Ok(pc + 1);
                }
                let resolved = Value::resolve_heap_slot(&top);
                let items = match &resolved {
                    Value::Object(map) | Value::Dict(map) => {
                        if let (Some(start), Some(end)) = (map.get("start"), map.get("end")) {
                            let s = start.as_i64();
                            let e = end.as_i64();
                            let inclusive =
                                map.get("inclusive").map(|v| v.as_bool()).unwrap_or(false);
                            if inclusive {
                                if s <= e {
                                    (s..=e).map(platform_int_value).collect()
                                } else {
                                    Vec::new()
                                }
                            } else if s < e {
                                (s..e).map(platform_int_value).collect()
                            } else {
                                Vec::new()
                            }
                        } else {
                            vec![top.clone()]
                        }
                    }
                    Value::Array(a) => a.clone(),
                    // ADR-226 / M2-S2-01: String for-in = grapheme Character (align .count / s[i]).
                    Value::Str(s) => crate::grapheme::graphemes_vec(&s)
                        .into_iter()
                        .map(Value::Character)
                        .collect(),
                    other if other.as_i64() >= 0 => {
                        (0..other.as_i64()).map(platform_int_value).collect()
                    }
                    other => vec![other.clone()],
                };
                self.stack
                    .push(Value::Iterator(Box::new(IteratorState::Static {
                        items,
                        index: 0,
                    })));
            }
            Instruction::IteratorNext => {
                let iter =
                    self.stack
                        .pop()
                        .unwrap_or(Value::Iterator(Box::new(IteratorState::Static {
                            items: vec![],
                            index: 0,
                        })));
                match iter {
                    Value::Iterator(state) => match *state {
                        IteratorState::Static { items, index } => {
                            if index < items.len() {
                                let val = items[index].clone();
                                self.stack
                                    .push(Value::Iterator(Box::new(IteratorState::Static {
                                        items,
                                        index: index + 1,
                                    })));
                                self.stack.push(val);
                                self.stack.push(Value::Bool(true));
                            } else {
                                self.stack.push(Value::Bool(false));
                            }
                        }
                        IteratorState::Witness { obj, next_fn } => {
                            self.stack.push((*obj).clone());
                            let func =
                                program.functions.get(next_fn as usize).ok_or_else(|| {
                                    SyxDiagnostic::error(format!(
                                        "Unknown witness method index {next_fn}"
                                    ))
                                })?;
                            let arg_count = func.param_count as usize;
                            let start = self.stack.len().saturating_sub(arg_count);
                            let args: Vec<Value> = self.stack.drain(start..).collect();
                            let result = self.run_function_with_args(func, program, &args)?;
                            if matches!(result, Value::Nil) {
                                self.stack.push(Value::Bool(false));
                            } else {
                                self.stack.push(Value::Iterator(Box::new(
                                    IteratorState::Witness { obj, next_fn },
                                )));
                                self.stack.push(result);
                                self.stack.push(Value::Bool(true));
                            }
                        }
                    },
                    _ => self.stack.push(Value::Bool(false)),
                }
            }
            Instruction::IteratorDone => {
                self.stack.push(Value::Bool(false));
            }
            Instruction::MethodCall(id) => {
                // Wave A W509 / ADR-853: after successful MethodCall dispatch,
                // dual-register stack-top ObjectRef+MutRefCell into ManagedHeap +
                // heap_slots (CallHost-style; covers HOF/HTTP/SQL/host exits).
                // ≠ finish_call refactor / ≠ migrate / cleanup.
                self.call_method(*id, program)?;
            }
            Instruction::ArrayBuild(count) => {
                let mut items = Vec::with_capacity(*count as usize);
                for _ in 0..*count {
                    items.push(self.stack.pop().unwrap_or(Value::Nil));
                }
                items.reverse();
                if Value::heap_v4_enabled() {
                    let array = Value::Array(items);
                    self.stack.push(Value::alloc_heap_slot(
                        array,
                        syx_runtime_core::HeapSlotKind::Array,
                    ));
                } else {
                    self.stack.push(Value::Array(items));
                }
            }
            Instruction::DictBuild(count) => {
                let mut map = HashMap::new();
                for _ in 0..*count {
                    let key_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    syx_runtime_core::dict_insert(&mut map, key_val, val);
                }
                let type_name = map.get("__type").map(|v| {
                    let resolved = Value::resolve_heap_slot(v);
                    match resolved {
                        Value::Str(s) => s,
                        other => other.as_str(),
                    }
                });
                if type_name.as_deref() == Some("MutRefCell") {
                    // Wave A W502 / ADR-846: MutRefCell VM DictBuild ManagedHeap +
                    // heap_slots first-slice (align ObjectRef DictBuild W497/W498);
                    // ≠ MutRefCell full migrate / ≠ migrate_enabled / ≠ cleanup.
                    let v = Value::mut_ref_cell_from_map(map);
                    self.stack.push(v);
                } else if map.contains_key("__type") {
                    ensure_object_id(&mut map);
                    // Wave A W547 / ADR-891: DictBuild ObjectRef dual-track birth
                    // collapsed — heap-primary single-track via alloc_heap_slot
                    // when ManagedHeap enabled (no DictBuild-site
                    // maybe_track_objectref_value). Historical W497/W498/W500
                    // dual-register path retired at this site.
                    // Wave A W550 / ADR-894: re-entry ObjectRef maybe_track_*
                    // peeled (CallHost/ObjectGet/…). MutRefCell maybe_track_*
                    // kept. ≠ migrate_enabled / ≠ cleanup / ≠ B-09 `[x]`.
                    // Historical cite: ADR-841 / ADR-843 DictBuild align.
                    let v = Value::object_ref_from_map(map, true);
                    if syx_runtime_core::ManagedHeap::enabled() {
                        self.stack.push(Value::alloc_heap_slot(
                            v,
                            syx_runtime_core::HeapSlotKind::Object,
                        ));
                    } else {
                        self.stack.push(v);
                    }
                } else if Value::heap_v4_enabled() {
                    let dict = Value::Dict(map);
                    self.stack.push(Value::alloc_heap_slot(
                        dict,
                        syx_runtime_core::HeapSlotKind::Object,
                    ));
                } else {
                    self.stack.push(Value::Dict(map));
                }
            }
            Instruction::SetBuild(count) => {
                let mut items = Vec::new();
                for _ in 0..*count {
                    items.push(self.stack.pop().unwrap_or(Value::Nil));
                }
                items.reverse();
                let mut unique = Vec::new();
                for item in items {
                    if !unique.iter().any(|u| value_equals(u, &item)) {
                        unique.push(item);
                    }
                }
                // ADR-224 / M2-S1-02: allocate HeapSlot so Set.insert/remove can write back.
                if Value::heap_v4_enabled() {
                    let set = Value::Set(unique);
                    self.stack.push(Value::alloc_heap_slot(
                        set,
                        syx_runtime_core::HeapSlotKind::Set,
                    ));
                } else {
                    self.stack.push(Value::Set(unique));
                }
            }
            Instruction::ConstNumeric { kind, bits } => {
                if let Some(k) = syx_stdlib::NumericKind::from_byte(*kind) {
                    self.stack.push(value_from_numeric(k, *bits));
                } else {
                    self.stack.push(Value::Nil);
                }
            }
            Instruction::ConstNumeric128 { kind, lo, hi } => {
                if let Some(k) = syx_stdlib::NumericKind::from_byte(*kind) {
                    self.stack.push(value_from_numeric128(k, *lo, *hi));
                } else {
                    self.stack.push(Value::Nil);
                }
            }
            Instruction::SimdAdd { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Add)?;
            }
            Instruction::SimdSub { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Sub)?;
            }
            Instruction::SimdMul { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Mul)?;
            }
            Instruction::SimdDiv { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Div)?;
            }
            Instruction::SimdEq { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_eq(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdLt { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_lt(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdGt { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_gt(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdNe { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_ne(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdLe { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_le(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdGe { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_ge(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdAnd { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_and(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdOr { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_or(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdXor { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_xor(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdNot { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_not(&v) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdShl { lanes, element } => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_shl(&v, amount) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdShr { lanes, element } => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_shr(&v, amount) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdMin { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_min(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdMax { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_max(&a, &b) {
                    let _ = (lanes, element);
                    self.stack.push(r);
                }
            }
            Instruction::SimdF32Add { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Add)?;
            }
            Instruction::SimdF32Sub { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Sub)?;
            }
            Instruction::SimdF32Mul { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Mul)?;
            }
            Instruction::SimdF32Div { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Div)?;
            }
            Instruction::SimdF64Add { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Add)?;
            }
            Instruction::SimdI16Add { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Add)?;
            }
            Instruction::SimdI16Sub { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Sub)?;
            }
            Instruction::SimdI16Mul { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Mul)?;
            }
            Instruction::SimdI8Add { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Add)?;
            }
            Instruction::SimdI8Sub { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Sub)?;
            }
            Instruction::SimdI8Mul { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Mul)?;
            }
            Instruction::SimdI8And { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_and(&a, &b) {
                    self.stack.push(r);
                }
            }
            Instruction::SimdI8Or { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_or(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Xor { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_xor(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Not { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_not(&v) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Min { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_min(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Max { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_max(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8UMin { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_umin(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8UMax { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_umax(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Abs { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_abs(&v) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Neg { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_neg(&v) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Extract { lanes, element } => {
                let lane = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_extract_lane(&v, lane) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Replace { lanes, element } => {
                let scalar = self.stack.pop().unwrap_or(Value::Nil);
                let lane = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_replace_lane(&v, lane, &scalar) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Eq { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_eq(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Ne { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_ne(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Le { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_le(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Ge { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_ge(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8AvgrU { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_avgr_u(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Popcnt { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_popcnt(&v) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Bitmask { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_bitmask(&v) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8AllTrue { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_all_true(&v) {
                    self.stack.push(r);
                } else {
                    self.stack.push(Value::Bool(false));
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8AnyTrue { lanes, element } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_any_true(&v) {
                    self.stack.push(r);
                } else {
                    self.stack.push(Value::Bool(false));
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Lt { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_lt(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Gt { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_gt(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8AddSat { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_add_sat(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8SubSat { lanes, element } => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_sub_sat(&a, &b) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Shl { lanes, element } => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_shl(&v, amount) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI8Shr { lanes, element } => {
                let amount = self.stack.pop().unwrap_or(Value::I32(0)).as_i32();
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(r) = syx_runtime_core::simd_shr(&v, amount) {
                    self.stack.push(r);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdI64Add { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Add)?;
            }
            Instruction::SimdF64Sub { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Sub)?;
            }
            Instruction::SimdF64Mul { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Mul)?;
            }
            Instruction::SimdF64Div { lanes, element } => {
                self.exec_simd_binop(*lanes, *element, SimdBinOp::Div)?;
            }
            Instruction::SimdReduceAdd { lanes, element } => {
                let v = self
                    .stack
                    .last()
                    .ok_or_else(|| SyxDiagnostic::error("empty stack"))?;
                if let Some(sum) = syx_runtime_core::simd_reduce_add(v) {
                    self.stack.pop();
                    self.stack.push(sum);
                }
                let _ = (lanes, element);
            }
            Instruction::SimdLoad {
                lanes,
                element,
                offset,
            } => {
                let array = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(k) = syx_stdlib::NumericKind::from_byte(*element) {
                    if let Some(v) =
                        syx_runtime_core::simd_load_from_array(&array, *lanes, k, *offset as usize)
                    {
                        self.stack.push(v);
                    }
                }
            }
            Instruction::SimdStore {
                lanes,
                element,
                offset,
            } => {
                let simd = self.stack.pop().unwrap_or(Value::Nil);
                let mut array = self.stack.pop().unwrap_or(Value::Array(vec![]));
                let _ = (lanes, element);
                if syx_runtime_core::simd_store_to_array(&mut array, *offset as usize, &simd)
                    .is_some()
                {
                    self.stack.push(array);
                }
            }
            Instruction::SimdBroadcast { lanes, element } => {
                let scalar = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(k) = syx_stdlib::NumericKind::from_byte(*element) {
                    if let Some(v) = syx_runtime_core::simd_broadcast(*lanes, k, &scalar) {
                        self.stack.push(v);
                    }
                }
            }
            Instruction::NumericAdd(k) => self.exec_numeric_binop(*k, numeric_add)?,
            Instruction::NumericSub(k) => self.exec_numeric_binop(*k, numeric_sub)?,
            Instruction::NumericMul(k) => self.exec_numeric_binop(*k, numeric_mul)?,
            Instruction::NumericDiv(k) => self.exec_numeric_binop(*k, numeric_div)?,
            Instruction::NumericMod(k) => self.exec_numeric_binop(*k, numeric_mod)?,
            Instruction::NumericBitAnd(k) => self.exec_numeric_binop(*k, numeric_bitand)?,
            Instruction::NumericBitOr(k) => self.exec_numeric_binop(*k, numeric_bitor)?,
            Instruction::NumericBitXor(k) => self.exec_numeric_binop(*k, numeric_bitxor)?,
            Instruction::NumericShl(k) => self.exec_numeric_binop(*k, numeric_shl)?,
            Instruction::NumericShr(k) => self.exec_numeric_binop(*k, numeric_shr)?,
            Instruction::NumericNeg(k) => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(k) = syx_stdlib::NumericKind::from_byte(*k) {
                    if let Some(r) = numeric_neg(k, &v) {
                        self.stack.push(r);
                    } else {
                        return Err(SyxDiagnostic::error("Numeric overflow"));
                    }
                }
            }
            Instruction::NumericBitNot(k) => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(k) = syx_stdlib::NumericKind::from_byte(*k) {
                    if let Some(r) = numeric_bitnot(k, &v) {
                        self.stack.push(r);
                    } else {
                        return Err(SyxDiagnostic::error("Bitwise not on non-integer"));
                    }
                }
            }
            Instruction::NumericEq(k) => {
                self.exec_numeric_cmp(*k, |o| o == std::cmp::Ordering::Equal)?
            }
            Instruction::NumericNe(k) => {
                self.exec_numeric_cmp(*k, |o| o != std::cmp::Ordering::Equal)?
            }
            Instruction::NumericLt(k) => {
                self.exec_numeric_cmp(*k, |o| o == std::cmp::Ordering::Less)?
            }
            Instruction::NumericLe(k) => self.exec_numeric_cmp(*k, |o| {
                o == std::cmp::Ordering::Less || o == std::cmp::Ordering::Equal
            })?,
            Instruction::NumericGt(k) => {
                self.exec_numeric_cmp(*k, |o| o == std::cmp::Ordering::Greater)?
            }
            Instruction::NumericGe(k) => self.exec_numeric_cmp(*k, |o| {
                o == std::cmp::Ordering::Greater || o == std::cmp::Ordering::Equal
            })?,
            Instruction::NumericCast { from, to } => {
                let v = self.stack.pop().unwrap_or(Value::Nil);
                if let (Some(fk), Some(tk)) = (
                    syx_stdlib::NumericKind::from_byte(*from),
                    syx_stdlib::NumericKind::from_byte(*to),
                ) {
                    match try_numeric_cast(fk, tk, &v) {
                        Ok(casted) => self.stack.push(casted),
                        Err(msg) => {
                            return Err(SyxDiagnostic::error(msg).with_code("R003"));
                        }
                    }
                }
            }
            Instruction::F64Add => self.binop_f64(|a, b| a + b),
            Instruction::F64Sub => self.binop_f64(|a, b| a - b),
            Instruction::F64Mul => self.binop_f64(|a, b| a * b),
            Instruction::F64Div => self.binop_f64(|a, b| a / b),
            Instruction::F64Neg => {
                let v = self.stack.pop().unwrap_or(Value::F64(0.0));
                let n = match v {
                    Value::F64(n) => n,
                    Value::F32(n) => n as f64,
                    _ => v.as_i64() as f64,
                };
                self.stack.push(Value::F64(-n));
            }
            Instruction::F64Eq => self.binop_f64_bool(|a, b| (a - b).abs() < f64::EPSILON),
            Instruction::F64Ne => self.binop_f64_bool(|a, b| (a - b).abs() >= f64::EPSILON),
            Instruction::ObjectGet => {
                // Wave A W512 / ADR-856: after ObjectGet re-entry, dual-register
                // stack-top ObjectRef+MutRefCell into ManagedHeap + heap_slots
                // (residual birth/re-entry). ≠ migrate / cleanup / enable.
                let key_val = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let obj = self.stack.pop().unwrap_or(Value::Nil);
                let field = obj.get_by_key(&key_val);
                self.stack.push(field);
            }
            Instruction::StrEq => {
                let b = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let a = self.stack.pop().unwrap_or(Value::Str(String::new()));
                self.stack.push(Value::Bool(a.as_str() == b.as_str()));
            }
            Instruction::StrNe => {
                let b = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let a = self.stack.pop().unwrap_or(Value::Str(String::new()));
                self.stack.push(Value::Bool(a.as_str() != b.as_str()));
            }
            Instruction::BoolEq => {
                let b = self.stack.pop().unwrap_or(Value::Bool(false));
                let a = self.stack.pop().unwrap_or(Value::Bool(false));
                self.stack.push(Value::Bool(a.as_bool() == b.as_bool()));
            }
            Instruction::BoolNe => {
                let b = self.stack.pop().unwrap_or(Value::Bool(false));
                let a = self.stack.pop().unwrap_or(Value::Bool(false));
                self.stack.push(Value::Bool(a.as_bool() != b.as_bool()));
            }
            Instruction::ValueEq => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(value_equals(&a, &b)));
            }
            Instruction::ValueNe => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(!value_equals(&a, &b)));
            }
            Instruction::RefEq => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(value_identical(&a, &b)));
            }
            Instruction::RefNe => {
                let b = self.stack.pop().unwrap_or(Value::Nil);
                let a = self.stack.pop().unwrap_or(Value::Nil);
                self.stack.push(Value::Bool(!value_identical(&a, &b)));
            }
            Instruction::Return => return Ok(code.len()),
            Instruction::GcCollect => {
                // Wave A W491 / ADR-835: ManagedHeap-only (MemoryPool mark-sweep retired).
                // Wave A W527 / ADR-871: sweep Type_deinit bridge.
                self.run_gc_collect(program)?;
            }
            Instruction::GcPin => {}
            Instruction::GcUnpin => {}
            // Wave A W544 / ADR-888: Instruction::ObjectRefRetain/Release peeled.
            Instruction::HeapCompact => {
                // Wave A W488 / ADR-832 + W489 / ADR-833: compact rewrite covers
                // suspend_frame + locals_frames + mailbox Value queues.
                // Wave A W498 / ADR-842: also rewrite `heap_slots` indices.
                // Wave A W527 / ADR-871: collect_global swept ObjectRef Type_deinit.
                let _roots = self.collect_live_roots();
                let swept = syx_runtime_core::collect_global(&_roots);
                for v in swept {
                    if Value::object_ref_is_weak_dead(&v) {
                        continue;
                    }
                    self.run_deinit_for_value(&v, program)?;
                }
                if let Ok(mut heap) = syx_runtime_core::global_heap().lock() {
                    let forwarding = heap.compact();
                    syx_runtime_core::ManagedHeap::rewrite_slot_indices(
                        &mut self.stack,
                        &forwarding,
                    );
                    syx_runtime_core::ManagedHeap::rewrite_slot_indices(
                        &mut self.locals,
                        &forwarding,
                    );
                    syx_runtime_core::ManagedHeap::rewrite_slot_indices(
                        &mut self.globals,
                        &forwarding,
                    );
                    for frame in &mut self.locals_frames {
                        syx_runtime_core::ManagedHeap::rewrite_slot_indices(frame, &forwarding);
                    }
                    if let Some(frame) = &mut self.suspend_frame {
                        syx_runtime_core::ManagedHeap::rewrite_slot_indices(
                            &mut frame.stack,
                            &forwarding,
                        );
                        syx_runtime_core::ManagedHeap::rewrite_slot_indices(
                            &mut frame.locals,
                            &forwarding,
                        );
                    }
                    for slot in &mut self.heap_slots {
                        if let Some(new_idx) = forwarding
                            .iter()
                            .find_map(|&(old, new)| (*slot == old).then_some(new))
                        {
                            *slot = new_idx;
                        }
                    }
                    actor_mailbox::mailbox_rewrite_slots(&forwarding);
                }
            }
            Instruction::ArrayGet => {
                // Wave A W513 / ADR-857: after ArrayGet re-entry, dual-register
                // stack-top ObjectRef+MutRefCell into ManagedHeap + heap_slots.
                // ≠ migrate / cleanup / enable / wrap emit retire.
                let idx = self.stack.pop().unwrap_or(Value::I32(0));
                let arr = self.stack.pop().unwrap_or(Value::Array(vec![]));
                let arr = match arr {
                    Value::HeapSlot { .. } => Value::resolve_heap_slot(&arr),
                    other => other,
                };
                match arr {
                    Value::Array(items) => {
                        let i = idx.as_i32();
                        if i < 0 || (i as usize) >= items.len() {
                            return Err(SyxDiagnostic::error(format!(
                                "Array index {i} out of bounds (count {})",
                                items.len()
                            ))
                            .with_code("R003"));
                        }
                        self.stack.push(items[i as usize].clone());
                    }
                    Value::Str(s) => {
                        let i = idx.as_i32();
                        match crate::grapheme::try_grapheme_at(&s, i) {
                            Ok(g) => self.stack.push(Value::Character(g)),
                            Err((i, count)) => {
                                return Err(SyxDiagnostic::error(format!(
                                    "String index {i} out of bounds (count {count})"
                                ))
                                .with_code("R003"));
                            }
                        }
                    }
                    other => self.stack.push(other),
                }
            }
            Instruction::ArrSorted => {
                let receiver = self.stack.pop().unwrap_or(Value::Nil);
                let sorted = self.sort_array_receiver(receiver, program)?;
                self.stack.push(sorted);
            }
            Instruction::CallSuper(idx) => {
                let func = program
                    .functions
                    .get(*idx as usize)
                    .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function index {idx}")))?;
                if func.name.ends_with("_init") && func.init_body_pc > 0 {
                    let arg_count = func.param_count as usize;
                    if self.stack.len() > arg_count {
                        let self_idx = self.stack.len() - arg_count - 1;
                        let self_val = self.stack[self_idx].clone();
                        let start = self.stack.len() - arg_count;
                        let args: Vec<Value> = self.stack.drain(start..).collect();
                        self.stack.remove(self_idx);
                        let result =
                            self.run_init_on_existing_self(func, program, self_val, &args)?;
                        self.stack.push(result);
                        return Ok(pc + 1);
                    }
                }
                let arg_count = func.param_count as usize;
                let start = self.stack.len().saturating_sub(arg_count);
                let args: Vec<Value> = self.stack.drain(start..).collect();
                let result = self.run_function_with_args(func, program, &args)?;
                return self.finish_call(result, pc);
            }
            Instruction::Call(idx) => {
                let func = program
                    .functions
                    .get(*idx as usize)
                    .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function index {idx}")))?;
                if crate::jit::JitCompiler::fastpath_enabled()
                    || crate::jit::JitCompiler::tier2_opt_in_enabled()
                {
                    let _ = self.jit.maybe_compile(func.name.as_str(), program);
                }
                let arg_count = func.param_count as usize;
                let start = self.stack.len().saturating_sub(arg_count);
                let mut args: Vec<Value> = self.stack.drain(start..).collect();
                // Phase 128: materialize `@Lazy` proxy receivers before instance methods
                // (not init/deinit — first arg is a ctor parameter, not `self`).
                let is_ctor = func.name.ends_with("_init") || func.name.ends_with("_deinit");
                if func.name.ends_with("_deinit") {
                    if let Some(recv) = args.first() {
                        Value::mark_object_weak_dead(recv);
                    }
                }
                if !is_ctor {
                    if let Some(recv) = args.first_mut() {
                        *recv =
                            crate::stdlib_phase120_hosts::materialize_lazy_receiver(recv.clone())?;
                    }
                }
                let func = self.resolve_virtual_method(func, &args, program);
                let result = self.run_function_with_args(func, program, &args)?;
                return self.finish_call(result, pc);
            }
            Instruction::CallIndirect(arg_count) => {
                let fn_idx_val = self.stack.pop().unwrap_or(Value::I32(0));
                let fn_idx = fn_idx_val.as_i32() as u32;
                let func = program.functions.get(fn_idx as usize).ok_or_else(|| {
                    SyxDiagnostic::error(format!("Unknown indirect function index {fn_idx}"))
                })?;
                let start = self.stack.len().saturating_sub(*arg_count as usize);
                let mut args: Vec<Value> = self.stack.drain(start..).collect();
                let is_ctor = func.name.ends_with("_init") || func.name.ends_with("_deinit");
                if func.name.ends_with("_deinit") {
                    if let Some(recv) = args.first() {
                        Value::mark_object_weak_dead(recv);
                    }
                }
                if !is_ctor {
                    if let Some(recv) = args.first_mut() {
                        *recv =
                            crate::stdlib_phase120_hosts::materialize_lazy_receiver(recv.clone())?;
                    }
                }
                let func = self.resolve_virtual_method(func, &args, program);
                let result = self.run_function_with_args(func, program, &args)?;
                return self.finish_call(result, pc);
            }
            Instruction::MakeClosure(fn_index, capture_count) => {
                let count = *capture_count as usize;
                let start = self.stack.len().saturating_sub(count);
                let captures: Vec<Value> = self.stack.drain(start..).collect();
                self.stack.push(Value::Closure {
                    fn_index: *fn_index,
                    captures,
                });
            }
            Instruction::CallClosure => {
                let closure_val = self.stack.pop().unwrap_or(Value::Nil);
                if let Value::Closure { fn_index, captures } = closure_val {
                    let func = program.functions.get(fn_index as usize).ok_or_else(|| {
                        SyxDiagnostic::error(format!("Unknown closure function index {fn_index}"))
                    })?;
                    let arg_count = func.param_count as usize;
                    let start = self.stack.len().saturating_sub(arg_count);
                    let args: Vec<Value> = self.stack.drain(start..).collect();
                    let result = self.run_closure_with_captures(func, program, &captures, &args)?;
                    return self.finish_call(result, pc);
                } else {
                    self.stack.push(Value::Nil);
                }
            }
            Instruction::ActorRegister => {
                let obj = self.stack.pop().unwrap_or(Value::Nil);
                let id = NEXT_ACTOR_ID.fetch_add(1, Ordering::Relaxed);
                self.stack.push(obj.register_actor_id(id));
            }
            Instruction::ActorCall(idx) => {
                let func = program
                    .functions
                    .get(*idx as usize)
                    .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function index {idx}")))?;
                let arg_count = func.param_count as usize;
                let start = self.stack.len().saturating_sub(arg_count);
                let args: Vec<Value> = self.stack.drain(start..).collect();
                let actor_id = args.first().and_then(actor_id_from_value).unwrap_or(0);
                // Wave A W489 / ADR-833: enqueue args so GcCollect sees mailbox Values.
                actor_mailbox::mailbox_enqueue(actor_id, &args);
                if let Some(rt) = &self.runtime {
                    let actor_ref = args.first().cloned().unwrap_or(Value::Nil);
                    let send = rt.actor_send_method(&actor_ref, func.name.as_str(), args);
                    let _ = actor_mailbox::mailbox_dequeue(actor_id);
                    match send {
                        // Wave A W508 / ADR-852: runtime ActorCall success reuses
                        // finish_call dual-register (ObjectRef+MutRefCell → ManagedHeap
                        // + heap_slots). ≠ MethodCall / ≠ migrate / cleanup.
                        Ok(result) => return self.finish_call(result, pc),
                        // ADR-264: preserve C009 (and other actor codes) for try/catch.
                        Err(err) => return self.fail_host_or_raise(err, pc),
                    }
                } else {
                    let _depth = actor_mailbox::mailbox_enter(actor_id);
                    let lock = actor_lock_for(actor_id);
                    let _guard = lock.lock().unwrap_or_else(|e| e.into_inner());
                    let result = self.run_function_with_args(func, program, &args);
                    actor_mailbox::mailbox_leave(actor_id);
                    let _ = actor_mailbox::mailbox_dequeue(actor_id);
                    let result = result?;
                    return self.finish_call(result, pc);
                }
            }
            Instruction::AwaitPoll => {
                cooperative_yield();
            }
            Instruction::AwaitSuspend { resume_fn, state } => {
                let awaited = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(rt) = &self.runtime {
                    if let Some(result) = crate::ws::try_poll_ws_request(&awaited) {
                        self.stack.push(result?);
                    } else if let Some(result) = crate::http::try_poll_http_request(&awaited) {
                        self.stack.push(result?);
                    } else if let Some(result) = crate::hosts::try_poll_file_read(&awaited) {
                        self.stack.push(result?);
                    } else if let Some(result) = crate::hosts::try_poll_tcp_read(&awaited) {
                        self.stack.push(result?);
                    } else if let Some(result) = rt.try_await_task_value(&awaited) {
                        self.stack.push(result?);
                    } else if matches!(
                        &awaited,
                        Value::Object(map)
                            if map.contains_key("__file_async_pending")
                                || map.contains_key("__http_async_pending")
                                || map.contains_key("__tcp_async_pending")
                    ) {
                        // ADR-278/279: blocking_pool File/HTTP/Tcp adapters — help-while-wait.
                        // Do not suspend at pc+1 (would discard pending and, with the
                        // historical double-push of CallHost args, leave leftover args
                        // as the await result).
                        loop {
                            cooperative_yield();
                            if let Some(result) = crate::http::try_poll_http_request(&awaited) {
                                self.stack.push(result?);
                                break;
                            }
                            if let Some(result) = crate::hosts::try_poll_file_read(&awaited) {
                                self.stack.push(result?);
                                break;
                            }
                            if let Some(result) = crate::hosts::try_poll_tcp_read(&awaited) {
                                self.stack.push(result?);
                                break;
                            }
                        }
                    } else {
                        self.suspend_frame = Some(VmSuspendFrame {
                            pc: pc + 1,
                            locals: self.locals.clone(),
                            stack: self.stack.clone(),
                            resume_fn: *resume_fn,
                            state: *state,
                        });
                        self.async_state = *state;
                        self.suspended = true;
                        return Ok(pc + 1);
                    }
                } else {
                    if let Some(result) = crate::channel_hosts::try_poll_channel_recv(&awaited) {
                        self.stack.push(result?);
                    } else if let Some(result) = crate::http::try_poll_http_request(&awaited) {
                        self.stack.push(result?);
                    } else if let Some(result) = crate::ws::try_poll_ws_request(&awaited) {
                        self.stack.push(result?);
                    } else if let Value::Object(map) = &awaited {
                        if map.contains_key("__await_value") {
                            if let Some(host) = crate::task_hosts::await_task_host() {
                                let mut stack = vec![awaited.clone()];
                                host(&mut stack)?;
                                self.stack.push(stack.pop().unwrap_or(Value::Nil));
                            } else {
                                self.stack.push(awaited);
                            }
                        } else if map.contains_key("__channel_recv_pending") {
                            cooperative_yield();
                            if let Some(result) =
                                crate::channel_hosts::try_poll_channel_recv(&awaited)
                            {
                                self.stack.push(result?);
                            } else {
                                self.suspend_frame = Some(VmSuspendFrame {
                                    pc: pc + 1,
                                    locals: self.locals.clone(),
                                    stack: self.stack.clone(),
                                    resume_fn: *resume_fn,
                                    state: *state,
                                });
                                self.async_state = *state;
                                self.suspended = true;
                                return Ok(pc + 1);
                            }
                        } else if map.contains_key("__http_async_pending") {
                            cooperative_yield();
                            if let Some(result) = crate::http::try_poll_http_request(&awaited) {
                                self.stack.push(result?);
                            } else {
                                self.suspend_frame = Some(VmSuspendFrame {
                                    pc: pc + 1,
                                    locals: self.locals.clone(),
                                    stack: self.stack.clone(),
                                    resume_fn: *resume_fn,
                                    state: *state,
                                });
                                self.async_state = *state;
                                self.suspended = true;
                                return Ok(pc + 1);
                            }
                        } else if map.contains_key("__file_async_pending") {
                            // ADR-278: help-while-wait (same as with-runtime path).
                            loop {
                                cooperative_yield();
                                if let Some(result) = crate::hosts::try_poll_file_read(&awaited) {
                                    self.stack.push(result?);
                                    break;
                                }
                            }
                        } else if map.contains_key("__tcp_async_pending") {
                            // ADR-279: help-while-wait for TcpReadAsync.
                            loop {
                                cooperative_yield();
                                if let Some(result) = crate::hosts::try_poll_tcp_read(&awaited) {
                                    self.stack.push(result?);
                                    break;
                                }
                            }
                        } else if map.contains_key("__ws_async_pending") {
                            cooperative_yield();
                            if let Some(result) = crate::ws::try_poll_ws_request(&awaited) {
                                self.stack.push(result?);
                            } else {
                                self.suspend_frame = Some(VmSuspendFrame {
                                    pc: pc + 1,
                                    locals: self.locals.clone(),
                                    stack: self.stack.clone(),
                                    resume_fn: *resume_fn,
                                    state: *state,
                                });
                                self.async_state = *state;
                                self.suspended = true;
                                return Ok(pc + 1);
                            }
                        } else {
                            cooperative_yield();
                            if let Some(host) = crate::task_hosts::await_task_host() {
                                let mut stack = vec![awaited.clone()];
                                host(&mut stack)?;
                                self.stack.push(stack.pop().unwrap_or(Value::Nil));
                            } else {
                                self.stack.push(awaited);
                            }
                        }
                    } else {
                        cooperative_yield();
                        if let Some(host) = crate::task_hosts::await_task_host() {
                            let mut stack = vec![awaited.clone()];
                            host(&mut stack)?;
                            self.stack.push(stack.pop().unwrap_or(Value::Nil));
                        } else {
                            self.stack.push(awaited);
                        }
                    }
                }
            }
            Instruction::ActorSend(idx) => {
                let func = program
                    .functions
                    .get(*idx as usize)
                    .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function index {idx}")))?;
                let arg_count = func.param_count as usize;
                let start = self.stack.len().saturating_sub(arg_count);
                let args: Vec<Value> = self.stack.drain(start..).collect();
                let actor_id = args.first().and_then(actor_id_from_value).unwrap_or(0);
                // Wave A W489 / ADR-833: enqueue args so GcCollect sees mailbox Values.
                actor_mailbox::mailbox_enqueue(actor_id, &args);
                if let Some(rt) = &self.runtime {
                    let actor_ref = args.first().cloned().unwrap_or(Value::Nil);
                    let result = rt.actor_send_method(&actor_ref, func.name.as_str(), args);
                    let _ = actor_mailbox::mailbox_dequeue(actor_id);
                    self.stack.push(result?);
                } else {
                    let _depth = actor_mailbox::mailbox_enter(actor_id);
                    let lock = actor_lock_for(actor_id);
                    let _guard = lock.lock().unwrap_or_else(|e| e.into_inner());
                    let result = self.run_function_with_args(func, program, &args);
                    actor_mailbox::mailbox_leave(actor_id);
                    let _ = actor_mailbox::mailbox_dequeue(actor_id);
                    self.stack.push(result?);
                }
            }
            Instruction::TaskSpawn(idx) => {
                let func = program
                    .functions
                    .get(*idx as usize)
                    .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function index {idx}")))?;
                let arg_count = func.param_count as usize;
                let start = self.stack.len().saturating_sub(arg_count);
                let args: Vec<Value> = self.stack.drain(start..).collect();
                if let Some(rt) = &self.runtime {
                    let handle = rt.spawn_detached_fn(func.name.as_str(), args)?;
                    self.stack.push(handle);
                } else {
                    let result = self.run_function_with_args(func, program, &args)?;
                    self.stack.push(result);
                }
            }
            Instruction::TaskJoin => {
                let task_val = self.stack.pop().unwrap_or(Value::Nil);
                if let Some(rt) = &self.runtime {
                    let result = rt.join_task_value(&task_val)?;
                    self.stack.push(result);
                } else {
                    self.stack.push(task_val);
                }
            }
            Instruction::CallWitness(_protocol, method) => {
                let func = program.functions.get(*method as usize).ok_or_else(|| {
                    SyxDiagnostic::error(format!("Unknown witness method index {method}"))
                })?;
                let arg_count = func.param_count as usize;
                let start = self.stack.len().saturating_sub(arg_count);
                let args: Vec<Value> = self.stack.drain(start..).collect();
                let result = self.run_function_with_args(func, program, &args)?;
                return self.finish_call(result, pc);
            }
            Instruction::ArraySet => {
                let val = self.stack.pop().unwrap_or(Value::Nil);
                let idx = self.stack.pop().unwrap_or(Value::I32(0));
                let arr_outer = self.stack.pop().unwrap_or(Value::Array(vec![]));
                let arr = Value::resolve_heap_slot(&arr_outer);
                if let Value::Array(mut items) = arr {
                    let i = idx.as_i32();
                    if i < 0 || (i as usize) >= items.len() {
                        return Err(SyxDiagnostic::error(format!(
                            "Array index {i} out of bounds (count {})",
                            items.len()
                        ))
                        .with_code("R003"));
                    }
                    items[i as usize] = val;
                    let updated = Value::Array(items);
                    if Value::heap_v4_enabled() {
                        self.stack
                            .push(Value::write_back_heap_slot(arr_outer, updated));
                    } else {
                        self.stack.push(updated);
                    }
                } else {
                    self.stack.push(arr_outer);
                }
            }
            Instruction::ObjectSet => {
                // Wave A W515 / ADR-859: after ObjectSet MutRefCell write-back,
                // dual-register stack-top ObjectRef+MutRefCell into ManagedHeap
                // + heap_slots. Wave A W532 / ADR-876: also dual-track nested
                // write-path `val` before set. ≠ migrate / ≠ ArraySet / ≠ deep
                // field-graph GC walk / ≠ enable / ≠ B-09 [x].
                let key = self.stack.pop().unwrap_or(Value::Str(String::new()));
                let val = self.stack.pop().unwrap_or(Value::Nil);
                let obj = self.stack.pop().unwrap_or(Value::Nil);
                // Only real string/character keys use the field path. Numerics also
                // have a non-empty as_str() display, but must go through dict_insert
                // so Dictionary<Int, _> IndexAssign round-trips with ObjectGet.
                let key_mat = Value::materialize_receiver(key.clone());
                let string_key = match &key_mat {
                    Value::Str(s) if !s.is_empty() => Some(s.clone()),
                    Value::Character(c) if !c.is_empty() => Some(c.clone()),
                    _ => None,
                };
                if let Some(k) = string_key {
                    self.stack.push(Value::mut_ref_cell_set_field(obj, &k, val));
                } else {
                    // Hashable non-string Dictionary keys (Int / class instances).
                    let resolved = Value::materialize_receiver(obj.clone());
                    let updated = match resolved {
                        Value::Dict(mut map) => {
                            syx_runtime_core::dict_insert(&mut map, key, val);
                            Value::Dict(map)
                        }
                        Value::Object(mut map) => {
                            syx_runtime_core::dict_insert(&mut map, key, val);
                            Value::Object(map)
                        }
                        other => other,
                    };
                    self.stack
                        .push(Value::write_back_heap_slot(obj, updated));
                }
            }
            _ => {}
        }
        Ok(pc + 1)
    }

    fn exec_numeric_binop<F: Fn(NumericKind, &Value, &Value) -> Option<Value>>(
        &mut self,
        kind: u8,
        f: F,
    ) -> SyxResult<()> {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        if let Some(k) = syx_stdlib::NumericKind::from_byte(kind) {
            if let Some(r) = f(k, &a, &b) {
                self.stack.push(r);
            } else {
                return Err(SyxDiagnostic::error("Numeric overflow or division by zero"));
            }
        }
        Ok(())
    }

    fn exec_simd_binop(&mut self, _lanes: u8, _element: u8, op: SimdBinOp) -> SyxResult<()> {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        if let Some(r) = simd_binop(op, &a, &b) {
            self.stack.push(r);
        } else {
            return Err(SyxDiagnostic::error("SIMD operation failed"));
        }
        Ok(())
    }

    fn exec_numeric_cmp<F: Fn(std::cmp::Ordering) -> bool>(
        &mut self,
        kind: u8,
        pred: F,
    ) -> SyxResult<()> {
        let b = self.stack.pop().unwrap_or(Value::Nil);
        let a = self.stack.pop().unwrap_or(Value::Nil);
        if let Some(k) = syx_stdlib::NumericKind::from_byte(kind) {
            let ok = numeric_cmp(k, &a, &b).is_some_and(pred);
            self.stack.push(Value::Bool(ok));
        } else {
            self.stack.push(Value::Bool(false));
        }
        Ok(())
    }

    fn binop_f64<F: Fn(f64, f64) -> f64>(&mut self, f: F) {
        let b = self.stack.pop().unwrap_or(Value::F64(0.0));
        let a = self.stack.pop().unwrap_or(Value::F64(0.0));
        let af = match a {
            Value::F64(v) => v,
            Value::F32(v) => v as f64,
            _ => a.as_i64() as f64,
        };
        let bf = match b {
            Value::F64(v) => v,
            Value::F32(v) => v as f64,
            _ => b.as_i64() as f64,
        };
        self.stack.push(Value::F64(f(af, bf)));
    }

    fn binop_f64_bool<F: Fn(f64, f64) -> bool>(&mut self, f: F) {
        let b = self.stack.pop().unwrap_or(Value::F64(0.0));
        let a = self.stack.pop().unwrap_or(Value::F64(0.0));
        let af = match a {
            Value::F64(v) => v,
            _ => a.as_i64() as f64,
        };
        let bf = match b {
            Value::F64(v) => v,
            _ => b.as_i64() as f64,
        };
        self.stack.push(Value::Bool(f(af, bf)));
    }

    fn binop_i32<F: Fn(i32, i32) -> i32>(&mut self, f: F) {
        let b = self.stack.pop().unwrap_or(Value::I32(0));
        let a = self.stack.pop().unwrap_or(Value::I32(0));
        self.stack.push(Value::I32(f(a.as_i32(), b.as_i32())));
    }

    fn binop_i32_cmp<F: Fn(i32, i32) -> bool>(&mut self, f: F) {
        let b = self.stack.pop().unwrap_or(Value::I32(0));
        let a = self.stack.pop().unwrap_or(Value::I32(0));
        self.stack.push(Value::Bool(f(a.as_i32(), b.as_i32())));
    }

    fn binop_bool<F: Fn(bool, bool) -> bool>(&mut self, f: F) {
        let b = self.stack.pop().unwrap_or(Value::Bool(false));
        let a = self.stack.pop().unwrap_or(Value::Bool(false));
        self.stack.push(Value::Bool(f(a.as_bool(), b.as_bool())));
    }

    fn call_method(&mut self, id: u32, program: &CompiledProgram) -> SyxResult<()> {
        let method = method_from_index(id).unwrap_or(MethodId::StrTrim);
        let arg_count = match method {
            MethodId::StrReplacingOccurrences | MethodId::StrSubstring => 2,
            MethodId::StrSplit | MethodId::StrHasPrefix | MethodId::StrMatches => 1,
            MethodId::ArrContains | MethodId::ArrJoined => 1,
            MethodId::ArrMap | MethodId::ArrFilter | MethodId::ArrReduce => 1,
            MethodId::HttpResponseHeader => 1,
            MethodId::SqlRowGetString
            | MethodId::SqlRowGetInt
            | MethodId::SqlRowGetFloat
            | MethodId::SqlRowGetBlob
            | MethodId::SqlRowIsNull => 1,
            MethodId::DictContainsKey
            | MethodId::DictRemove
            | MethodId::SetInsert
            | MethodId::SetRemove
            | MethodId::SetContains => 1,
            MethodId::HttpResponseJson
            | MethodId::HttpResponseStatus
            | MethodId::HttpResponseText
            | MethodId::HttpResponseBody
            | MethodId::HttpResponseOk
            | MethodId::HttpResponseHeaders
            | MethodId::SqlRowColumnNames => 0,
            _ => 0,
        };
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(self.stack.pop().unwrap_or(Value::Nil));
        }
        args.reverse();
        let receiver = self.stack.pop().unwrap_or(Value::Nil);
        if method == MethodId::ArrMap {
            if let (Value::Array(items), Some(Value::Closure { fn_index, captures })) =
                (&receiver, args.first())
            {
                let func = program.functions.get(*fn_index as usize).ok_or_else(|| {
                    SyxDiagnostic::error(format!("Unknown closure function index {fn_index}"))
                })?;
                let mut mapped = Vec::with_capacity(items.len());
                for item in items {
                    mapped.push(self.run_closure_with_captures(
                        func,
                        program,
                        captures,
                        &[item.clone()],
                    )?);
                }
                self.stack.push(Value::Array(mapped));
                return Ok(());
            }
            // ADR-225 / M2-S1-04: no fallthrough to empty-array success.
            return Err(SyxDiagnostic::error(
                "unsupported array HOF 'map': requires Array receiver and Closure (feature-off or illegal shape)",
            )
            .with_code("U001"));
        }
        if method == MethodId::ArrFilter {
            if let (Value::Array(items), Some(Value::Closure { fn_index, captures })) =
                (&receiver, args.first())
            {
                let func = program.functions.get(*fn_index as usize).ok_or_else(|| {
                    SyxDiagnostic::error(format!("Unknown closure function index {fn_index}"))
                })?;
                let mut filtered = Vec::new();
                for item in items {
                    let keep =
                        self.run_closure_with_captures(func, program, captures, &[item.clone()])?;
                    if keep.as_bool() {
                        filtered.push(item.clone());
                    }
                }
                self.stack.push(Value::Array(filtered));
                return Ok(());
            }
            return Err(SyxDiagnostic::error(
                "unsupported array HOF 'filter': requires Array receiver and Closure (feature-off or illegal shape)",
            )
            .with_code("U001"));
        }
        if method == MethodId::ArrReduce {
            if let (Value::Array(items), Some(Value::Closure { fn_index, captures })) =
                (&receiver, args.first())
            {
                let func = program.functions.get(*fn_index as usize).ok_or_else(|| {
                    SyxDiagnostic::error(format!("Unknown closure function index {fn_index}"))
                })?;
                if items.is_empty() {
                    self.stack.push(Value::Nil);
                    return Ok(());
                }
                let mut acc = items[0].clone();
                for item in items.iter().skip(1) {
                    acc = self.run_closure_with_captures(
                        func,
                        program,
                        captures,
                        &[acc, item.clone()],
                    )?;
                }
                self.stack.push(acc);
                return Ok(());
            }
            return Err(SyxDiagnostic::error(
                "unsupported array HOF 'reduce': requires Array receiver and Closure (feature-off or illegal shape)",
            )
            .with_code("U001"));
        }
        if method == MethodId::ArrSorted {
            let resolved = match &receiver {
                Value::HeapSlot { .. } => Value::resolve_heap_slot(&receiver),
                other => other.clone(),
            };
            if let Value::Array(items) = resolved {
                let sorted = self.sort_array_values(items, program)?;
                self.stack.push(sorted);
                return Ok(());
            }
        }
        if matches!(
            method,
            MethodId::HttpResponseJson
                | MethodId::HttpResponseStatus
                | MethodId::HttpResponseText
                | MethodId::HttpResponseBody
                | MethodId::HttpResponseOk
                | MethodId::HttpResponseHeader
                | MethodId::HttpResponseHeaders
        ) {
            self.stack
                .push(http::call_http_response_method(method, &receiver, &args));
            return Ok(());
        }
        if matches!(
            method,
            MethodId::SqlRowGetString
                | MethodId::SqlRowGetInt
                | MethodId::SqlRowGetFloat
                | MethodId::SqlRowGetBlob
                | MethodId::SqlRowIsNull
                | MethodId::SqlRowColumnNames
        ) {
            // ADR-246: HostFn stack order — receiver then args (name on top for get*).
            self.stack.push(receiver);
            for a in args {
                self.stack.push(a);
            }
            return match method {
                MethodId::SqlRowGetString => {
                    crate::stdlib_phase89_hosts::host_sql_row_get_string(&mut self.stack)
                }
                MethodId::SqlRowGetInt => {
                    crate::stdlib_phase89_hosts::host_sql_row_get_int(&mut self.stack)
                }
                MethodId::SqlRowGetFloat => {
                    crate::stdlib_phase89_hosts::host_sql_row_get_float(&mut self.stack)
                }
                MethodId::SqlRowGetBlob => {
                    crate::stdlib_phase89_hosts::host_sql_row_get_blob(&mut self.stack)
                }
                MethodId::SqlRowIsNull => {
                    crate::stdlib_phase89_hosts::host_sql_row_is_null(&mut self.stack)
                }
                MethodId::SqlRowColumnNames => {
                    crate::stdlib_phase89_hosts::host_sql_row_column_names(&mut self.stack)
                }
                _ => Ok(()),
            };
        }
        self.stack
            .push(hosts::call_receiver_method(method, &receiver, &args));
        Ok(())
    }

    fn virtual_fn_from_vtable_slot<'a>(
        receiver: &Value,
        method: &str,
        program: &'a CompiledProgram,
    ) -> Option<&'a FunctionUnit> {
        let key = Value::Str(format!("__vtable_{method}"));
        let idx = match receiver.get_by_key(&key) {
            Value::I32(i) => i as u32,
            _ => return None,
        };
        program.functions.get(idx as usize)
    }

    /// Unwrap `MutRefCell` / heap slots so virtual dispatch sees runtime `__type`.
    fn virtual_dispatch_receiver(receiver: &Value) -> Value {
        Value::dispatch_receiver_for_method(receiver.clone())
    }

    fn resolve_virtual_method<'a>(
        &self,
        func: &'a FunctionUnit,
        args: &[Value],
        program: &'a CompiledProgram,
    ) -> &'a FunctionUnit {
        let Some((static_class, method)) = syx_ast::parse_mangled_instance_method(&func.name)
        else {
            return func;
        };
        let static_class = static_class.as_str();
        let method = method.as_str();
        if method.is_empty() || static_class.is_empty() {
            return func;
        }
        // Constructors are never virtual: `App(logger: FastLogger())` lowers to
        // Call(App_init) with the dependency as the first stack arg; treating that
        // arg as `self` would wrongly redirect to FastLogger_init.
        if method == "init" || method == "deinit" {
            return func;
        }
        let Some(receiver) = args.first() else {
            return func;
        };
        let receiver = Self::virtual_dispatch_receiver(receiver);
        // Phase 126: protocol witness slots from class init (`__witness_Protocol_method`).
        let witness_key = Value::Str(format!("__witness_{static_class}_{method}"));
        if let Value::I32(idx) = receiver.get_by_key(&witness_key) {
            if let Some(slot_fn) = program.functions.get(idx as usize) {
                if slot_fn.name != func.name {
                    return slot_fn;
                }
            }
        }
        // Phase 125: instance vtable slot wins when present (MIR init emits __vtable_*).
        if let Some(slot_fn) = Self::virtual_fn_from_vtable_slot(&receiver, method, program) {
            if slot_fn.name != func.name {
                return slot_fn;
            }
        }
        // Prefer __type dispatch; vtable slots are advisory (stale indices fall back).
        let Some(runtime_ty) = receiver.object_ref_type_name() else {
            return func;
        };
        if runtime_ty == static_class {
            return func;
        }
        let dynamic = SmolStr::new(format!("{runtime_ty}_{method}"));
        program
            .functions
            .iter()
            .find(|f| f.name == dynamic)
            .unwrap_or(func)
    }

    fn comparable_witness_fn_index(value: &Value) -> Option<u32> {
        match value {
            Value::Object(map) | Value::Dict(map) => map
                .get("__witness_Comparable_compare")
                .and_then(|v| match v {
                    Value::I32(idx) => Some(*idx as u32),
                    _ => None,
                }),
            _ => None,
        }
    }

    fn compare_with_witness(
        &mut self,
        a: &Value,
        b: &Value,
        witness_idx: u32,
        program: &CompiledProgram,
    ) -> SyxResult<std::cmp::Ordering> {
        let func = program.functions.get(witness_idx as usize).ok_or_else(|| {
            SyxDiagnostic::error(format!("Unknown compare witness index {witness_idx}"))
        })?;
        let cmp = self.run_function_with_args(func, program, &[a.clone(), b.clone()])?;
        Ok(match cmp.as_i64().cmp(&0) {
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        })
    }

    fn sort_array_values(
        &mut self,
        mut items: Vec<Value>,
        program: &CompiledProgram,
    ) -> SyxResult<Value> {
        let witness_idx = items.first().and_then(Self::comparable_witness_fn_index);
        if let Some(idx) = witness_idx {
            let len = items.len();
            for i in 0..len {
                for j in (i + 1)..len {
                    let ord = self.compare_with_witness(&items[i], &items[j], idx, program)?;
                    if ord == std::cmp::Ordering::Greater {
                        items.swap(i, j);
                    }
                }
            }
        } else {
            items.sort_by(syx_runtime_core::value_compare);
        }
        // Match ArrayBuild: HeapSlot so in-place hosts (append/…) can write back.
        let array = Value::Array(items);
        if Value::heap_v4_enabled() {
            Ok(Value::alloc_heap_slot(
                array,
                syx_runtime_core::HeapSlotKind::Array,
            ))
        } else {
            Ok(array)
        }
    }

    fn sort_array_receiver(
        &mut self,
        receiver: Value,
        program: &CompiledProgram,
    ) -> SyxResult<Value> {
        let resolved = match &receiver {
            Value::HeapSlot { .. } => Value::resolve_heap_slot(&receiver),
            other => other.clone(),
        };
        match resolved {
            Value::Array(items) => self.sort_array_values(items, program),
            other => Ok(other),
        }
    }
}

#[cfg(all(test, feature = "interpreter"))]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use syx_codegen::FunctionUnit;

    #[test]
    fn prints_array_contents() {
        let mut vm = WasmVM::default();
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![
                Instruction::ConstI32(1),
                Instruction::ConstI32(2),
                Instruction::ConstI32(3),
                Instruction::ArrayBuild(3),
                Instruction::Print,
                Instruction::Return,
            ],
            instruction_spans: vec![],
            local_names: vec![],
            local_types: vec![],
            source_name: SmolStr::new(""),
            init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        vm.run_program(&program).unwrap();
        assert_eq!(vm.printed, vec!["[1, 2, 3]"]);
    }

    #[test]
    fn executes_print() {
        let mut vm = WasmVM::default();
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![
                Instruction::ConstStr("Hello, World".into()),
                Instruction::Print,
                Instruction::Return,
            ],
            instruction_spans: vec![],
            local_names: vec![],
            local_types: vec![],
            source_name: SmolStr::new(""),
            init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        vm.run_program(&program).unwrap();
        assert_eq!(vm.printed, vec!["Hello, World"]);
    }

    #[test]
    fn collect_live_roots_includes_suspend_frame_and_locals_frames() {
        // Wave A W488 / ADR-832: suspend_frame + locals_frames must enter GcCollect roots.
        let mut vm = WasmVM::default();
        vm.stack.push(Value::I64(1));
        vm.locals.push(Value::I64(2));
        vm.locals_frames
            .push(vec![Value::I64(42), Value::Str("frame".into())]);
        vm.suspend_frame = Some(VmSuspendFrame {
            pc: 0,
            locals: vec![Value::I64(99)],
            stack: vec![Value::Str("suspended".into())],
            resume_fn: 0,
            state: 0,
        });
        let roots = vm.collect_live_roots();
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(42))),
            "locals_frames value must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "frame")),
            "locals_frames str must be a root: {roots:?}"
        );
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(99))),
            "suspend_frame locals must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "suspended")),
            "suspend_frame stack must be a root: {roots:?}"
        );
    }

    #[test]
    fn collect_live_roots_includes_mailbox_value_queue() {
        // Wave A W489 / ADR-833: mailbox queue Values must enter GcCollect roots.
        let id = 88_901;
        while actor_mailbox::mailbox_dequeue(id).is_some() {}
        actor_mailbox::mailbox_enqueue(id, &[Value::I64(1234), Value::Str("queue".into())]);
        let vm = WasmVM::default();
        let roots = vm.collect_live_roots();
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(1234))),
            "mailbox queue I64 must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "queue")),
            "mailbox queue Str must be a root: {roots:?}"
        );
        let _ = actor_mailbox::mailbox_dequeue(id);
    }
}

/// Phase 33 FB1: VM/native generational GC remembered set prod (`SYX_VM_NATIVE_GC=0` opt-out).
pub fn vm_native_gc_enabled() -> bool {
    std::env::var("SYX_VM_NATIVE_GC")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(
            std::env::var("SYX_WASI_GC_PHASE33")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(
                    std::env::var("SYX_WASI_GC_PHASE33_PROD")
                        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
                        .map(|off| !off)
                        .unwrap_or(true),
                ),
        )
}

static VM_NATIVE_GC_REMEMBERED: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static VM_NATIVE_GC_COLLECTED: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn vm_native_gc_remembered_set_mark(slot: i32) -> i32 {
    if !vm_native_gc_enabled() {
        return slot.max(0);
    }
    VM_NATIVE_GC_REMEMBERED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    slot.max(0)
}

pub fn vm_native_gc_remembered_set_count() -> i32 {
    VM_NATIVE_GC_REMEMBERED.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn vm_native_gc_remembered_set_collect() -> i32 {
    if !vm_native_gc_enabled() {
        return 0;
    }
    let marked = vm_native_gc_remembered_set_count();
    VM_NATIVE_GC_COLLECTED.fetch_add(marked, std::sync::atomic::Ordering::SeqCst);
    VM_NATIVE_GC_REMEMBERED.store(0, std::sync::atomic::Ordering::SeqCst);
    marked
}

static VM_NATIVE_GC_CARD_TABLE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn gc_native_remembered_set_mark(slot: i32) -> i32 {
    vm_native_gc_remembered_set_mark(slot)
}

pub fn gc_native_remembered_set_count() -> i32 {
    vm_native_gc_remembered_set_count()
}

pub fn gc_native_remembered_set_prod_collect() -> i32 {
    vm_native_gc_remembered_set_collect()
}

pub fn gc_native_card_table_mark(card: i32) -> i32 {
    if !vm_native_gc_enabled() {
        return card.max(0);
    }
    VM_NATIVE_GC_CARD_TABLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    card.max(0)
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
    vm_native_gc_remembered_set_count()
}

pub fn unified_gc_remembered_set_collect() -> i32 {
    if !unified_gc_enabled() {
        return 0;
    }
    vm_native_gc_remembered_set_collect()
}

static UNIFIED_GC_PROMOTE_COUNT: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(0);

/// Phase 35 FL1: unified GC promotion prod default on.
pub fn unified_gc_promotion_prod_default() -> bool {
    std::env::var("SYX_UNIFIED_GC_PROMOTION_PROD")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|off| !off)
        .unwrap_or(unified_gc_prod_default())
}

pub fn unified_gc_promotion_enabled() -> bool {
    std::env::var("SYX_UNIFIED_GC_PROMOTION")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(unified_gc_promotion_prod_default() && unified_gc_enabled())
}

pub fn unified_gc_promote(slot: i32) -> i32 {
    if !unified_gc_promotion_enabled() {
        return slot.max(0);
    }
    UNIFIED_GC_PROMOTE_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    unified_gc_remembered_set_mark(slot)
}

pub fn unified_gc_promote_count() -> i32 {
    UNIFIED_GC_PROMOTE_COUNT.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn unified_gc_promotion_collect() -> i32 {
    if !unified_gc_promotion_enabled() {
        return 0;
    }
    unified_gc_remembered_set_collect() + unified_gc_promote_count()
}

pub fn gc_native_card_table_count() -> i32 {
    VM_NATIVE_GC_CARD_TABLE.load(std::sync::atomic::Ordering::SeqCst)
}

/// Reset the Phase 120/121/122 runtime service container (embed/tests).
pub fn reset_service_container() {
    let _ = stdlib_phase120_hosts::host_service_reset(&mut Vec::new());
}

/// Phase 122: explicit destroy with `@PreDestroy` hooks (embed/tests).
pub fn destroy_service_container() {
    let _ = stdlib_phase120_hosts::host_service_destroy(&mut Vec::new());
}
