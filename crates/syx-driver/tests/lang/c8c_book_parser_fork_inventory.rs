//! C-8c: book↔parser fork live inventory must exist with ≥20 classified rows.
//! ≠ C-8 [x] · ≠ fork converged · ≠ parser surface changes.
//! Post C-8d/C-8e: no live book_ahead / doc_debt / legacy_accept rows.

fn census_text() -> &'static str {
    // C-9g2: public fixture — not docs/census (maintainer ledger in plan/harness).
    include_str!("fixtures/c8c_book_parser_fork_inventory.md")
}

fn inventory_rows(census: &str) -> Vec<Vec<&str>> {
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in census.lines() {
        if line.starts_with("| id |") {
            in_table = true;
            continue;
        }
        if in_table {
            if !line.starts_with('|') {
                break;
            }
            if line.contains("---") {
                continue;
            }
            let cols: Vec<&str> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim())
                .collect();
            if cols.len() >= 7 && cols[0].starts_with('F') {
                rows.push(cols);
            }
        }
    }
    rows
}

const CLASSES: &[&str] = &[
    "parked_honest",
    "book_ahead",
    "legacy_accept",
    "doc_debt",
];

#[test]
fn c8c_inventory_exists_with_min_rows_and_classes() {
    let census = census_text();
    assert!(
        census.contains("C-8c") && census.contains("Live inventory"),
        "c-8c census must exist with Live inventory section"
    );
    let rows = inventory_rows(census);
    assert!(
        rows.len() >= 20,
        "inventory must have ≥20 rows; got {}",
        rows.len()
    );
    for row in &rows {
        assert!(
            CLASSES.contains(&row[4]),
            "row {} class must be closed enum; got {}",
            row[0],
            row[4]
        );
        assert!(!row[5].is_empty(), "row {} must have evidence", row[0]);
    }
    let present: std::collections::BTreeSet<&str> = rows.iter().map(|r| r[4]).collect();
    assert!(
        present.contains("parked_honest"),
        "inventory must keep parked_honest; present={present:?}"
    );
    for c in &present {
        assert!(
            CLASSES.contains(c),
            "unexpected class {c}; present={present:?}"
        );
    }
    assert!(
        !present.contains("book_ahead")
            && !present.contains("doc_debt")
            && !present.contains("legacy_accept"),
        "C-8d/C-8e cleared book_ahead/doc_debt/legacy_accept; present={present:?}"
    );
}

#[test]
fn c8c_inventory_seeds_callsuper_convenience_legacy_bare() {
    let rows = inventory_rows(census_text());
    let callsuper = rows
        .iter()
        .find(|r| r[1].contains("super.init") || r[2].contains("CallSuper") || r[0] == "F01")
        .expect("CallSuper / omit super.init row must exist");
    assert_eq!(
        callsuper[4], "parked_honest",
        "CallSuper row must be parked_honest"
    );
    assert!(
        callsuper[2].contains("CallSuper") || callsuper[3].contains("CallSuper"),
        "CallSuper row must mention CallSuper"
    );

    let convenience = rows
        .iter()
        .find(|r| r[1].contains("convenience") || r[2].contains("convenience"))
        .expect("convenience init row must exist");
    assert!(convenience[0].starts_with('F'), "convenience row id ok");

    // F18 bare-field: parked_honest after C-8e; evidence still cites bare / ADR.
    let bare = rows
        .iter()
        .find(|r| {
            r[0] == "F18"
                || ((r[1].contains("bare")
                    || r[2].contains("bare")
                    || r[3].contains("x: Int")
                    || r[5].contains("bare field")
                    || r[5].contains("language-book-ssot"))
                    && (r[1].contains("bare") || r[0] == "F18" || r[5].contains("Legacy bare")))
        })
        .expect("bare-field inventory row must exist");
    assert_eq!(
        bare[4], "parked_honest",
        "F18 bare-field must be parked_honest after C-8e"
    );
    assert!(
        bare[5].contains("language-book-ssot") || bare[5].contains("bare"),
        "F18 evidence must keep ADR / bare pointer"
    );

    let f19 = rows
        .iter()
        .find(|r| r[0] == "F19" || r[1].contains("unmarked mutating"))
        .expect("F19 unmarked mutating row must exist");
    assert_eq!(f19[4], "parked_honest", "F19 must be parked_honest after C-8e");
}

#[test]
fn c8c_ch15_callsuper_honesty_still_holds() {
    // Inventory must not regress C-8a honesty hinge.
    let en = include_str!("../../../../books/language-guide/15-initialization.md");
    assert!(
        en.contains("≠ CallSuper") || en.contains("≠ `CallSuper`"),
        "en Ch15 must keep ≠ CallSuper"
    );
    assert!(
        en.contains("Not implemented") && en.contains("CallSuper"),
        "en Ch15 must keep CallSuper Not implemented"
    );
}
