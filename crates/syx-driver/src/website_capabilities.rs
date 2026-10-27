//! Website same-source capability consumer (W377 / ADR-721).
//! Minimal docs-site / HTML surface over `run_capabilities_report()` — mirrors LSP deepen.
//! Inventory ≠ website met; HTML snippet is not a hosted site.

use serde_json::{json, Value};

use crate::capabilities_report::run_capabilities_report;

/// Same-source summary or row lookup for a website-facing consumer.
/// Always goes through `run_capabilities_report()` (no second JSON embed).
pub fn capabilities_for_website(id: Option<&str>) -> Value {
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

/// Minimal HTML snippet for a docs-site surface (not a deployable website).
pub fn render_capabilities_html_snippet(id: Option<&str>) -> String {
    let report = run_capabilities_report();
    let mut out = String::new();
    out.push_str("<div class=\"syx-capabilities\" data-schema=\"");
    out.push_str(&html_escape(&report.schema));
    out.push_str("\">");
    out.push_str("<p>schema=");
    out.push_str(&html_escape(&report.schema));
    out.push_str(" count=");
    out.push_str(&report.rows.len().to_string());
    out.push_str("</p>");
    if let Some(want) = id {
        match report.rows.iter().find(|r| r.id == want) {
            Some(row) => {
                out.push_str("<table><tr><th>id</th><th>status</th><th>native</th></tr>");
                out.push_str("<tr><td>");
                out.push_str(&html_escape(&row.id));
                out.push_str("</td><td>");
                out.push_str(&html_escape(&row.status));
                out.push_str("</td><td>");
                out.push_str(&html_escape(&row.native));
                out.push_str("</td></tr></table>");
            }
            None => {
                out.push_str("<p class=\"error\">unknown capability id: ");
                out.push_str(&html_escape(want));
                out.push_str("</p>");
            }
        }
    }
    out.push_str("</div>");
    out
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
    fn website_capabilities_same_source_count_and_lookup() {
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
            "HTML snippet must include schema and lookup id: {html}"
        );
    }
}
