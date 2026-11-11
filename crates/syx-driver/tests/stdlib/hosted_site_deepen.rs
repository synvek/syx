//! Hosted same-source deepen
//! Locks helper+fixture nail; Hosted full site remains unmet; bans marketing deploy / B `[x]`.

#[test]
fn hosted_site_deepen_nail_behavior() {
    use syx_driver::capabilities::run_capabilities_report;
    use syx_driver::hosted_capabilities::{
        capabilities_for_hosted, render_hosted_capabilities_page, write_hosted_capabilities_fixture,
    };

    let summary = capabilities_for_hosted(None);
    let report = run_capabilities_report();
    assert_eq!(summary["schema"], "syx-capability-v0");
    assert_eq!(
        summary["count"].as_u64().unwrap() as usize,
        report.rows.len()
    );
    let row = capabilities_for_hosted(Some("datetime.calendar"));
    assert_eq!(row["id"], "datetime.calendar");
    assert_eq!(row["status"], "implemented");
    assert_eq!(row["native"], "real");
    let html = render_hosted_capabilities_page(Some("datetime.calendar"));
    assert!(
        html.contains("<html>")
            && html.contains("syx-capability-v0")
            && html.contains("datetime.calendar"),
        "HTML page must include html/schema/id: {html}"
    );

    let path = std::env::temp_dir().join("syx-hosted-deepen-w380-nail.html");
    write_hosted_capabilities_fixture(&path, Some("datetime.calendar")).expect("write fixture");
    let got = std::fs::read_to_string(&path).expect("read fixture");
    assert!(
        got.contains("<html>")
            && got.contains("datetime.calendar")
            && got.contains("syx-capability-v0"),
        "fixture must contain same-source hosted page: {got}"
    );
    let _ = std::fs::remove_file(&path);
}
