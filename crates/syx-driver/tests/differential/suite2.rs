//! Suite-2: File text read/write/close (M0-D03).

use super::diff_exact;

#[test]
fn suite2_file_text_roundtrip() {
    diff_exact("suite2/file_text_roundtrip.sy");
}

#[test]
fn suite2_file_handle_close() {
    diff_exact("suite2/file_handle_close.sy");
}

#[test]
fn suite2_file_open_close_loop() {
    diff_exact("suite2/file_open_close_loop.sy");
}

#[test]
fn suite2_file_exists_roundtrip() {
    // Wave A W1 / ADR-345 / A-02 thin: exists → write → exists → delete → exists
    diff_exact("suite2/file_exists_roundtrip.sy");
}

#[test]
fn suite2_file_overwrite_roundtrip() {
    // Wave A W2 / ADR-346 / A-02: write → read → overwrite → read
    diff_exact("suite2/file_overwrite_roundtrip.sy");
}
