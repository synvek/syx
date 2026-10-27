//! ADR-188 / M2 W44: EC-M2-4 Tcp HostFn availability first nail (matrix honesty).
// was: ec_m2_4_tcp_availability

#[test]
fn ec_m2_4_tcp_availability_host_fn_overrides() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["TcpConnect", "TcpRead", "TcpWrite", "TcpHandleRelease"] {
        assert!(
            toml.contains(&format!("name = \"{name}\"")),
            "host_fn_capabilities.toml must override {name}"
        );
    }
    for name in ["TcpConnect", "TcpHandleRelease"] {
        let block = toml
            .split(&format!("name = \"{name}\""))
            .nth(1)
            .unwrap_or("")
            .split("[[host]]")
            .next()
            .unwrap_or("");
        assert!(
            block.contains("vm = \"real\""),
            "{name} must declare vm=real: {block}"
        );
    }
    assert!(
        !toml.contains("name = \"TcpStreamTls\""),
        "TcpStreamTls must stay deferred this window"
    );
    assert!(
        !toml.contains("name = \"TcpSockbuf\""),
        "TcpSockbuf must stay deferred this window"
    );
    assert!(
        !toml.contains("name = \"TcpHandleRefcount\""),
        "TcpHandleRefcount must stay deferred this window"
    );
}

#[test]
fn ec_m2_4_tcp_availability_product_net_tcp() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.tcp\""),
        "capabilities.toml must declare net.tcp"
    );
    assert!(
        caps.contains("ADR-188") || caps.contains("availability"),
        "net.tcp notes must cite ADR-188"
    );
    assert!(
        caps.contains("ec_m2_4_tcp_availability"),
        "net.tcp tests must list ec_m2_4_tcp_availability"
    );
}

#[test]
fn ec_m2_4_tcp_availability_generated_md_not_unsupported() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    for name in ["TcpConnect", "TcpHandleRelease"] {
        let row = md
            .lines()
            .find(|l| l.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} row missing"));
        assert!(
            !row.contains("| unsupported | unsupported |"),
            "{name} must not be all-unsupported: {row}"
        );
        assert!(
            row.contains("| real |"),
            "{name} must list a real backend: {row}"
        );
    }
}

#[test]
fn ec_m2_4_tcp_availability_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_tcp_availability_slice"),
        "inventory must contain ec_m2_4_tcp_availability_slice"
    );
    assert!(
        inv.contains("ADR-188") || inv.contains("signed-ADR-188"),
        "inventory must note ADR-188"
    );
}
