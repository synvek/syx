//! WSS marketing product soft-close inventory.
//! Locks SSOT docs + points back to existing floor nails.
//! ≠ full WSS product GA; ≠ extension; ≠ close-code full set
//! ≠ Playwright; ≠ SLA; ≠ websocat drop; ≠ todo 8 overall closeout
//! ≠ net.tls.server HostFn GA; no new runtime / ws_rfc6455 changes.

#[test]
fn wss_marketing_product_soft_close_docs_ssot() {
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("Decision (Wave W1250")
            && adr.contains("ADR-1596")
            && adr.contains("WSS 营销产品 soft-close")
            && adr.contains("产品地板 SSOT")
            && adr.contains("OUT（永久不作为本子轴开片理由）"),
        "ADR-1596 must define W1250 WSS marketing soft-close SSOT + OUT list"
    );
    assert!(
        adr.contains("RFC6455 extension")
            && adr.contains("close 码全集")
            && adr.contains("Playwright")
            && adr.contains("websocat"),
        "ADR-1596 OUT list must name extension / close-code full / Playwright / websocat"
    );

    let ug = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        ug.contains("W1250 / ADR-1596")
            && (ug.contains("WSS 营销产品 soft-close") || ug.contains("WSS 营销 soft-close"))
            && ug.contains("宣称")
            && ug.contains("soft-close met"),
        "UsageGuide must claim W1250 WSS marketing soft-close met"
    );

    let sgate = include_str!("../../../../tools/sgate/README.md");
    assert!(
        sgate.contains("W1250")
            && sgate.contains("ADR-1596")
            && (sgate.contains("marketing product soft-close")
                || sgate.contains("marketing soft-close")
                || sgate.contains("WSS 营销")),
        "sgate README must document W1250 marketing soft-close"
    );

    let todo = include_str!("../../../../plan/todo.md");
    assert!(
        todo.contains("W1250 / ADR-1596")
            && todo.contains("WSS 营销产品 soft-close met"),
        "plan/todo.md item 8 must record W1250 soft-close met"
    );
}

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
