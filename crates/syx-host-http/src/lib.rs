//! Native HTTP host domain. `syx_host_http_install` fills shakeable host slots.

use syx_runtime_core::{
    register_http_async_poll, register_shakeable_host_id, register_ws_async_poll,
};
use syx_stdlib::HostFnId;

/// Register every HTTP-domain `HostFnId` into the shakeable host table.
pub fn install() {
    register_shakeable_host_id(HostFnId::HttpGet, syx_http_impl::host_http_get);
    register_shakeable_host_id(HostFnId::HttpPost, syx_http_impl::host_http_post);
    register_shakeable_host_id(
        HostFnId::HttpResponseJson,
        syx_http_impl::host_http_response_json,
    );
    register_shakeable_host_id(HostFnId::HttpRequest, syx_http_impl::host_http_request);
    register_shakeable_host_id(
        HostFnId::HttpResponseStatus,
        syx_http_impl::host_http_response_status,
    );
    register_shakeable_host_id(
        HostFnId::HttpResponseText,
        syx_http_impl::host_http_response_text,
    );
    register_shakeable_host_id(
        HostFnId::HttpResponseOk,
        syx_http_impl::host_http_response_ok,
    );
    register_shakeable_host_id(
        HostFnId::HttpResponseHeader,
        syx_http_impl::host_http_response_header,
    );
    register_shakeable_host_id(
        HostFnId::HttpResponseHeaders,
        syx_http_impl::host_http_response_headers,
    );
    register_shakeable_host_id(HostFnId::HttpServe, syx_http_impl::host_http_serve);
    register_shakeable_host_id(
        HostFnId::HttpMakeResponse,
        syx_http_impl::host_http_make_response,
    );
    register_shakeable_host_id(
        HostFnId::HttpRequestAsync,
        syx_http_impl::host_http_request_async,
    );
    register_shakeable_host_id(HostFnId::WsConnect, syx_http_impl::host_ws_connect);
    register_shakeable_host_id(
        HostFnId::WsConnectAsync,
        syx_http_impl::host_ws_connect_async,
    );
    register_shakeable_host_id(HostFnId::WsSendAsync, syx_http_impl::host_ws_send_async);
    register_shakeable_host_id(HostFnId::WsSend, syx_http_impl::host_ws_send);
    register_shakeable_host_id(HostFnId::WsSendProd, syx_http_impl::host_ws_send_prod);
    register_shakeable_host_id(HostFnId::WsRecv, syx_http_impl::host_ws_recv);
    register_shakeable_host_id(HostFnId::WsClose, syx_http_impl::host_ws_close);
    register_shakeable_host_id(
        HostFnId::HttpShutdown,
        syx_http_impl::host_http_serve_shutdown,
    );
    register_shakeable_host_id(
        HostFnId::HttpBodyStreamOpen,
        syx_http_impl::host_http_body_stream_open,
    );
    register_shakeable_host_id(
        HostFnId::HttpBodyStreamOpenOpts,
        syx_http_impl::host_http_body_stream_open_opts,
    );
    register_shakeable_host_id(
        HostFnId::HttpBodyStreamReadChunk,
        syx_http_impl::host_http_body_stream_read_chunk,
    );
    register_shakeable_host_id(
        HostFnId::HttpBodyStreamClose,
        syx_http_impl::host_http_body_stream_close,
    );
    register_http_async_poll(syx_http_impl::try_poll_http_request);
    register_ws_async_poll(syx_http_impl::try_poll_ws_request);
}

/// C ABI entry the native init calls when the HTTP domain is linked.
#[no_mangle]
pub extern "C" fn syx_host_http_install() {
    install();
}
