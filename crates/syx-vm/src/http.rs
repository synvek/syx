//! VM-facing HTTP surface. Heavy client/server code lives in `syx-http-impl`.

pub use syx_http_impl::*;

use std::sync::Once;

/// Install interpreter/native hooks once so HTTP serve can run bytecode handlers
/// and WebSocket hosts can reach the RFC6455 client that stays in this crate.
pub fn ensure_http_vm_bridge() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        syx_http_impl::install_vm_bridge(syx_http_impl::HttpVmBridge {
            capture_ctx,
            clear_ctx,
            spawn_blocking,
            run_bytecode,
        });
        syx_http_impl::install_ws_rfc(syx_http_impl::WsRfcHooks {
            connect_ws: crate::ws_rfc6455::rfc6455_connect,
            connect_wss: crate::ws_rfc6455::rfc6455_connect_wss,
            send_any: crate::ws_rfc6455::rfc6455_send_any,
            recv: crate::ws_rfc6455::rfc6455_recv,
            close: crate::ws_rfc6455::rfc6455_close,
        });
    });
}

fn spawn_blocking(job: Box<dyn FnOnce() + Send + 'static>) {
    crate::blocking_execute::execute_on_blocking_pool(job);
}

#[cfg(feature = "interpreter")]
struct VmHttpCtx(crate::ProgramCtx);

#[cfg(feature = "interpreter")]
impl syx_http_impl::HttpCtxClone for VmHttpCtx {
    fn box_clone(&self) -> Box<dyn syx_http_impl::HttpCtxClone + Send> {
        Box::new(VmHttpCtx(self.0.clone()))
    }

    fn install(&self) {
        crate::set_program_ctx(self.0.clone());
    }
}

#[cfg(feature = "interpreter")]
fn capture_ctx() -> Option<syx_http_impl::HttpCtxToken> {
    crate::with_program_ctx(|ctx| syx_http_impl::HttpCtxToken::new(VmHttpCtx(ctx.clone())))
}

#[cfg(not(feature = "interpreter"))]
fn capture_ctx() -> Option<syx_http_impl::HttpCtxToken> {
    None
}

#[cfg(feature = "interpreter")]
fn clear_ctx() {
    crate::clear_program_ctx();
}

#[cfg(not(feature = "interpreter"))]
fn clear_ctx() {}

#[cfg(feature = "interpreter")]
fn run_bytecode(
    name: &str,
    req: syx_runtime_core::Value,
) -> Option<syx_diagnostics::SyxResult<syx_runtime_core::Value>> {
    use syx_diagnostics::Span;

    let ctx = crate::with_program_ctx(|ctx| ctx.clone())?;
    Some((|| {
        let handler_fn = ctx
            .program
            .functions
            .iter()
            .find(|f| f.name == name)
            .ok_or_else(|| {
                crate::vm_error_at(
                    format!("Unknown handler function '{name}'"),
                    None,
                    Span::DUMMY,
                    "R003",
                )
            })?;
        let mut vm = crate::VmInstance::new();
        vm.runtime = ctx.runtime.clone();
        vm.run_function_with_args(handler_fn, &ctx.program, &[req])
    })())
}

#[cfg(not(feature = "interpreter"))]
fn run_bytecode(
    _name: &str,
    _req: syx_runtime_core::Value,
) -> Option<syx_diagnostics::SyxResult<syx_runtime_core::Value>> {
    None
}
