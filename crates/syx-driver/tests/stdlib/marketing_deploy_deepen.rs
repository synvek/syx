//! Marketing same-source deepen
//! Locks helper+fixture nail; Hosted full site / marketing met remain unmet; bans B `[x]`.

#[test]
fn marketing_deploy_deepen_nail_behavior() {
    use syx_driver::capabilities::run_capabilities_report;
    use syx_driver::marketing_capabilities::{
        capabilities_for_marketing, render_marketing_capabilities_page,
        write_marketing_capabilities_fixture,
    };

    let summary = capabilities_for_marketing(None);
    let report = run_capabilities_report();
    assert_eq!(summary["schema"], "syx-capability-v0");
    assert_eq!(
        summary["count"].as_u64().unwrap() as usize,
        report.rows.len()
    );
    let row = capabilities_for_marketing(Some("datetime.calendar"));
    assert_eq!(row["id"], "datetime.calendar");
    assert_eq!(row["status"], "implemented");
    assert_eq!(row["native"], "real");
    let html = render_marketing_capabilities_page(Some("datetime.calendar"));
    assert!(
        html.contains("<html>")
            && html.contains("syx-capability-v0")
            && html.contains("datetime.calendar")
            && html.contains("marketing fixture"),
        "HTML page must include html/schema/id/fixture label: {html}"
    );

    let path = std::env::temp_dir().join("syx-marketing-deepen-w390-nail.html");
    write_marketing_capabilities_fixture(&path, Some("datetime.calendar")).expect("write fixture");
    let got = std::fs::read_to_string(&path).expect("read fixture");
    assert!(
        got.contains("<html>")
            && got.contains("datetime.calendar")
            && got.contains("syx-capability-v0")
            && got.contains("marketing fixture"),
        "fixture must contain same-source marketing page: {got}"
    );
    let _ = std::fs::remove_file(&path);

    let docs = std::path::Path::new("/tmp/docs/marketing/forbidden.html");
    assert!(
        write_marketing_capabilities_fixture(docs, None).is_err(),
        "path guard must reject docs/ targets"
    );
}

