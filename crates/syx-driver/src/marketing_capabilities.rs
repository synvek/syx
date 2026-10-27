//! Marketing same-source capability consumer (W390 / ADR-734).
//! Minimal full HTML page + fixture write over `run_capabilities_report()` — mirrors W380 hosted deepen.
//! Inventory thin remains met; fixture/temp page is not marketing full-site deploy.

use std::io;
use std::path::Path;

use serde_json::{json, Value};

use crate::capabilities_report::run_capabilities_report;

/// Same-source summary or row lookup for a marketing-facing fixture consumer.
/// Always goes through `run_capabilities_report()` (no second JSON embed).
pub fn capabilities_for_marketing(id: Option<&str>) -> Value {
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

/// Minimal complete HTML document labeled as a marketing fixture (≠ production deploy).
pub fn render_marketing_capabilities_page(id: Option<&str>) -> String {
    let report = run_capabilities_report();
    let mut body = String::new();
    body.push_str("<div class=\"syx-marketing-capabilities\" data-schema=\"");
    body.push_str(&html_escape(&report.schema));
    body.push_str("\" data-fixture=\"marketing\">");
    body.push_str("<p>marketing fixture (not production deploy) schema=");
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
    out.push_str("<title>SYX capabilities (marketing fixture)</title></head><body>");
    out.push_str(&body);
    out.push_str("</body></html>");
    out
}

fn path_targets_docs_or_books(path: &Path) -> bool {
    path.components()
        .any(|c| matches!(c.as_os_str().to_str(), Some("docs") | Some("books")))
}

/// Write marketing capabilities HTML to `path` (temp_dir or tests/fixtures/marketing only).
/// Rejects paths that include `docs/` or `books/` components.
pub fn write_marketing_capabilities_fixture(path: &Path, id: Option<&str>) -> io::Result<()> {
    if path_targets_docs_or_books(path) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "marketing fixture must not target docs/ or books/ trees",
        ));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let html = render_marketing_capabilities_page(id);
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
    fn marketing_capabilities_same_source_count_and_lookup() {
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
            "marketing page must include html + schema + lookup id + fixture label: {html}"
        );
    }

    #[test]
    fn marketing_capabilities_fixture_write_temp() {
        let path = std::env::temp_dir().join("syx-marketing-capabilities-w390.html");
        write_marketing_capabilities_fixture(&path, Some("datetime.calendar")).unwrap();
        let got = std::fs::read_to_string(&path).unwrap();
        assert!(
            got.contains("<html>")
                && got.contains("datetime.calendar")
                && got.contains("marketing fixture"),
            "fixture file must contain marketing page: {got}"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn marketing_capabilities_rejects_docs_books_paths() {
        let docs = Path::new("/tmp/docs/marketing/index.html");
        let books = Path::new("/tmp/books/shared/marketing.html");
        assert!(write_marketing_capabilities_fixture(docs, None).is_err());
        assert!(write_marketing_capabilities_fixture(books, None).is_err());
    }
}
