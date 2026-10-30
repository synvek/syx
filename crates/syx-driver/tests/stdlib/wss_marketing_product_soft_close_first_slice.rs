//! WSS marketing product soft-close inventory.
//! Locks SSOT docs + points back to existing floor nails.
//! ≠ full WSS product GA; ≠ extension; ≠ close-code full set
//! ≠ Playwright; ≠ SLA; ≠ websocat drop; ≠ todo 8 overall closeout
//! ≠ net.tls.server HostFn GA; no new runtime / ws_rfc6455 changes.


#[test]
fn wss_marketing_product_soft_close_floor_nails_indexed() {
    // Representative floor nails (already shipped); inventory only — no new runtime.
    let terminate = include_str!("http_serve_upgrade_terminate.rs");
    assert!(
        terminate.contains("fn http_serve_upgrade_terminate_accept_text_and_close"),
        "plain upgradeTerminate floor nail must remain"
    );

    let tls = include_str!("http_serve_tls.rs");
    assert!(
        tls.contains("fn http_serve_https_upgrade_terminate_accept_and_text_echo")
            && tls.contains("fn http_serve_https_hmr_path_same_port_roundtrip"),
        "HTTPS terminate + HMR-over-TLS floor nails must remain"
    );

    let proxy = include_str!("http_serve_upgrade_proxy.rs");
    assert!(
        proxy.contains("fn http_serve_upgrade_proxy_hmr_path_same_port_roundtrip"),
        "plain HMR same-port upgradeUpstream floor nail must remain"
    );

    let sgate_nail = include_str!("sgate_tls_native_first_slice.rs");
    assert!(
        sgate_nail.contains("sgate_tls_native_first_slice")
            && sgate_nail.contains("W1249"),
        "sgate TLS native first-slice nail must remain as HTTPS carrier floor"
    );

    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("http_serve_upgrade_proxy_hmr_path_same_port_roundtrip")
            && adr.contains("wss_marketing_product_soft_close_first_slice"),
        "ADR-1596 must index floor nails + this inventory nail"
    );
}
