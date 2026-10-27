//! Hosted same-source capability consumer (W380 / ADR-724).
//! Minimal full HTML page + fixture write over `run_capabilities_report()` — mirrors W377 website deepen.
//! Inventory ≠ hosted met; fixture/temp page is not marketing deploy.

use std::io;
use std::path::Path;

use serde_json::{json, Value};

use crate::capabilities_report::run_capabilities_report;

/// Same-source summary or row lookup for a hosted-facing consumer.
/// Always goes through `run_capabilities_report()` (no second JSON embed).
pub fn capabilities_for_hosted(id: Option<&str>) -> Value {
    let report = run_capabilities_report();
    match id {
        None => json!({
            "schema": report.schema,
            "count": report.rows.len(),
        }),
        Some(want) => match report.rows.iter().find(|r| r.id == want) {
            Some(row) => json!({
                "id": row.id,
                "status": row.status,
                "vm": row.vm,
                "mir": row.mir,
                "native": row.native,
                "wasm": row.wasm,
            }),
            None => json!({ "error": format!("unknown capability id: {want}") }),
        },
    }
}

/// Minimal complete HTML document for a static-hostable surface (not marketing deploy).
pub fn render_hosted_capabilities_page(id: Option<&str>) -> String {
    let report = run_capabilities_report();
    let mut body = String::new();
    body.push_str("<div class=\"syx-hosted-capabilities\" data-schema=\"");
    body.push_str(&html_escape(&report.schema));
    body.push_str("\">");
    body.push_str("<p>schema=");
    body.push_str(&html_escape(&report.schema));
    body.push_str(" count=");
    body.push_str(&report.rows.len().to_string());
    body.push_str("</p>");
    if let Some(want) = id {
        match report.rows.iter().find(|r| r.id == want) {
            Some(row) => {
                body.push_str("<table><tr><th>id</th><th>status</th><th>native</th></tr>");
                body.push_str("<tr><td>");
                body.push_str(&html_escape(&row.id));
                body.push_str("</td><td>");
                body.push_str(&html_escape(&row.status));
                body.push_str("</td><td>");
                body.push_str(&html_escape(&row.native));
                body.push_str("</td></tr></table>");
            }
            None => {
                body.push_str("<p class=\"error\">unknown capability id: ");
                body.push_str(&html_escape(want));
                body.push_str("</p>");
            }
        }
    }
    body.push_str("</div>");

    let mut out = String::new();
    out.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\">");
    out.push_str("<title>SYX capabilities (hosted fixture)</title></head><body>");
    out.push_str(&body);
    out.push_str("</body></html>");
    out
}

/// Write hosted capabilities HTML to `path` (temp_dir or tests/fixtures/hosted only).
/// Callers must not target docs/ or books/ marketing trees.
pub fn write_hosted_capabilities_fixture(path: &Path, id: Option<&str>) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let html = render_hosted_capabilities_page(id);
    std::fs::write(path, html)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hosted_capabilities_same_source_count_and_lookup() {
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
            "hosted page must include html + schema + lookup id: {html}"
        );
    }

    #[test]
    fn hosted_capabilities_fixture_write_temp() {
        let path = std::env::temp_dir().join("syx-hosted-capabilities-w380.html");
        write_hosted_capabilities_fixture(&path, Some("datetime.calendar")).unwrap();
        let got = std::fs::read_to_string(&path).unwrap();
        assert!(
            got.contains("<html>") && got.contains("datetime.calendar"),
            "fixture file must contain hosted page: {got}"
        );
        let _ = std::fs::remove_file(&path);
    }
}
