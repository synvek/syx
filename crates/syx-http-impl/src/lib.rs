//! reqwest/hyper HTTP and WebSocket host implementations.
//!
//! Linked when the HTTP shake domain is referenced (`syx-host-http`) or when
//! `syx-vm` is built with the `http` feature.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

mod bridge;
mod error;
mod http_impl;
mod ws;
mod ws_terminate;

pub use bridge::{
    install_vm_bridge, install_ws_rfc, register_native_http_handler, HttpCtxClone, HttpCtxToken,
    HttpVmBridge, NativeHttpHandlerFn, WsRfcHooks,
};
pub use http_impl::{
    call_http_response_method, execute_http_request, host_http_body_stream_close,
    host_http_body_stream_open, host_http_body_stream_open_opts, host_http_body_stream_read_chunk,
    host_http_get, host_http_make_response, host_http_post, host_http_request,
    host_http_request_async, host_http_response_header, host_http_response_headers,
    host_http_response_json, host_http_response_ok, host_http_response_status,
    host_http_response_text, host_http_serve, host_http_serve_shutdown, host_url_build,
    host_url_encode_query, host_url_parse, http_body_stream_force_close,
    http_body_stream_live_count, http_make_response, http_request_object, http_response,
    is_http_response, make_http_request_awaitable, parse_response_json, try_poll_http_request,
};
pub use ws::{
    host_ws_close, host_ws_connect, host_ws_connect_async, host_ws_recv, host_ws_send,
    host_ws_send_async, host_ws_send_prod, try_poll_ws_request, ws_mock_enabled,
};
