//! Website same-source deepen
//! Locks deepen helper+nail; product met exit is (narrow); bans hosted full site / B `[x]`.

#[test]
fn website_capability_deepen_nail_behavior() {
    use syx_driver::capabilities::run_capabilities_report;
    use syx_driver::website_capabilities::{
        capabilities_for_website, render_capabilities_html_snippet,
    };

    let summary = capabilities_for_website(None);
    let report = run_capabilities_report();
    assert_eq!(summary["schema"], "syx-capability-v0");
    assert_eq!(
        summary["count"].as_u64().unwrap() as usize,
        report.rows.len()
    );
    let row = capabilities_for_website(Some("datetime.calendar"));
    assert_eq!(row["id"], "datetime.calendar");
    assert_eq!(row["status"], "implemented");
    assert_eq!(row["native"], "real");
    let html = render_capabilities_html_snippet(Some("datetime.calendar"));
    assert!(
        html.contains("syx-capability-v0") && html.contains("datetime.calendar"),
        "HTML must include schema and id: {html}"
    );
}
