//! ADR-186 / M2 W42: EC-M2-4 HTTP HostFn availability first nail (matrix honesty).
// was: ec_m2_4_http_availability

#[test]
fn ec_m2_4_http_availability_host_fn_overrides() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "HttpGet",
        "HttpPost",
        "HttpRequest",
        "HttpServe",
        "HttpMakeResponse",
        "HttpResponseStatus",
        "HttpResponseText",
        "HttpResponseOk",
        "HttpResponseHeader",
        "HttpResponseHeaders",
        "HttpResponseJson",
    ] {
        assert!(
            toml.contains(&format!("name = \"{name}\"")),
            "host_fn_capabilities.toml must override {name}"
        );
    }
    for name in ["HttpGet", "HttpServe"] {
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
        !toml.contains("name = \"HttpRequestAsync\""),
        "HttpRequestAsync must stay deferred this window"
    );
}

#[test]
fn ec_m2_4_http_availability_product_net_http() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"net.http\"") && caps.contains("id = \"net.http.server\""),
        "capabilities.toml must declare net.http + net.http.server"
    );
    assert!(
        caps.contains("ADR-186") || caps.contains("availability"),
        "HTTP capability notes must cite ADR-186"
    );
}

#[test]
fn ec_m2_4_http_availability_generated_md_not_unsupported() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    for name in ["HttpGet", "HttpServe"] {
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
fn ec_m2_4_http_availability_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_http_availability_slice"),
        "inventory must contain ec_m2_4_http_availability_slice"
    );
    assert!(
        inv.contains("ADR-186") || inv.contains("signed-ADR-186"),
        "inventory must note ADR-186"
    );
}
