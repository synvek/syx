//! Suite-3: HTTP loopback + connect-refused (M0-D04 / ADR-134).

use super::diff_exact;

#[test]
fn suite3_http_get_ok() {
    diff_exact("suite3/http_get_ok.sy");
}

#[test]
fn suite3_http_connect_refused() {
    diff_exact("suite3/http_connect_refused.sy");
}
