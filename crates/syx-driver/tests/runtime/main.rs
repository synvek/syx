//! SYX 96.0 / Phase 118 — runtime integration gates.

mod async_native;
mod gc_churn;
mod gc_churn_quad;
mod gc_churn_stress;
mod gc_generational;
mod gc_relocating;
mod hof_closure;
mod hof_generic_witness;
mod kvo_chain;
mod net_grpc;
mod net_grpc_stream;
mod net_http2;
mod net_tls_wss;
mod reflect_witness_mandatory;
mod runtime_cast;
mod select_async;
mod select_mixed_send_recv;
mod select_deadline_strict;
mod stdlib_parity_hard;
