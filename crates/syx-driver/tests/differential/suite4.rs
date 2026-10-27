//! Suite-4: in-memory SQLite differential (M0-D05 / ADR-134).
//!
//! Uses `__syx_sql_exec` / `__syx_sql_query` only (no `__syx_sql_prepare` / ORM).
//! Resets rusqlite store before each backend run.

use super::diff_exact;

#[test]
fn suite4_sql_count_select() {
    diff_exact("suite4/sql_count_select.sy");
}
