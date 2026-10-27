//! ADR-143 / M1 W7: document version gate + `$/cancelRequest` thin slice.

use super::*;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn m1_w7_stale_did_change_dropped() {
    let mut docs: HashMap<String, DocumentState> = HashMap::new();
    let open = json!({
        "textDocument": {
            "uri": "file:///t.sy",
            "languageId": "syx",
            "version": 1,
            "text": "func main() { print(1) }\n"
        }
    });
    let applied = apply_document_notification(&mut docs, "textDocument/didOpen", &open);
    assert!(applied.is_some());
    assert_eq!(docs["file:///t.sy"].version, 1);
    assert_eq!(docs["file:///t.sy"].text, "func main() { print(1) }\n");

    let stale = json!({
        "textDocument": { "uri": "file:///t.sy", "version": 1 },
        "contentChanges": [{ "text": "STALE\n" }]
    });
    assert!(
        apply_document_notification(&mut docs, "textDocument/didChange", &stale).is_none(),
        "same version must be dropped"
    );
    assert_eq!(docs["file:///t.sy"].text, "func main() { print(1) }\n");
    assert_eq!(docs["file:///t.sy"].version, 1);

    let older = json!({
        "textDocument": { "uri": "file:///t.sy", "version": 0 },
        "contentChanges": [{ "text": "OLDER\n" }]
    });
    assert!(apply_document_notification(&mut docs, "textDocument/didChange", &older).is_none());
    assert_eq!(docs["file:///t.sy"].text, "func main() { print(1) }\n");
}

#[test]
fn m1_w7_newer_did_change_applied() {
    let mut docs: HashMap<String, DocumentState> = HashMap::new();
    let open = json!({
        "textDocument": {
            "uri": "file:///t.sy",
            "version": 1,
            "text": "old\n"
        }
    });
    apply_document_notification(&mut docs, "textDocument/didOpen", &open).unwrap();

    let newer = json!({
        "textDocument": { "uri": "file:///t.sy", "version": 2 },
        "contentChanges": [{ "text": "new\n" }]
    });
    let applied = apply_document_notification(&mut docs, "textDocument/didChange", &newer);
    assert_eq!(applied, Some(("file:///t.sy".into(), "new\n".into())));
    assert_eq!(docs["file:///t.sy"].version, 2);
    assert_eq!(docs["file:///t.sy"].text, "new\n");
}

#[test]
fn m1_w7_should_accept_version_gate() {
    assert!(should_accept_version(None, 0));
    assert!(should_accept_version(Some(1), 2));
    assert!(!should_accept_version(Some(1), 1));
    assert!(!should_accept_version(Some(2), 1));
}

#[test]
fn m1_w7_cancel_request_suppresses_response() {
    let mut cancel = CancelRegistry::new();
    let id = json!(42);
    assert!(cancel.should_respond(&id), "uncancelled must respond");

    cancel.register_cancel_notification(Some(&json!({ "id": 42 })));
    assert!(cancel.is_cancelled(&id));
    assert!(
        !cancel.should_respond(&id),
        "cancelled id must suppress success result"
    );
    assert!(
        !cancel.is_cancelled(&id),
        "finish clears mark to avoid set growth"
    );
    // Subsequent request with same id may respond again.
    assert!(cancel.should_respond(&id));
}
