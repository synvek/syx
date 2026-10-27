//! todo 8 overall soft-close inventory.
//! Locks item-8 done-face SSOT + OUT list; points back to floor nails.
//! ≠ Playwright/SLA/websocat/extension/close-full/WSS-GA "delivered"
//! ≠ HostFn+1; ≠ Wave E flip; no new runtime / ws_rfc6455 changes.

#[test]
fn todo8_overall_soft_close_docs_ssot() {
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("Decision (Wave W1251")
            && adr.contains("ADR-1597")
            && adr.contains("项总 soft-close")
            && adr.contains("项 8 完成面 SSOT")
            && adr.contains("OUT（项级永久，todo 8 不再开片）"),
        "ADR-1597 must define todo 8 overall soft-close SSOT + item-level OUT"
    );
    assert!(
        adr.contains("Playwright HMR GA")
            && adr.contains("全量并发 SLA")
            && adr.contains("websocat")
            && adr.contains("RFC6455 extension")
            && adr.contains("close 码全集"),
        "ADR-1597 OUT list must name Playwright / SLA / websocat / extension / close-code full"
    );

    let ug = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        ug.contains("W1251 / ADR-1597")
            && (ug.contains("todo 8 总 soft-close") || ug.contains("项 8 总 soft-close") || ug.contains("todo 8 soft-close"))
            && ug.contains("soft-close met"),
        "UsageGuide must claim W1251 todo 8 overall soft-close met"
    );

    let sgate = include_str!("../../../../tools/sgate/README.md");
    assert!(
        sgate.contains("W1251")
            && sgate.contains("ADR-1597")
            && (sgate.contains("overall soft-close")
                || sgate.contains("todo 8 soft-close")
                || sgate.contains("项 8")),
        "sgate README must document W1251 todo 8 overall soft-close"
    );

    let todo = include_str!("../../../../plan/todo.md");
    assert!(
        todo.contains("W1251 / ADR-1597")
            && (todo.contains("todo 8 soft-close")
                || todo.contains("项 8 soft-close")
                || todo.contains("项收口 met")
                || todo.contains("总 soft-close met"))
            && (todo.contains("项 8 不再开片") || todo.contains("不再开片")),
        "plan/todo.md item 8 must record W1251 overall soft-close / no further slices"
    );
}

#[test]
fn todo8_overall_soft_close_floor_nails_indexed() {
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
        "plain HMR same-port floor nail must remain"
    );

    let overlap = include_str!("http_serve_overlap.rs");
    assert!(
        overlap.contains("fn http_serve_overlap_slow_fast_plain_get")
            && overlap.contains("fn http_serve_overlap_slow_plain_and_hmr_upgrade"),
        "overlap + overlap×HMR floor nails must remain"
    );

    let sgate_nail = include_str!("sgate_tls_native_first_slice.rs");
    assert!(
        sgate_nail.contains("W1249") && sgate_nail.contains("sgate_tls_native_first_slice"),
        "sgate TLS native nail must remain"
    );

    let marketing = include_str!("wss_marketing_product_soft_close_first_slice.rs");
    assert!(
        marketing.contains("W1250") && marketing.contains("ADR-1596"),
        "W1250 marketing soft-close nail must remain"
    );

    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("todo8_overall_soft_close_first_slice")
            && adr.contains("http_serve_overlap_slow_plain_and_hmr_upgrade"),
        "ADR-1597 must index this inventory nail + overlap×HMR floor"
    );
}
